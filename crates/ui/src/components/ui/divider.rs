use sycamore::prelude::*;

#[component(inline_props)]
pub fn Divider(#[prop(attributes(html, div))] attributes: Attributes, children: Children) -> View {
    view! {
        div(class="divider", ..attributes) {
            span { (children) }
        }
    }
}
