mod tests;
mod info;
mod header;
mod repo;

use crate::info::*;
use crate::header::*;
use crate::repo::*;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    Header::presentation();

    let user = Header::show_field();
    Info::response(user.clone()).await?;

    println!("--------------------------------- \n");
    println!("Repositories");

    Repo::response(user).await?;
    Ok(())
}