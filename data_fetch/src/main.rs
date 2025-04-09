use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;
use anyhow::Result;

trait Pricing {
    fn fetch_price(&self) -> Result<f64>;
    fn save_price(&self, price: f64) -> Result<()>;
}

#[derive(Debug)]
struct Bitcoin;

impl Pricing for Bitcoin {
    fn fetch_price(&self) -> Result<f64> {
        #[derive(Deserialize)]
        struct ApiResponse {
            bitcoin: Coin,
        }

        #[derive(Deserialize)]
        struct Coin {
            usd: f64,
        }

        let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
        let response = ureq::get(url).call()?.into_string()?;
        let parsed: ApiResponse = serde_json::from_str(&response)?;
        Ok(parsed.bitcoin.usd)
    }

    fn save_price(&self, price: f64) -> Result<()> {
        let mut file = File::create("bitcoin_price.txt")?;
        writeln!(file, "Bitcoin price: ${}", price)?;
        Ok(())
    }
}

#[derive(Debug)]
struct Ethereum;

impl Pricing for Ethereum {
    fn fetch_price(&self) -> Result<f64> {
        #[derive(Deserialize)]
        struct ApiResponse {
            ethereum: Coin,
        }

        #[derive(Deserialize)]
        struct Coin {
            usd: f64,
        }

        let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
        let response = ureq::get(url).call()?.into_string()?;
        let parsed: ApiResponse = serde_json::from_str(&response)?;
        Ok(parsed.ethereum.usd)
    }

    fn save_price(&self, price: f64) -> Result<()> {
        let mut file = File::create("ethereum_price.txt")?;
        writeln!(file, "Ethereum price: ${}", price)?;
        Ok(())
    }
}

#[derive(Debug)]
struct SP500;

impl Pricing for SP500 {
    fn fetch_price(&self) -> Result<f64> {
        Ok(5000.00)
    }

    fn save_price(&self, price: f64) -> Result<()> {
        let mut file = File::create("sp500_price.txt")?;
        writeln!(file, "S&P 500 index: {}", price)?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let bitcoin = Bitcoin;
    let ethereum = Ethereum;
    let sp500 = SP500;

    loop {
        println!("Fetching prices..");

        for asset in [&bitcoin as &dyn Pricing, &ethereum, &sp500] {
            match asset.fetch_price() {
                Ok(price) => {
                    println!("Fetched price: ${}", price);
                    if let Err(e) = asset.save_price(price) {
                        eprintln!("Error saving price: {}", e);
                    }
                }
                Err(e) => eprintln!("Error fetching price: {}", e),
            }
        }

        println!("Sleeping for 10 seconds..\n");
        thread::sleep(Duration::from_secs(10));
    }
    