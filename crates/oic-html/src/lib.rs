use html5ever::tendril::TendrilSink;
use html5ever::{parse_document, serialize, serialize::SerializeOpts};
use markup5ever_rcdom::{Handle, NodeData, RcDom, SerializableHandle};

/// 使用 html5ever 解析并简化 HTML：
/// - 解析为 DOM
/// - 对非 <pre>/<code>/<textarea>/<script>/<style> 下的文本节点压缩空白
/// - 重新序列化为紧凑的 HTML
pub fn minify_html(input: &str) -> String {
    // 解析为 DOM
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut input.as_bytes())
        .unwrap_or_else(|_| RcDom::default());

    // 在 DOM 上压缩空白
    minify_node(&dom.document, false);

    // 重新序列化
    let mut out = Vec::new();
    if let Err(_e) = serialize(
        &mut out,
        &SerializableHandle::from(dom.document.clone()),
        SerializeOpts {
            // 不 pretty-print，尽量紧凑输出
            scripting_enabled: false,
            create_missing_parent: true,
            ..Default::default()
        },
    ) {
        return input.to_owned();
    }

    String::from_utf8(out).unwrap_or_else(|_| input.to_owned())
}

/// 针对 Bytes 版本的封装。
pub fn minify_html_bytes(input: &[u8]) -> Vec<u8> {
    match std::str::from_utf8(input) {
        Ok(s) => minify_html(s).into_bytes(),
        Err(_) => input.to_vec(),
    }
}

fn minify_node(handle: &Handle, in_preserve_whitespace: bool) {
    let node = handle;

    // 判断当前节点是否是需要保留空白的标签
    let mut preserve = in_preserve_whitespace;
    if let NodeData::Element { ref name, ref attrs, .. } = node.data {
        let local = name.local.as_ref().to_ascii_lowercase();
        if matches!(
            local.as_str(),
            "pre" | "code" | "textarea" | "script" | "style"
        ) {
            preserve = true;
        }

        // 压缩 class 属性中的空白（包括换行），例如：
        // class="foo
        //   bar   baz" -> class="foo bar baz"
        let mut attrs_mut = attrs.borrow_mut();
        for attr in attrs_mut.iter_mut() {
            if attr.name.local.as_ref().eq_ignore_ascii_case("class") {
                let v = attr.value.to_string();
                let v = collapse_whitespace(&v);
                attr.value = v.into();
            }
        }
    }

    // 压缩文本节点空白
    if let NodeData::Text { ref contents } = node.data {
        if !preserve {
            let orig = contents.borrow().to_string();
            let collapsed = collapse_whitespace(&orig);
            *contents.borrow_mut() = collapsed.into();
        }
    }

    // 递归处理子节点
    for child in node.children.borrow().iter() {
        minify_node(child, preserve);
    }
}

fn collapse_whitespace(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_ws = false;

    for ch in s.chars() {
        if ch.is_whitespace() {
            if !in_ws {
                // 将任意一串空白压缩为单个空格
                out.push(' ');
                in_ws = true;
            }
        } else {
            in_ws = false;
            out.push(ch);
        }
    }

    out.trim().to_string()
}

