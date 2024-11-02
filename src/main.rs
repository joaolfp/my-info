mod tests;
mod info;
mod header;
mod repo;

use crate::info::*;
use crate::header::*;
use crate::repo::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    presentation();

    let user = header();
    response(user.clone()).await?;

    println!("--------------------------------- \n");
    println!("Repositories");

    response_repo(user).await?;
    Ok(())
}