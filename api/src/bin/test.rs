extern crate fast_log;

use std::path::PathBuf;
use std::fs;
use std::io::Read;
use api::services::G;
use regex::Regex;
use gray_matter::Matter;
use gray_matter::engine::YAML;
use api::dbs::{init_rbatis};
use std::sync::Arc;
use api::models::{
    Taxonomy, Node, NewNode, NewTaxonomy,
    NodeTagsMap, NodeCategoryMap, NodeBody,
};
use rbatis::rbatis::Rbatis;
use rbatis::crud::CRUD;
use rbatis::Error;
use serde::{Deserialize, Serialize};
use log::{ warn };
use api::services;

#[tokio::main]
async fn main() {
    fast_log::init_log("target/test.log",
                       // 1000,
                       log::Level::Warn,
                       None,
                       true);
    let rb = init_rbatis().await;
    let rb: Arc<Rbatis> = Arc::new(rb);
    let res = services::find_user_avatar(rb.clone(), &1).await;

    println!("----{:?}", res);
}