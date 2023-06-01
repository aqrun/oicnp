use crate::models::{
    Taxonomies, NewTaxonomy, NodeTaxonomiesMap,
};
use crate::typings::{
    TaxonomyBundle, NodeBundle,
};
use oicnp_core::{
    DatabaseConnection,
    entities::{
        cms_nodes,
    },
    prelude::{
        anyhow::{anyhow, Result}
    }
};

// #[py_sql("UPDATE taxonomies SET count = count + 1 WHERE tid = #{tid}")]
pub async fn taxonomy_increase_count(
    db: &DatabaseConnection,
    tid: &i32,
) -> Result<()> {
    todo!()
}

pub async fn find_taxonomy(
    db: &DatabaseConnection,
    name: &str,
    bundle: &TaxonomyBundle
) -> Result<Taxonomies> {
    // let wrapper = rb.new_wrapper()
    //     .eq("name", name)
    //     .eq("bundle", bundle.to_string());

    // let result: Result<Option<Taxonomies>, Error> = rb.fetch_by_wrapper(wrapper)
    //     .await;

    // if let Ok(res) = result {
    //     if let Some(taxonomy) = res {
    //         return Ok(taxonomy);
    //     }
    // }
    Err(anyhow!("Taxonomy not exist, {}", name))
}

pub async fn save_taxonomy(
    db: &DatabaseConnection,
    name: &str,
    bundle: &TaxonomyBundle,
) -> Result<Taxonomies> {
    // if let Ok(tag) = find_taxonomy(
    //     rb.clone(),
    //     name,
    //     bundle
    // ).await {
    //     return Ok(tag);
    // }

    // let new_tag = NewTaxonomy {
    //     vid: String::from(name),
    //     pid: 0,
    //     bundle: bundle.to_string(),
    //     name: String::from(name),
    //     description: String::from(""),
    //     description_format: String::from(""),
    //     weight: 0,
    // };

    // if let Ok(_res) = rb.save(&new_tag, &[]).await {
    //     if let Ok(tag) = find_taxonomy(rb.clone(), name, bundle).await {
    //         return Ok(tag);
    //     }
    // }

    Err(anyhow!("Save tag failed: {}", name))
}

pub async fn find_node_taxonomy_map(
    db: &DatabaseConnection,
    nid: i32,
    tid: i32,
) -> Result<NodeTaxonomiesMap> {
    // let w = rb.new_wrapper()
    //     .eq("nid", nid)
    //     .eq("tid", tid);
    // let res: Result<Option<NodeTaxonomiesMap>, Error> = rb.fetch_by_wrapper(w.clone()).await;

    // if let Ok(res) = res {
    //     if let Some(res) = res {
    //         return Ok(res);
    //     }
    // }
    Err(anyhow!("map not exist"))
}

pub async fn save_node_taxonomy_map(
    db: &DatabaseConnection,
    nid: i32,
    tid: i32,
    bundle: &TaxonomyBundle
) -> Result<NodeTaxonomiesMap> {
    // if let Ok(map) = find_node_taxonomy_map(rb.clone(), nid, tid).await {
    //     return Ok(map);
    // }

    // let map = NodeTaxonomiesMap {
    //     bundle: NodeBundle::Article.to_string(),
    //     nid,
    //     tid
    // };
    // if let Err(err) = rb.save(&map, &[]).await {
    //     return Err(err.to_string());
    // }

    // if let TaxonomyBundle::Tag = bundle {
    //     if let Err(err) = taxonomy_increase_count(rb.clone(), &tid).await {
    //         return Err(format!("Tag increase count error, {}", err.to_string()));
    //     }
    // }

    // if let Ok(map) = find_node_taxonomy_map(rb.clone(), nid, tid).await {
    //     return Ok(map);
    // }
    Err(anyhow!("Node Taxonomy map save failed"))
}

pub async fn save_tags(
    db: &DatabaseConnection,
    tags: &Vec<String>,
    nid: i32
) -> Result<Vec<Taxonomies>> {
    // let mut tags_list: Vec<Taxonomies> = vec!();

    // for tag_name in tags {
    //     if let Ok(tag) = save_taxonomy(rb.clone(), &tag_name, &TaxonomyBundle::Tag).await {
    //         let res = save_node_taxonomy_map(
    //             rb.clone(), nid, tag.tid, &TaxonomyBundle::Tag
    //         )
    //             .await;

    //         match res {
    //             Ok(_map) => tags_list.push(tag),
    //             Err(err) => return Err(err),
    //         }

    //     }
    // }
    let tags_list = Vec::new();
    Ok(tags_list)
}

pub async fn save_category(
    db: &DatabaseConnection,
    category_name: &str,
    nid: i32
) -> Result<Taxonomies> {
    // if let Ok(cat) = save_taxonomy(rb.clone(), category_name, &TaxonomyBundle::Category).await {
    //     let _res = save_node_taxonomy_map(
    //         rb.clone(), nid, cat.tid, &TaxonomyBundle::Category
    //     ).await?;

    //     return Ok(cat);
    // }

    Err(anyhow!("Save Category failed: {}", category_name))
}