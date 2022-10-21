#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use crate::context::Context;
use crate::error::ServerError;

pub mod error;

mod context;
mod nspdata;

pub async fn run() -> Result<(), ServerError> {
    let context = Context::new_with_in_memory_cache();

    let _rocket = rocket::build()
        .manage(context)
        .mount("/nspdata", routes![nspdata::handle_search])
        .launch()
        .await?;

    Ok(())
}
