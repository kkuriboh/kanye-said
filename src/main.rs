use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Quote {
    quote: String
}

impl Quote {
    async fn get(api_key: &str) -> Result<Self, ExitFailure> {
        let url = Url::parse(&*api_key)?;
        let res = reqwest::get(url).await?.json::<Quote>().await?;
        Ok(res)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let api_key = "https://api.kanye.rest";
    let res = Quote::get(&api_key).await?;
    println!("Kanye said: {}", res.quote);
    Ok(())
}