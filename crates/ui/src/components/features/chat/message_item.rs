use crate::utils::to_local_datetime;
use gilvave_core::dto::message::MessageView;
use sycamore::prelude::*;
use time::format_description::parse_borrowed;

#[component(inline_props)]
pub fn MessageItem(
    message_view: MessageView,
    avatar_url: String,
    is_first_in_group: bool,
    is_last_in_group: bool,
) -> View {
    let format = parse_borrowed::<3>("[hour]:[minute]").unwrap();
    let local_dt = to_local_datetime(message_view.created_at);
    let formatted_timestamp = local_dt.format(&format).unwrap();
    let first_char = message_view
        .author_name
        .chars()
        .next()
        .unwrap_or('?')
        .to_uppercase()
        .to_string();

    let mut class_list = String::from("message");
    if !is_first_in_group {
        class_list.push_str(" chained");
    }
    if is_first_in_group {
        class_list.push_str(" group-first");
    }
    if is_last_in_group {
        class_list.push_str(" group-last");
    }

    let has_avatar = !avatar_url.is_empty();

    let avatar_node = if is_first_in_group {
        if has_avatar {
            view! {
                div(class="message-avatar") {
                    img(class="message-avatar-img", src=avatar_url, alt="")
                }
            }
        } else {
            view! {
                div(class="message-avatar") {
                    span { (first_char) }
                }
            }
        }
    } else {
        view! {
            div(class="message-avatar-spacer")
        }
    };

    let header_node = if is_first_in_group {
        let ts = formatted_timestamp.clone();
        view! {
            div(class="message-header") {
                span(class="author") { (message_view.author_name) }
                span(class="timestamp") { (ts) }
            }
        }
    } else {
        view! {}
    };

    let chained_node = if !is_first_in_group {
        let ts = formatted_timestamp.clone();
        view! {
            span(class="chained-time") { (ts) }
        }
    } else {
        view! {}
    };

    view! {
        div(class=class_list) {
            (avatar_node)
            div(class="message-body") {
                (header_node)
                div(class="message-bubble") {
                    p { (message_view.content) }
                    (chained_node)
                }
            }
        }
    }
}
