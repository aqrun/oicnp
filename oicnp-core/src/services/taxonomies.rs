use anyhow::{anyhow, Result};
use crate::{
    DatabaseConnection,
    models::{
        NewTaxonomy, Taxonomies,
    },
    typings::{
        NodeBundle,
    },
    entities::{
        prelude::{
            CmsTaxonomies,
        },
        cms_taxonomies,
    },
};
use sea_orm::*;
use crate::utils::uuid;

pub async fn find_taxonomy_by_tid() {

}

pub async fn find_taxonomy_by_vid(
    db: &DatabaseConnection,
    vid: &str,
) -> Result<Taxonomies> {
    let mut q = CmsTaxonomies::find();
    q = q.filter(cms_taxonomies::Column::Vid.eq(vid));

    let res = match q.into_model::<Taxonomies>().one(db).await {
        Ok(data) => data,
        Err(err) => {
            return Err(anyhow!("Taxonomies not exist: {}, {:?}", vid, err));
        },
    };

    if let Some(data) = res {
        return Ok(data);
    }

    Err(anyhow!("Taxonomies not exist: {}", vid))
}

pub async fn save_taxonomy(
    db: &DatabaseConnection,
    new_taxonomy: &NewTaxonomy
) -> Result<Taxonomies> {
    if let Ok(data) = find_taxonomy_by_vid(db, &new_taxonomy.vid).await {
        return Ok(data);
    }

    let t = cms_taxonomies::ActiveModel {
        tid: Set(uuid()),
        vid: Set(Some(String::from(&new_taxonomy.vid))),
        pid: Set(Some(String::from(&new_taxonomy.pid))),
        name: Set(Some(String::from(&new_taxonomy.name))),
        description: Set(Some(String::from(&new_taxonomy.description))),
        description_format: Set(Some(String::from(&new_taxonomy.description_format))),
        weight: Set(Some(new_taxonomy.weight)),
        ..Default::default()
    };

    let res: cms_taxonomies::Model = t.insert(db).await?;
    let taxonomy = Taxonomies::from_model(&res);
    Ok(taxonomy)
}