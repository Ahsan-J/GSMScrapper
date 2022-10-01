use std::env;
use gsm_scraper::process_gsm_url;

fn main()  {
    let args: Vec<String> = env::args().collect();
    let mobile_data = process_gsm_url(&args[1]);
    println!("Mobile Data {:?}", mobile_data);
}
