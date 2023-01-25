use reqwest::{Client, Proxy};

#[tokio::main]
async fn main() {
    let client = Client::new();

    let resp = client
        .get("http://ipinfo.io")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{:#?}", resp);

    let proxy_client = Client::builder()
        .proxy(Proxy::all("").unwrap()) // http://user:pass@host:port
        .build()
        .unwrap();

    let resp = proxy_client
        .get("http://ipinfo.io")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{:#?}", resp);
}
