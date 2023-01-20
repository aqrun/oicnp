use regex::Regex;

pub fn generate_slug(file_name: &str) -> (String, String) {
    let mut file_arr: Vec<&str> = file_name.split(".").collect();
    file_arr.pop();
    let new_file_name = file_arr.join("-");
    file_arr = new_file_name.split("-").collect();
    let year = file_arr[0];
    let month = file_arr[1];
    let day = file_arr[2]; // .parse::<i32>().expect("Day error");
    let date = format!("{}-{}-{}", year, month, day);

    let re = Regex::new(r"[\.+\s]+").unwrap();
    let source_title = file_arr[3..].join("-");
    let title = re.replace_all(&source_title, "-");
    let slug = format!("{}-{}", date, title);

    let res = (date, slug);
    return res;
}

pub fn is_valid_matter_content(content: &str) -> bool {
    let reg_matter = Regex::new(r#"---([\s\S]*)---"#)
        .expect("Matter reg not valid");
    reg_matter.is_match(content)
}
