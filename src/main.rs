use std::env::args;

use toml::Value;

#[tokio::main]
async fn main() {
    // let resp = reqwest::get("   ")
    //     .await?
    //     .json::<HashMap<String, String>>()
    //     .await?;
    // println!("{:#?}", resp);
    // Ok(())

    let ape_toml = std::fs::read_to_string("ape.toml").expect("couldnt open ape.toml");

    let ape_toml: Value = toml::from_str(&ape_toml).unwrap();
    let ape_toml = ape_toml["ape"].clone();

    let args: Vec<String> = args().collect();
    let mut args = args.iter();
    args.next();
    let api = args.next().unwrap();

    let api_info = ape_toml[api].clone();

    let url = api_info.get("url").expect("url is essential!");
    let url = url.as_str().expect("Expected a string for a url");

    let method = api_info.get("method");
    let method = if method.is_some() {
        method.unwrap().as_str().expect("Expected a string")
    } else {
        "get"
    };

    let client = reqwest::Client::new();

    let resp = match method {
        "get" => client.get(url).send().await.unwrap().text().await.unwrap(),
        // "post" => {
        // let post_body = api_info.get("body");
        // let req = if let Some(post_body) = post_body {
        //     let post_body = post_body.as_str().unwrap();
        //     client.post(url).body(post_body).send().await.unwrap()
        // } else {
        //     client.post(url).send().await.unwrap()
        // };
        // req.text().await.unwrap()
        // }
        _ => {
            panic!("what method is `{}`.. never heard of it", method);
        }
    };
    println!("{}", resp);
}
