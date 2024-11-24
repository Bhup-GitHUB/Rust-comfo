use reqwest::Client;
use tokio::task;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let api_key = "Pivito";
    let base_url = "https://maps.googleapis.com/maps/api/geocode/json";
    let addresses = vec![
        "1600 Amphitheatre Parkway, Mountain View, CA",
        "1 Infinite Loop, Cupertino, CA",
    ];

    let results: Vec<_> = futures::future::join_all(
        addresses.iter().map(|address| {
            let client = &client;
            let api_key = api_key.to_string();
            task::spawn(async move {
                let response = client
                    .get(base_url)
                    .query(&[("address", address), ("key", &api_key)])
                    .send()
                    .await?;
                let data: Value = response.json().await?;
                Ok::<_, reqwest::Error>(data)
            })
        }),
    )
    .await
    .into_iter()
    .filter_map(|res| res.ok())
    .map(|res| res.unwrap_or_else(|_| Value::Null))
    .collect();

    let extracted_results: Vec<_> = results
        .into_iter()
        .filter_map(|data| data.get("results").cloned())
        .flat_map(|result| result.as_array().cloned().unwrap_or_default())
        .map(|item| item.get("formatted_address").cloned())
        .collect();

    let formatted_addresses = extracted_results
        .into_iter()
        .filter_map(|address| address.and_then(|val| val.as_str().map(String::from)))
        .collect::<Vec<_>>();

    println!("{:?}", formatted_addresses);

    Ok(())

    //still pending
}
