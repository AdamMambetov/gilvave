use sycamore::{futures::spawn_local_scoped, prelude::*};

use crate::{components::common::ServerContext, http::api::Api};

use super::member_item::MemberItem;

#[component()]
pub fn MembersPanel() -> View {
    let server_context = use_context::<ServerContext>();

    create_effect(move || {
        if let Some(server) = server_context.current.get_clone() {
            spawn_local_scoped(async move {
                if let Ok(members) = Api::get_members(server.id).await {
                    server_context.members.set(members);
                }
            })
        } else {
            server_context.members.set(vec![]);
        }
    });

    view! {
        div(class="discord-members-panel") {
            div(class="panel-section") {
                h3 { "Онлайн" }
                Indexed(
                    list=server_context.members,
                    view=|member| { view! { MemberItem(member_view=member) } },
                )
            }

            div(class="panel-section") {
                h3 { "Офлайн" }
            }
        }
    }
}
