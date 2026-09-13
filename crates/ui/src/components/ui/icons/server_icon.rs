use gilvave_core::dto::server::ServerSmallPart;
use sycamore::prelude::*;

use crate::components::common::classes;

#[component(inline_props)]
pub fn ServerIcon(
    server: ServerSmallPart,
    #[prop(attributes(html, div))] attributes: Attributes,
) -> View {
    let first_char = server
        .name
        .chars()
        .next()
        .unwrap_or('?')
        .to_uppercase()
        .to_string();
    let icon = if server.icon_url.is_empty() {
        view! { span { (first_char) } }
    } else {
        view! { img(src=server.icon_url) }
    };

    view! {
        div(class="server-icon", ..attributes) {
            (icon)
        }
    }
}

#[component(inline_props)]
pub fn ServerIconCustom(
    icon: String,
    #[prop(attributes(html, div))] attributes: Attributes,
) -> View {
    let class = classes(vec![
        "server-icon".into(),
        ("new", { icon == "+" }.into()).into(),
    ]);

    view! {
        div(class=class, ..attributes) {
            span { (icon) }
        }
    }
}
