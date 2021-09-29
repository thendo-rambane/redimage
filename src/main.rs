use dotenv::dotenv;
mod reddit;
use reddit::RedditApi;

fn main() {
    dotenv().unwrap();
    let mut reddit_api = RedditApi::new();
    if reddit_api.authenticate().is_ok() {
        match reddit_api.get_account() {
            Ok(account) => {
                if let Ok(acc_json) = serde_json::to_string(&account) {
                    println!("{}", acc_json)
                } else {
                    println!("Main: could not serialize")
                }
            }
            Err(error) => {
                println!("{}", error)
            }
        };
        match reddit_api.get_following(100) {
            Ok(account) => {
                if let Ok(json) = serde_json::to_string(&account) {
                    println!("{}", json)
                } else {
                    println!("Main: could not serialize")
                }
            }
            Err(error) => {
                println!("{}", error)
            }
        };
    } else {
        println!("Auth Error")
    }
}
