use loco_rs::prelude::*;
use loco_rs::config::Config;
use serde::{Deserialize, Serialize};
use crate::entities::prelude::*;
use crate::models::users::{LoginParams, RegisterParams};
use serde_json::{json, Value};
use anyhow::{Result, anyhow};
use sea_orm::QueryOrder;

pub async fn get_menu_tree(db: &DatabaseConnection) -> Result<()> {

    let mut q = MenuEntity::find();

    // q = q.filter(NoteColumn::Id.eq(x));

    q = q.order_by_asc(MenuColumn::Weight);
    let res = q.all(db).await?;

    println!("{:?}", res);

    Ok(())
}