use gilvave_core::{
    dto::{
        command::{CommandArgs, CommandResponse, CommandResult},
        server::Server,
    },
    ids::ServerId,
};
use sycamore::{futures::spawn_local_scoped, prelude::*};

use crate::{
    components::common::{CreateServerContext, ServerContext, classes},
    utils::invoke_command,
};

pub(super) fn server_card(server: Server, expanded_id: Signal<Option<ServerId>>) -> View {
    let id = server.id;
    let name = server.name.clone();
    let name_sm = server.name.clone();
    let first_char = server
        .name
        .chars()
        .next()
        .unwrap_or('?')
        .to_uppercase()
        .to_string();
    let desc_short = {
        let description = server.description.clone();
        let truncated: String = description.chars().take(60).collect();
        if truncated.len() < description.len() {
            format!("{truncated}...")
        } else {
            truncated
        }
    };

    let expanded_icon = first_char.clone();
    let toggle_expand = move |_| {
        expanded_id.set((expanded_id.get() != Some(id)).then_some(id));
    };
    let join_server = move |_| {
        spawn_local_scoped(async move {
            let args = CommandArgs::JoinPublicServer {
                server_id: server.id,
            }
            .to_json();
            let res = invoke_command(args).await;
            if let CommandResult::Ok(CommandResponse::JoinPublicServer(server)) = res {
                let context = use_context::<ServerContext>();
                context.current.set(Some(server));
                let context = use_context::<CreateServerContext>();
                context.is_modal_open.set(false);
            }
        });
    };
    let card_icon = if server.icon_url.is_empty() {
        view! { span { (first_char) } }
    } else {
        view! { img(src=server.icon_url, width="50", height="50") }
    };

    view! {
        div(
            class=classes(vec![
                "server-browser-card".into(),
                ("expanded", { expanded_id.get() == Some(id) }.into()).into(),
            ]),
            on:click=toggle_expand,
        ) {
            div(class="card-cover") {
                img(src=server.cover, alt="")
            }
            div(
                class=classes(vec![
                    "card-collapsed-overlay".into(),
                    ("hidden", { expanded_id.get() == Some(id) }.into()).into(),
                ]),
            ) {
                div(class="card-icon-bottom") { (card_icon) }
                div(class="card-collapsed-text") {
                    div(class="card-server-name-sm") { (name_sm) }
                    div(class="card-desc-short") { (desc_short) }
                }
            }
            div(
                class=classes(vec![
                    "card-expanded-body".into(),
                    ("hidden", { expanded_id.get() != Some(id) }.into()).into(),
                ]),
            ) {
                div(class="card-icon-centered") { span { (expanded_icon) } }
                div(class="card-server-name") { (name) }
                div(class="card-description") { (server.description) }
                div(class="card-members") {
                    svg(
                        xmlns="http://www.w3.org/2000/svg", width="14", height="14",
                        viewBox="0 0 24 24", fill="none", stroke="currentColor",
                        stroke-width="2", stroke-linecap="round", stroke-linejoin="round",
                    ) {
                        path(d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2")
                        circle(cx="9", cy="7", r="4")
                        path(d="M23 21v-2a4 4 0 0 0-3-3.87")
                        path(d="M16 3.13a4 4 0 0 1 0 7.75")
                    }
                    span { (format!("{} участников", server.members_count)) }
                }
                div(class="card-join-row") {
                    button(class="card-join-btn", on:click=join_server) { "Присоединиться" }
                }
            }
        }
    }
}
