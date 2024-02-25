use anyhow::{anyhow, Result};
use crate::models::{NewShortLink, ShortLink};
use crate::DbConn;
use crate::entities::{
    short_link,
    prelude::*,
};
use sea_orm::*;
use sea_query::Expr;

pub async fn save_short_link(
    db: &DbConn,
    new_short_link: &NewShortLink
) -> Result<i64> {
    let model_data = short_link::ActiveModel {
        link: Set(String::from(&new_short_link.link)),
        name: Set(String::from(&new_short_link.name)),
        description: Set(String::from(&new_short_link.description)),
        deleted: Set(String::from(&new_short_link.deleted)),
        created_by: Set(new_short_link.created_by),
        ..Default::default()
    };

    let res: short_link::Model = match model_data.insert(db).await {
        Ok(data) => data,
        Err(err) => {
            return Err(anyhow!("Short links save failed {}", err.to_string()));
        }
    };

    Ok(res.id)
}

pub async fn find_short_link_by_id(db: &DbConn, id: &str) -> Result<ShortLink> {
    let mut q = ShortLinkEntity::find();
    q = q.filter(short_link::Column::Id.eq(id));
    q = q.filter(short_link::Column::Deleted.eq("0".to_string()));

    let res = q.into_model::<ShortLink>().one(db).await?;

    if let Some(data) = res {
        return Ok(data);
    }

    Err(anyhow!("Short link not exist: {}", id))
}

pub async fn update_short_link_viewed(db: &DbConn, id: &str) -> Result<i32> {
    if let Ok(res) = find_short_link_by_id(db, id).await {
        ShortLinkEntity::update_many()
            .col_expr(short_link::Column::Viewed, Expr::value(res.viewed + 1))
            .filter(short_link::Column::Id.eq(id))
            .exec(db)
            .await?;
        return Ok(res.viewed + 1);
    }
    Ok(0)
}