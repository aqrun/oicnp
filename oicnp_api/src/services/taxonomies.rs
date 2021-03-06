use std::sync::Arc;
use rbatis::crud::CRUD;
use rbatis::rbatis::Rbatis;
use rbatis::Error;
use rbatis::py_sql;
use crate::models::{
    Taxonomies, NewTaxonomy, NodeTaxonomiesMap,
};
use crate::typings::{
    TaxonomyBundle, NodeBundle,
};

#[py_sql("UPDATE taxonomies SET count = count + 1 WHERE tid = #{tid}")]
pub async fn taxonomy_increase_count(
    rb: Arc<Rbatis>,
    tid: &i32,
) -> Result<(), Error> {
    todo!()
}

pub async fn find_taxonomy(rb: Arc<Rbatis>, name: &str, bundle: &TaxonomyBundle) -> Result<Taxonomies, String> {
    let wrapper = rb.new_wrapper()
        .eq("name", name)
        .eq("bundle", bundle.to_string());

    let result: Result<Option<Taxonomies>, Error> = rb.fetch_by_wrapper(wrapper)
        .await;

    if let Ok(res) = result {
        if let Some(taxonomy) = res {
            return Ok(taxonomy);
        }
    }
    Err(format!("Taxonomy not exist, {}", name))
}

pub async fn save_taxonomy(rb: Arc<Rbatis>, name: &str, bundle: &TaxonomyBundle) -> Result<Taxonomies, String> {
    if let Ok(tag) = find_taxonomy(
        rb.clone(),
        name,
        bundle
    ).await {
        return Ok(tag);
    }

    let new_tag = NewTaxonomy {
        vid: String::from(name),
        pid: 0,
        bundle: bundle.to_string(),
        name: String::from(name),
        description: String::from(""),
        description_format: String::from(""),
        weight: 0,
    };

    if let Ok(_res) = rb.save(&new_tag, &[]).await {
        if let Ok(tag) = find_taxonomy(rb.clone(), name, bundle).await {
            return Ok(tag);
        }
    }

    Err(format!("Save tag failed: {}", name))
}

pub async fn find_node_taxonomy_map(rb: Arc<Rbatis>, nid: i32, tid: i32) -> Result<NodeTaxonomiesMap, String> {
    let w = rb.new_wrapper()
        .eq("nid", nid)
        .eq("tid", tid);
    let res: Result<Option<NodeTaxonomiesMap>, Error> = rb.fetch_by_wrapper(w.clone()).await;

    if let Ok(res) = res {
        if let Some(res) = res {
            return Ok(res);
        }
    }
    Err(format!("map not exist"))
}

pub async fn save_node_taxonomy_map(
    rb: Arc<Rbatis>,
    nid: i32,
    tid: i32,
    bundle: &TaxonomyBundle
) -> Result<NodeTaxonomiesMap, String> {
    if let Ok(map) = find_node_taxonomy_map(rb.clone(), nid, tid).await {
        return Ok(map);
    }

    let map = NodeTaxonomiesMap {
        bundle: NodeBundle::Article.to_string(),
        nid,
        tid
    };
    if let Err(err) = rb.save(&map, &[]).await {
        return Err(err.to_string());
    }

    if let TaxonomyBundle::Tag = bundle {
        if let Err(err) = taxonomy_increase_count(rb.clone(), &tid).await {
            return Err(format!("Tag increase count error, {}", err.to_string()));
        }
    }

    if let Ok(map) = find_node_taxonomy_map(rb.clone(), nid, tid).await {
        return Ok(map);
    }
    Err(format!("Node Taxonomy map save failed"))
}

pub async fn save_tags(rb: Arc<Rbatis>, tags: &Vec<String>, nid: i32) -> Result<Vec<Taxonomies>, String> {
    let mut tags_list: Vec<Taxonomies> = vec!();

    for tag_name in tags {
        if let Ok(tag) = save_taxonomy(rb.clone(), &tag_name, &TaxonomyBundle::Tag).await {
            let res = save_node_taxonomy_map(
                rb.clone(), nid, tag.tid, &TaxonomyBundle::Tag
            )
                .await;

            match res {
                Ok(_map) => tags_list.push(tag),
                Err(err) => return Err(err),
            }

        }
    }
    Ok(tags_list)
}

pub async fn save_category(rb: Arc<Rbatis>, category_name: &str, nid: i32) -> Result<Taxonomies, String> {
    if let Ok(cat) = save_taxonomy(rb.clone(), category_name, &TaxonomyBundle::Category).await {
        let _res = save_node_taxonomy_map(
            rb.clone(), nid, cat.tid, &TaxonomyBundle::Category
        ).await?;

        return Ok(cat);
    }

    Err(format!("Save Category failed: {}", category_name))
}