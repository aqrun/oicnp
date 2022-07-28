use yew::prelude::*;
use web_sys::{HtmlInputElement};
use crate::utils::log;

#[function_component(InputTest)]
pub fn input_test() -> Html {

    let name_change_handle = {
        Callback::from(move |e: Event| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            let val: String = input.value();
            // let array = js_sys::Array::new();
            // array.push(&format!("name: {}", val).into());
            // array.push(&"2342".into());
            // console::log(&array);
            log(&vec![&val, "43"]);
        })
    };

    html! {
        <div>
            <label>
                { "Name:" }
                <input
                    type="text"
                    name="name"
                    onchange={name_change_handle}
                />
            </label>
        </div>
    }
}
