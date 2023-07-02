use regex::Regex;
use oicnp_core::utils::{RFC3339_DATE, slugify_paths_without_date};

pub fn generate_slug(file_name: &str) -> (String, String) {
    let (date_time, slug) = slugify_paths_without_date(file_name);
    (date_time, slug)
}

pub fn is_valid_matter_content(content: &str) -> bool {
    let reg_matter = Regex::new(r#"---([\s\S]*)---"#)
        .expect("Matter reg not valid");
    reg_matter.is_match(content)
}
