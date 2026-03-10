use oic_html::minify_html;

#[test]
fn compresses_simple_whitespace() {
    let input = r#"
        <div>
            <span> hello   world </span>
        </div>
    "#;

    let out = minify_html(input);

    // 主要验证：
    // 1）结构不变；
    // 2）"hello   world" 中连续空白压成单个空格。
    assert!(out.contains("<div"));
    assert!(out.contains("<span>hello world</span>"));
}

#[test]
fn collapses_whitespace_between_tags() {
    let input = "<div>  <span>1</span>   <span>2</span> </div>";
    let out = minify_html(input);

    // 标签间空白应该被压缩到最小（具体序列化风格由 html5ever 决定，这里只断言不含多余空白）
    assert!(!out.contains(">  <"));
    assert!(out.contains("<span>1</span>"));
    assert!(out.contains("<span>2</span>"));
}

#[test]
fn preserves_pre_whitespace() {
    let input = r#"
        <pre>
    line 1
      line 2    with    spaces
        </pre>
    "#;

    let out = minify_html(input);

    // pre 内部的空白应当原样（或至少不被 collapse_whitespace 破坏成一串单空格）
    // 这里不精确匹配整段，只检查特征片段仍存在
    assert!(out.contains("line 1"));
    assert!(out.contains("  line 2    with    spaces"));
}

#[test]
fn preserves_code_whitespace() {
    let input = r#"<code>fn  main() {  println!("hi"); }</code>"#;
    let out = minify_html(input);

    // code 内的多个空格不应被合并
    assert!(out.contains("fn  main() {  println!(\"hi\"); }"));
}

#[test]
fn preserves_textarea_newlines() {
    let input = r#"<textarea> line1
  line2
    line3</textarea>"#;

    let out = minify_html(input);

    // textarea 里的换行结构要保持
    assert!(out.contains("line1"));
    assert!(out.contains("line2"));
    assert!(out.contains("line3"));
}

#[test]
fn preserves_script_content() {
    let input = r#"
        <script>
            function  test() {
                console.log(  "hi"  );
            }
        </script>
    "#;

    let out = minify_html(input);

    // 不做 JS 压缩，只要不把内部空格乱改即可
    assert!(out.contains("function  test()"));
    assert!(out.contains("console.log(  \"hi\"  );"));
}

#[test]
fn preserves_style_content() {
    let input = r#"
        <style>
          .foo  {   margin:  0  auto; }
        </style>
    "#;

    let out = minify_html(input);

    // 同样不对 CSS 内容做压缩
    assert!(out.contains(".foo  {   margin:  0  auto; }"));
}

#[test]
fn handles_chinese_and_multibyte() {
    let input = r#"
        <div>  Rust 灵犀   技术 博客  </div>
    "#;

    let out = minify_html(input);

    // 中文 + 空格正常保留顺序，空白压缩
    assert!(out.contains("Rust 灵犀 技术 博客"));
}

#[test]
fn idempotent_on_already_minified_html() {
    let input = "<div><span>hi</span><span>there</span></div>";
    let out = minify_html(input);

    // 再跑一次不应破坏结构或引入空白
    let out2 = minify_html(&out);
    assert_eq!(out, out2);
}