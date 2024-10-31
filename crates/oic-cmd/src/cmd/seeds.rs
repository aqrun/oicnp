use reqwest::Client;
use loco_rs::{
    // environment::Environment,
    Result,
    Error,
};
use serde_json::{json, Value};

const ADMIN_API: &'static str = "http://localhost:5150/api/admin";


pub async fn run() {
    if let Err(err) = seed_notes().await {
        println!("SeedDataErr: {}", err);
    }
    
}

async fn seed_notes() -> Result<()> {
    println!("1123");
    // let cfg = Environment::Development.load()?;
    let params = json!({
        "title": "note-alex",
        "content": "note-alex-content"
    });

    let res = post("/note/add", &params).await?;

    println!("--res value: {}", res);

    Ok(())
}

/// post 请求
async fn post(uri: &str, params: &Value) -> Result<Value> {
    let client = Client::new();

    let url = format!("{ADMIN_API}{uri}");

    let res = client.post(url.as_str())
        .json(params)
        .send()
        .await;

    let res = match res {
        Ok(res) => {
            if res.status().is_success() {
                let a = res.json::<Value>()
                    .await
                    .map_err(|err| {
                        Error::BadRequest(format!("{err}"))
                    })?;
                a
            } else {
                return Err(Error::BadRequest(format!("接口请求失败, 错误码：{}，URL：{}  ---{:?}", res.status(), res.url(), res)));
            }
        },
        Err(err) => {
            return Err(Error::BadRequest(format!("{err}")));
        },
    };

    Ok(res)
}