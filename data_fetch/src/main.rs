use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;
use anyhow::Result;

#[derive(Debug)]
enum Asset {
    Bitcoin,
    Ethereum,
    SP500,
}

impl Asset {
    fn fetch_price(&self) -> Result<f64> {
        #[derive(Deserialize)]
        struct Coin {
            usd: f64,
        }

        #[derive(Deserialize)]
        struct BitcoinResponse {
            bitcoin: Coin,
        }

        #[derive(Deserialize)]
        struct EthereumResponse {
            ethereum: Coin,
        }

        match self {
            Asset::Bitcoin => {
                let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";
                let response = ureq::get(url).call()?.into_string()?;
                let parsed: BitcoinResponse = serde_json::from_str(&response)?;
                Ok(parsed.bitcoin.usd)
            }
            Asset::Ethereum => {
                let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";
                let response = ureq::get(url).call()?.into_string()?;
                let parsed: EthereumResponse = serde_json::from_str(&response)?;
                Ok(parsed.ethereum.usd)
            }
            Asset::SP500 => {
                // Simulated or fixed value for SP500
                Ok(5000.00)
            }
        }
    }

    fn save_price(&self, price: f64) -> Result<()> {
        let filename = match self {
            Asset::Bitcoin => "bitcoin_price.txt",
            Asset::Ethereum => "ethereum_price.txt",
            Asset::SP500 => "sp500_price.txt",
        };

        let label = match self {
            Asset::Bitcoin => "Bitcoin price",
            Asset::Ethereum => "Ethereum price",
            Asset::SP500 => "S&P 500 index",
        };

        let mut file = File::create(filename)?;
        writeln!(file, "{}: ${}", label, price)?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let assets = vec![
        Asset::Bitcoin,
        Asset::Ethereum,
        Asset::SP500,
    ];

    loop {
        println!("Fetching prices...");

        for asset in &assets {
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

        println!("Sleeping for 10 seconds...\n");
        thread::sleep(Duration::from_secs(10));
    }
}
