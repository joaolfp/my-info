use reqwest;

mod tests;
mod network;
mod header;

use crate::network::*;
use crate::header::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    presentation();

    let user = header();
    response(user).await?;
    Ok(())
}