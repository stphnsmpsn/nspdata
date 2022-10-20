pub async fn download(base_url: &str, args: &crate::args::Args) -> Result<(), crate::error::Error> {
    println!("downloading nsp xml");
    match (&args.uid, &args.output_filename, args.start, args.end) {
        (Some(user_id), Some(output_filename), Some(start), Some(end)) => {
            let user_auth_response = bidgely_adapter::auth::auth(base_url, user_id).await?;

            let session_response =
                bidgely_adapter::session::session(base_url, user_auth_response.payload.as_str())
                    .await?;

            bidgely_adapter::feed::download_and_save_feed_xml(
                base_url,
                user_id,
                session_response.payload.token_details.access_token.as_str(),
                start,
                end,
                output_filename,
            )
            .await?;
        }
        _ => Err(crate::error::Error::BadArgument(
            "Must Provide UID, Start, End, and Output Filename For Download".to_string(),
        ))?,
    };
    Ok(())
}
