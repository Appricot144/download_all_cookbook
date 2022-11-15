use reqwest::Client;
use serde::Deserialize;
use tokio::process::Command;
//use std::process::Stdio;
use std::env;

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

// DLするtar-ballのfile pathをパースするが面倒なのでknife supermarket コマンドを実行している。
// <apiからDLする方法>
// GET /api/v1/cookbooks/COOKBOOK_NAME/versions/latestのレスポンスに
// "file": "<file path>" tar-ballのファイルパスがある
// このURIにgetすると多分amazonawsにリダイレクトするので-Lオプションをつけて
// curl -X "GET" -L "<file_path>" -o cookbook_name.tar.gz 
#[tokio::main]
async fn main() -> Result<()> {
	let args: Vec<String> = env::args().collect(); // download_all_cookbook --start 10 	
	if args.len() < 3 { panic!("no start-option \"--start NUM\""); }
	
	let client = Client::new();	
	let mut url = "https://supermarket.chef.io/api/v1/cookbooks".to_string();
	let response = client.get(&url).send().await?;
	let response = response.json::<SuperResponse>().await?;

	let mut start: i32 = args[2].parse()?;
	let total = response.total;
	url.push_str("?items=10&start=");

	loop {
		// set url
		let num_str = start.to_string();
		let add_url = url.clone() + &num_str;
		
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

		// set index
		start = response.start + SIZE;

		if start >= total { break }
	}
	Ok(())
}
