use crate::utils::uuid;
use crate::{
    entities::{cms_taxonomies, prelude::CmsTaxonomies},
    models::{NewTaxonomy, Taxonomies},
    typings::NodeBundle,
    DatabaseConnection,
};
use anyhow::{anyhow, Result};
use sea_orm::*;

pub async fn find_taxonomy_by_tid() {}

pub async fn find_taxonomy_by_vid(db: &DatabaseConnection, vid: &str) -> Result<Taxonomies> {
    let mut q = CmsTaxonomies::find();
    q = q.filter(cms_taxonomies::Column::Vid.eq(vid));

    let res = match q.into_model::<Taxonomies>().one(db).await {
        Ok(data) => data,
        Err(err) => {
            return Err(anyhow!("Taxonomies not exist: {}, {:?}", vid, err));
        }
    };

    if let Some(data) = res {
        return Ok(data);
    }

    Err(anyhow!("Taxonomies not exist: {}", vid))
}

/// 保存一条数据
pub async fn save_taxonomy(
    db: &DatabaseConnection,
    new_taxonomy: &NewTaxonomy,
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

/// 保存多条数据
pub async fn save_taxonomies(
    db: &DatabaseConnection,
    new_taxonomies: &Vec<NewTaxonomy>,
) -> Result<String> {
    let mut filtered_taxonomies: Vec<&NewTaxonomy> = Vec::new();

    for item in new_taxonomies.iter() {
        if let Err(_err) = find_taxonomy_by_vid(db, item.vid.as_str()).await {
            filtered_taxonomies.push(item);
        }
    }

    let taxonomy_models = filtered_taxonomies
        .into_iter()
        .map(|item| cms_taxonomies::ActiveModel {
            tid: Set(uuid()),
            vid: Set(Some(String::from(&item.vid))),
            pid: Set(Some(String::from(&item.pid))),
            name: Set(Some(String::from(&item.name))),
            description: Set(Some(String::from(&item.description))),
            description_format: Set(Some(String::from(&item.description_format))),
            weight: Set(Some(item.weight)),
            ..Default::default()
        })
        .collect::<Vec<cms_taxonomies::ActiveModel>>();

    match CmsTaxonomies::insert_many(taxonomy_models).exec(db).await {
        Ok(_data) => Ok("".to_string()),
        Err(err) => Err(anyhow!("Save Taxonomies failed {:?}", err)),
    }
}
