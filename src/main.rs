use std::env;
use GSMScraper::process_gsm_url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let mobile_data = process_gsm_url(&args[1]).await;
    println!("Mobile Data {:?}", mobile_data);
    Ok(())
}
