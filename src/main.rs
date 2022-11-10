use reqwest::Client;
use serde::Deserialize;
use tokio::process::Command;
//use std::process::Stdio;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const SIZE: i32 = 10;

#[derive(Deserialize, Clone)]
struct Cookbook {
	cookbook_name: String,
	cookbook_description: String,
    cookbook: String,
    cookbook_maintainer: String,
}

#[derive(Deserialize)]
struct SuperResponse {
	start: i32,
	total: i32,
	items: Vec<Cookbook>
}

#[tokio::main]
async fn main() -> Result<()> {
	let client = Client::new();	
	let mut url = "https://supermarket.chef.io/api/v1/cookbooks".to_string();
	let response = client.get(&url).send().await?;
	let response = response.json::<SuperResponse>().await?;

	let mut start = 0;
	let total = response.total;
	url.push_str("?item=10&start=");

	loop {
		// url set
		let num_str = start.to_string(); 
		let add_url = url.clone() + &num_str;
		//println!("{}", add_url);
		
		let response = client.get(&add_url).send().await?;
		let response = response.json::<SuperResponse>().await?;

		// download 10 entry
		for res in response.items.to_vec() {
			let child = Command::new("knife").arg("supermarket").arg("download")
				.arg(res.cookbook_name)											
				.spawn()
				.expect("failed to start dl")
				.wait()
				.await; 
			
			if let Ok(_) = child {
				println!("ok: {}", res.cookbook);
			}
		}

		// index set
		start = response.start + SIZE;

		if start >= total { break }
	}
	Ok(())
}
