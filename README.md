# NSP Data
___

Nova scotia power now provides data from smart meters as a downloadable XML file. This project has three goals:

1. To make it easier to download this data
2. To convert this data to more readable formats (i.e: JSON)
3. To provide insights into this data 

## Usage

### Downloading Data

In order to download data you must have your user ID. I am currently working on a way to retrieve this programmatically
but I have not yet figured this out. You can however easily grab it from your browser's dev tools by inspecting 
your local storage on: [https://nsp.bidgely.com/dashboard](https://nsp.bidgely.com/dashboard). You should see it in the
values for both the `analyticsSessionInfo` and `lastVisitedPremisesUuid` keys. 

Once you've got your UUID, you can download data using the following: 

`cargo run --bin nspdata -- --uid ${YOUR_UUID} --action download --start ${EPOCH_SECONDS_START} --end ${EPOCH_SECONDS_END} --output-filename ${FILENAME}`

### Converting Data

I currently only support converting to JSON so there are no arguments for destination format. Provided you have an input
file; obtained either by downloading as per the above or manually on the NSP website, you can convert to JSON as follows:

`cargo run --bin nspdata -- --action convert --input-filename ${INPUT_FILENAME} --output-filename ${OUTPUT_FILENAME}`

### Insights

I am currently working on some insights locally but haven't pushed anything yet. Values in the `IntervalReading` are in
KWh and I have verified that summing up the values for a day match the values shown on the NSP dashboard but. 


For example, if you wanted to see how much power you used each day in a period you could do something like:

***DISCLAIMER: CRUDE, UNOPTIMIZED CODE.*** 

```rust

let user_id = "your-user-id";
let start = 1664593200;
let end = 1665413999;

let user_auth_response = bidgely_adapter::auth::auth(BIDGELY_BASE_URL, user_id).await?;
let session_response = bidgely_adapter::session::session(BIDGELY_BASE_URL, user_auth_response.payload.as_str()).await?;
let feed = bidgely_adapter::feed::get_feed(BIDGELY_BASE_URL, user_id, session_response.payload.token_details.access_token.as_str(), start, end).await?;

let interval_blocks: Vec<bidgely_adapter::feed::IntervalBlock> = feed
    .entry
    .into_iter()
    .filter_map(|entry| 
        match entry.content.to_inner() {
            bidgely_adapter::feed::ContentType::IntervalBlock(e) => Some(e),
            _ => None,
        })
    .collect();

let mut days: Vec<u32> = vec![];

interval_blocks.iter().for_each(|interval_block| {
    let total = interval_block
        .interval_reading
        .iter()
        .fold(0, |acc, x| acc + x.value);
    days.push(total)
});

days.iter().for_each(|day| println!("{:?}", day));

```

## Things I've Noticed

Usage data is not immediately available. I have had mixed results retrieving data less than two days old. Data more than
two days old can reliably be retrieved. I'm not sure why this is but NSP, Bidgely, or UtilityAPI must be doing some 
batch processing before making data publicly available.