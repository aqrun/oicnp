---
title: 'Dioxus 实现监听界面滚动事件'
description: 'Dioxus 使用 web-sys 实现监听界面滚动事件, 并获取滚动偏移量'

taxonomies:
  categories: ['diary']
  tags: ['diary', 'rust', 'dioxus', 'web-sys', 'scroll']
---

## 场景

需要监听 document 的 onScroll 事件，获取界面的滚动偏移量

## 代码实现

主要使用 web-sys 库实现和浏览器交互

```rust
use dioxus::prelude::*;
use web_sys::{console, window};
use wasm_bindgen::prelude::*;

pub fn SomeComponent(cx: Scope) -> Element {
    // 滚动事件处理
    let scroll_handler = use_ref(cx, move || Closure::<dyn FnMut(_)>::new(|e: web_sys::Event| {
        let win = window().unwrap();
        let document = win.document().unwrap();
        // 滚动偏移量算法参考 ahooks useScroll
        // https://github.com/alibaba/hooks/blob/master/packages/hooks/src/useScroll/index.ts#L26
        let page_y_offset: f64 = win.page_y_offset().unwrap_or(0.0);
        let document_scroll_left = document.document_element().unwrap().scroll_top() as f64;
        let body_scroll_top = document.body().unwrap().scroll_top() as f64;

        // 获取最大的
        let scroll_data = vec![
            page_y_offset,
            document_scroll_left,
            body_scroll_top,
        ];
        let scroll_top: &f64 = scroll_data.iter().reduce(|item, n| {
            if n > item {
                return n;
            }
            item
        }).unwrap();

        let a = format!("scroll_top: {:?}", scroll_top);
        console::log_1(&a.into());
    }));


    use_effect(cx, (), |()| {
        let win = window().unwrap();
        let document = win.document().unwrap();

        // 监听 scroll 事件
        document.add_event_listener_with_callback(
            "scroll",
            scroll_handler.read().as_ref().unchecked_ref()
        );

        // TODO: 实现 remove_event_listener
    });

    cx.render(rsx! {
        div {
            "界面相关显示省略"
        }
    })
}
```

## 参考

- [Rust wasm-bindgen Paint Example](https://rustwasm.github.io/wasm-bindgen/examples/paint.html)
- [ahooks useScroll](https://github.com/alibaba/hooks/blob/master/packages/hooks/src/useScroll/index.ts#L26)
- [web-sys crate](https://docs.rs/web-sys)
