use sycamore::prelude::*;

use crate::components::common::classes;

#[component(inline_props)]
pub fn ServerIcon(
    server_name: Signal<String>,
    icon_url: Signal<String>,
    #[prop(attributes(html, div))] attributes: Attributes,
) -> View {
    let class = classes(vec![
        "server-icon".into(),
        ("new", { server_name.get_clone() == "+" }.into()).into(),
    ]);

    view! {
        div(class=class, ..attributes) {
            ({
                let icon_str = icon_url.get_clone();
                let first_char = server_name
                    .get_clone()
                    .chars()
                    .next()
                    .unwrap_or('?')
                    .to_uppercase()
                    .to_string();
                if icon_str.is_empty() {
                    view! { span { (first_char) } }
                } else {
                    view! { img(src=icon_str) }
                }
            })
        }
    }
}
