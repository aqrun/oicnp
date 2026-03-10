# oic_html

基于 `html5ever` 的轻量级 HTML 压缩库，主要用于在服务端对模板渲染后的 HTML 做简单的体积优化。

## 特性

- 使用 `html5ever + RcDom` 解析完整 HTML DOM。
- 对普通文本节点压缩空白：
  - 连续空白字符合并为单个空格。
  - 去除首尾多余空白。
- 在以下标签内保留原始空白，不做处理：
  - `<pre>`, `<code>`, `<textarea>`, `<script>`, `<style>`
- 重新序列化为紧凑 HTML，不做 pretty-print。

## 使用示例

use oic_html::minify_html;

let raw_html = r#"
  <div>
    <span> hello   world </span>
  </div>
"#;

let minified = minify_html(raw_html);
// => 结构保持不变，空白被压缩
