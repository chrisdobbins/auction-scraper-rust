use tokio;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize,Debug)]
struct LocationApiItem {
   id: i32,
   nickName: String,
   address: String,
   city: String,
   state: String,
   zip: String,
   hours: String,
   taxRate: f64,
   extendedHours: Option<String>,
   lat: String,
   lng: String, 
}

#[derive(Deserialize,Debug)]
struct LocationsApiResp {
    data: String,
}

#[derive(Deserialize,Debug)]
struct AuctionLocation {
    id: i32,
    nickName: String,
    address: String,
}

#[derive(Deserialize,Debug)]
struct Item {
    id: i32,
    lotCode: String,
    auctionId: i32,
    auctionNumber: String,
    quantity: i32,
    condition: String,
    brand: String,
    model: String,
    title: String,
    itemClosed: bool,
    hoursRemaining: i32,
    utcEndDateTime: String,
    removed: bool,
    auctionLocation: AuctionLocation,
    currentBid: f64,
    itemTimeRemaining: String,
}

#[derive(Deserialize,Debug)]
struct Condition {
    id: i32,
    name: String,
}

type Location = HashMap<i32, String>;
type Locations = HashMap<String, Location>;

#[derive(Deserialize,Debug)]
struct InitialFilter {
    itemSearchKeywords: String,
    itemCategories: serde_json::Value,
    locations: Locations,
    auctionEndDates: Vec<String>,
    conditions: Vec<Condition>,
}

#[derive(Deserialize,Debug)]
struct PageProp {
    initialData: InitialData,
    initialFilter: InitialFilter,
}

#[derive(Deserialize,Debug)]
struct Resp {
    pageProps: PageProp,
}

#[derive(Deserialize,Debug)]
struct InitialData {
    pageId: i32,
    pageCount: i32,
    totalCount: i32,
    items: Vec<Item>,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; rv:109.0) Gecko/20100101 Firefox/118.0".parse().unwrap());
    headers.insert("x-nextjs-data", "1".parse().unwrap());
    headers.insert("Accept","application/json".parse().unwrap());
    let res = client.get("https://www.bidfta.com/_next/data/Imsb_QePZ7Uw0EbJ6ol3c/en-US/items.json?pageId=1&itemSearchKeywords=le%20creuset").headers(headers).send().await?;
    
    let parsed = res.json::<Resp>().await?;
    println!("parsed response: {:#?}", parsed);
    
    
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Accept","application/json".parse().unwrap());
    let res = client.get("https://auction.bidfta.io/api//location/getAllLocations").headers(headers).send().await?;
    let parsed = &res.json::<Vec<LocationApiItem>>().await?;
    println!("{:?}", parsed);

    Ok(())
}

