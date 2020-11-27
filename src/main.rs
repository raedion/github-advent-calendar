use github_rs::client::{Executor, Github};
use serde_json::Value;

fn main() {
    let client = Github::new("API TOKEN").unwrap();
    let me = client.get()
        .user()
        .execute::<Value>();

    match me {
        Ok((headers, status, json)) => {
            println!("{:#?}", headers);
            println!("status: {}", status);
            if let Some(json) = json{
                println!("JSON: {}", json);
            }
        },
        Err(e) => println!("error: {}", e)
    }
}
