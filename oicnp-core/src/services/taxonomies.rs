use crate::{
    entities::prelude::*,
    models::{NewCategory, NewTag},
    DbConn,
};
use anyhow::{anyhow, Result};
use sea_orm::*;
use sea_orm::sea_query::Expr;

/// 根据ID查找单个分类
pub async fn find_taxonomy_by_id(db: &DbConn, cat_id: i64) -> Result<CategoryModel> {
    let q = CategoryEntity::find()
        .filter(CategoryColumn::CatId.eq(cat_id));

    if let Some(res) = q.into_model::<CategoryModel>()
        .one(db)
        .await?
    {
        return Ok(res);
    }

    Err(anyhow!("分类不存在 {}", cat_id))
}

/// 根据VID查找单个分类
pub async fn find_category_by_vid(db: &DbConn, vid: &str) -> Result<CategoryModel> {
    let mut q = CategoryEntity::find();
    q = q.filter(CategoryColumn::CatVid.eq(vid));

    let res = match q.into_model::<CategoryModel>().one(db).await {
        Ok(data) => data,
        Err(err) => {
            return Err(anyhow!("Categories not exist: {}, {:?}", vid, err));
        }
    };

    if let Some(data) = res {
        return Ok(data);
    }

    Err(anyhow!("Categories not exist: {}", vid))
}

pub async fn find_tag_by_vid(db: &DbConn, vid: &str) -> Result<TagModel> {
    let mut q = TagEntity::find();
    q = q.filter(TagColumn::TagVid.eq(vid));

    match q.into_model::<TagModel>().one(db).await {
        Ok(data) => {
            if let Some(data) = data {
                return Ok(data);
            }
        },
        Err(err) => {
            return Err(anyhow!("Tag not exist: {}, {:?}", vid, err));
        }
    };

    Err(anyhow!("Tag not exist: {}", vid))
}

pub async fn find_tag_by_tag_id(db: &DbConn, tag_id: i64) -> Result<TagModel> {
    let mut q = TagEntity::find();
    q = q.filter(TagColumn::TagId.eq(tag_id));

    match q.into_model::<TagModel>().one(db).await {
        Ok(data) => {
            if let Some(data) = data {
                return Ok(data);
            }
        },
        Err(err) => {
            return Err(anyhow!("Tag not exist: {}, {:?}", tag_id, err));
        }
    };

    Err(anyhow!("Tag not exist: {}", tag_id))
}

/// 保存一条数据
pub async fn save_category(
    db: &DbConn,
    new_taxonomy: &NewCategory,
) -> Result<CategoryModel> {
    if let Ok(data) = find_category_by_vid(db, &new_taxonomy.vid).await {
        return Ok(data);
    }

    let t = CategoryActiveModel {
        cat_vid: Set(String::from(&new_taxonomy.vid)),
        cat_pid: Set(new_taxonomy.pid),
        cat_name: Set(String::from(&new_taxonomy.name)),
        cat_desc: Set(String::from(&new_taxonomy.desc)),
        cat_desc_format: Set(String::from(&new_taxonomy.desc)),
        weight: Set(new_taxonomy.weight),
        ..Default::default()
    };

    let res: CategoryModel = t.insert(db).await?;

    Ok(res)
}

/// 保存多条数据
pub async fn save_categories(
    db: &DbConn,
    new_taxonomies: &Vec<NewCategory>,
) -> Result<String> {
    let mut filtered_categories: Vec<&NewCategory> = Vec::new();

    for item in new_taxonomies.iter() {
        if let Err(_err) = find_category_by_vid(db, item.vid.as_str()).await {
            filtered_categories.push(item);
        }
    }

    let categories_models = filtered_categories
        .into_iter()
        .map(|item| CategoryActiveModel {
            cat_vid: Set(String::from(&item.vid)),
            cat_pid: Set(item.pid),
            cat_name: Set(String::from(&item.name)),
            cat_desc: Set(String::from(&item.desc)),
            cat_desc_format: Set(String::from(&item.desc_format)),
            weight: Set(item.weight),
            ..Default::default()
        })
        .collect::<Vec<CategoryActiveModel>>();

    match CategoryEntity::insert_many(categories_models).exec(db).await {
        Ok(_data) => Ok("".to_string()),
        Err(err) => Err(anyhow!("Save Category failed {:?}", err)),
    }
}

pub async fn save_tags(
    db: &DbConn,
    new_tags: &[NewTag]
) -> Result<String> {
    let mut filtered_new_tags: Vec<NewTag> = Vec::new();

    for item in new_tags.iter() {
        if let Err(_err) = find_tag_by_vid(db, item.vid.as_str()).await {
            filtered_new_tags.push(item.clone());
        }
    }

    let tag_models = filtered_new_tags
        .into_iter()
        .map(|item| TagActiveModel {
            tag_id: Set(0),
            tag_vid: Set(String::from(&item.vid)),
            tag_name: Set(String::from(&item.name)),
            weight: Set(item.weight),
            tag_count: Set(item.count as i64),
        })
        .collect::<Vec<TagActiveModel>>();

    match TagEntity::insert_many(tag_models).exec(db).await {
        Ok(_data) => Ok("".to_string()),
        Err(err) => Err(anyhow!("Save tags failed {:?}", err)),
    }
}

pub async fn update_tag_count_by_id(db: &DbConn, tag_id: i64) -> Result<String> {
    let tag = find_tag_by_tag_id(db, tag_id).await?;

    TagEntity::update_many()
        .col_expr(TagColumn::TagCount, Expr::value(tag.tag_count + 1))
        .filter(TagColumn::TagId.eq(tag_id))
        .exec(db)
        .await?;

    Ok("update success".to_string())
}
