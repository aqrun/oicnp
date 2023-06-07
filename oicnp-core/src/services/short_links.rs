use anyhow::{anyhow, Result};
use crate::models::{NewShortLink, ShortLink};
use crate::{DbConn};
use crate::entities::{
    cms_short_links,
    prelude::{
        CmsShortLinks,
    },
};
use sea_orm::*;
use sea_query::{Alias, Expr};
use crate::utils::uuid;

pub async fn save_short_link(
    db: &DbConn,
    new_short_link: &NewShortLink
) -> Result<String> {
    let model_data = cms_short_links::ActiveModel {
        id: Set(uuid()),
        link: Set(String::from(&new_short_link.link)),
        name: Set(String::from(&new_short_link.name)),
        description: Set(String::from(&new_short_link.description)),
        deleted: Set(String::from(&new_short_link.deleted)),
        created_by: Set(String::from(&new_short_link.created_by)),
        ..Default::default()
    };

    let res: cms_short_links::Model = match model_data.insert(db).await {
        Ok(data) => data,
        Err(err) => {
            return Err(anyhow!("Short links save failed {}", err.to_string()));
        }
    };

    Ok(res.id)
}

pub async fn find_short_link_by_id(db: &DbConn, id: &str) -> Result<ShortLink> {
    let mut q = CmsShortLinks::find();
    q = q.filter(cms_short_links::Column::Id.eq(id));
    q = q.filter(cms_short_links::Column::Deleted.eq("0".to_string()));

    let res = q.into_model::<ShortLink>().one(db).await?;

    if let Some(data) = res {
        return Ok(data);
    }

    Err(anyhow!("Short link not exist: {}", id))
}