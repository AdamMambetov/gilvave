use sycamore::prelude::*;

use crate::components::common::classes;

#[component(inline_props)]
pub fn ServerIcon(
    server_name: Signal<String>,
    icon_url: Signal<String>,
    #[prop(default = false.into())] is_active: MaybeDyn<bool>,
    #[prop(attributes(html, div))] attributes: Attributes,
) -> View {
    let class = classes(vec![
        "server-icon".into(),
        ("active", is_active).into(),
        ("new", { server_name.with(|s| s == "+") }.into()).into(),
    ]);

    view! {
        div(class=class, ..attributes) {
            (if icon_url.with(|s| s.is_empty()) {
                let first_char = server_name.with(|s| {
                    s.chars().next().unwrap_or('?').to_uppercase().to_string()
                });
                view! { span { (first_char) } }
            } else {
                let icon_str = icon_url.get_clone();
                view! { img(src=icon_str) }
            })
        }
    }
}
