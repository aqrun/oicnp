use zino::{
    prelude::*,
    Request,
    Response,
    Result,
};
use crate::models::User;

pub async fn list(req: Request) -> Result {
    let query = Query::new(json!({
        "age": { "$gt": 20 },
    }));
    let records = User::find::<Map>(&query)
        .await
        .extract(&req)?;

    let data = json!({
        "name": "Alex",
        "age": 18,
        "users": records,
    });

    let mut res = Response::default()
        .context(&req);
    res.set_json_data(data);
    Ok(res.into())
}

