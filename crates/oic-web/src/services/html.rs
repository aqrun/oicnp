use pulldown_cmark::{html, Options, Parser};

pub fn render_markdown(markdown: &str) -> String {
  // 0.13 版本：启用常用扩展功能
  let mut options = Options::empty();
  options.insert(Options::ENABLE_STRIKETHROUGH);
  options.insert(Options::ENABLE_TABLES);
  options.insert(Options::ENABLE_FOOTNOTES);
  options.insert(Options::ENABLE_TASKLISTS);
  options.insert(Options::ENABLE_SMART_PUNCTUATION);
  
  // 创建解析器并转换为 HTML
  let parser = Parser::new_ext(markdown, options);
  let mut html_output = String::new();
  html::push_html(&mut html_output, parser);
  html_output
}