use reqwest;
use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct Product {

}

pub async fn http_client() {
    let response = reqwest::get("https://dummyjson.com/products")
    .await?
    .json::<Product>();
    .await?;
}
