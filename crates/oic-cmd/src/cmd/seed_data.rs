use std::fs::File;
use reqwest::Client;
use loco_rs::{
    // environment::Environment,
    Result,
    Error,
};
use oic_core::{
    models::notes::CreateNoteReqParams,
    typings::JsonRes,
};
use serde::Serialize;
use serde_json::Value;

const ADMIN_API: &'static str = "http://localhost:5150/api/admin";

pub async fn run() -> Result<()> {
    seed_notes().await?;
    Ok(())
}

///
/// note 种子
/// 
async fn seed_notes() -> Result<()> {
    // let cfg = Environment::Development.load()?;

    let file_path = "crates/oic-cmd/seeds/notes.yaml";
    let seed_data: Vec<CreateNoteReqParams> = serde_yaml::from_reader(File::open(file_path)?)?;

    // for row in seed_data {
    //     println!("{:?}", row);
    // }

    let res = post("/note/add-multi", &seed_data).await?;

    if res.code.is_some() && res.clone().code.unwrap().eq("200") {
        println!("添加成功: {:?}", res);
    } else {
        let res = res.clone();
        println!("添加失败: {}, {}", res.code.unwrap(), res.message.unwrap());
    }

    Ok(())
}

/// post 请求
async fn post(uri: &str, params: &impl Serialize) -> Result<JsonRes<Value>> {
    let client = Client::new();
    let url = format!("{ADMIN_API}{uri}");
    let res = client.post(url.as_str())
        .json(params)
        .send()
        .await;

    let res = match res {
        Ok(res) => {
            if res.status().is_success() {
                let a = res.json::<JsonRes<Value>>()
                    .await
                    .map_err(|err| {
                        Error::BadRequest(format!("1-{err}"))
                    })?;
                a
            } else {
                return Err(Error::BadRequest(
                    format!("接口请求失败, 错误码：{}，URL：{}",
                    res.status(),
                    res.url()
                )));
            }
        },
        Err(err) => {
            return Err(Error::BadRequest(format!("{err}")));
        },
    };

    Ok(res)
}