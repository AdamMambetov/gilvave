use gilvave_core::dto::message::MessageView;
use sycamore::prelude::*;
use time::format_description::parse;

#[component(inline_props)]
pub fn MessageItem(message_view: MessageView) -> View {
    let format = parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
    let formatted_timestamp = message_view.created_at.format(&format).unwrap();

    view! {
        div(class="message") {
            div(class="message-header") {
                span(class="author") { (message_view.author_name) }
                span(class="timestamp") { (formatted_timestamp) }
            }
            p { (message_view.content) }
        }
    }
}
