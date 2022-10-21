use crate::{BidgelyError, BIDGELY_BASE_URL};
use std::fs;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub struct Feed {
    pub id: String,
    pub title: String,
    pub updated: String,
    #[serde(rename(deserialize = "entry"))]
    pub entries: Vec<Entry>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Entry {
    pub id: String,
    pub link: Vec<Link>,
    pub title: String,
    pub content: Content,
    pub published: String,
    pub updated: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Link {
    pub href: String,
    pub rel: String,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Content {
    #[serde(rename(deserialize = "$value"))]
    inner: ContentType,
}

impl From<ContentType> for Content {
    fn from(inner: ContentType) -> Self {
        Self { inner }
    }
}

impl Content {
    pub fn to_inner(self) -> ContentType {
        self.inner
    }
}

impl std::ops::Deref for Content {
    type Target = ContentType;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for Content {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum ContentType {
    LocalTimeParameters(LocalTimeParameters),
    UsagePoint(UsagePoint),
    ReadingType(ReadingType),
    MeterReading,
    IntervalBlock(IntervalBlock),
    #[serde(other)]
    Other,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct LocalTimeParameters {
    pub dst_end_rule: String,
    pub dst_offset: String,
    pub dst_start_rule: String,
    pub tz_offset: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct UsagePoint {
    pub service_category: ServiceCategory,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ServiceCategory {
    pub kind: u32,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ReadingType {
    pub accumulation_behaviour: u32,
    pub commodity: u32,
    pub data_qualifier: u32,
    pub default_quality: u32,
    pub flow_direction: u32,
    pub interval_length: u32,
    pub kind: u32,
    pub phase: u32,
    pub power_of_ten_multiplier: u32,
    pub time_attribute: u32,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct IntervalBlock {
    pub interval: Interval,
    #[serde(rename = "IntervalReading")]
    pub interval_reading: Vec<IntervalReading>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Interval {
    pub duration: u64,
    pub start: u64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct IntervalReading {
    #[serde(rename = "ReadingQuality")]
    pub reading_quality: ReadingQuality,
    #[serde(rename = "timePeriod")]
    pub time_period: TimePeriod,
    pub value: u32,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct ReadingQuality {
    pub quality: u32, // todo: pub enum ReadingQuality
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct TimePeriod {
    pub duration: u64,
    pub start: u64,
}

pub async fn download_and_save_feed_xml(
    user_id: &str,
    token: &str,
    start: u64,
    end: u64,
    output_filename: &str,
) -> Result<(), BidgelyError> {
    let client = reqwest::Client::new();
    let xml_data = client.get(format!(
        "{BIDGELY_BASE_URL}/dashboard/users/{user_id}/gb-download?start={start}&end={end}&measurement-type=ELECTRIC"
    ))
        .header(reqwest::header::CONTENT_TYPE, "application/json;charset=UTF-8")
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"))
        .send()
        .await
        ?
        .text()
        .await
        ?;

    fs::write(
        format!(
            "{}.xml",
            std::path::Path::new(output_filename)
                .file_stem()
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or_else(|| "output")
        ),
        xml_data,
    )?;

    Ok(())
}

pub async fn get_feed(
    user_id: &str,
    token: &str,
    start: u64,
    end: u64,
) -> Result<Feed, BidgelyError> {
    let client = reqwest::Client::new();
    let xml_data = client.get(format!(
        "{BIDGELY_BASE_URL}/dashboard/users/{user_id}/gb-download?start={start}&end={end}&measurement-type=ELECTRIC"
    ))
        .header(reqwest::header::CONTENT_TYPE, "application/json;charset=UTF-8")
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"))
        .send()
        .await
        ?
        .text()
        .await
        ?;

    Ok(quick_xml::de::from_str(&xml_data)?)
}
