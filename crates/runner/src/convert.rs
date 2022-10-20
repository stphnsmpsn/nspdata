pub fn convert(args: &crate::args::Args) -> Result<(), crate::error::Error> {
    println!("converting nsp xml to json");
    match &args.input_filename {
        Some(file_name) => {
            let xml_string = std::fs::read_to_string(file_name)?;
            let feed: bidgely_adapter::feed::Feed = quick_xml::de::from_str(&xml_string)?;
            let output = serde_json::to_string(&feed)?;

            let output_filename = match &args.output_filename {
                None => std::path::Path::new(file_name)
                    .file_stem()
                    .and_then(std::ffi::OsStr::to_str)
                    .unwrap_or_else(|| "output"),
                Some(output_filename) => output_filename.as_str(),
            };

            std::fs::write(output_filename, output).expect("Unable to write file");
        }
        None => Err(crate::error::Error::BadArgument(
            "Must provide filename for convert".to_string(),
        ))?,
    };
    Ok(())
}
