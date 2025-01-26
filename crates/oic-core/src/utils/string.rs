use regex::Regex;
use super::slugify_paths_without_date;
use crate::utils::utc_now;
use loco_rs::prelude::*;

pub fn generate_slug(file_name: &str) -> (String, String) {
    let (date_time, slug) = slugify_paths_without_date(file_name);
    (date_time, slug)
}

pub fn is_valid_matter_content(content: &str) -> bool {
    let reg_matter = Regex::new(r#"---([\s\S]*)---"#)
        .expect("Matter reg not valid");
    reg_matter.is_match(content)
}

pub fn default_string() -> String {
    String::from("")
}

pub fn default_option_string() -> Option<String> {
    Some(String::from(""))
}

pub fn default_i64() -> i64 {
    0
}

pub fn default_i32() -> i32 {
    0
}

pub fn default_date_time() -> DateTime {
    utc_now()
}
