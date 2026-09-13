use gilvave_core::dto::command::{CommandArgs, CommandResponse, CommandResult};
use sycamore::{futures::spawn_local_scoped, prelude::*};

use crate::{components::common::ServerContext, utils::invoke_command};

use super::member_item::MemberItem;

#[component(inline_props)]
pub fn MembersPanel() -> View {
    let server_context = use_context::<ServerContext>();

    create_effect(move || {
        if let Some(server) = server_context.current.get_clone() {
            spawn_local_scoped(async move {
                let args = CommandArgs::GetMembers {
                    server_id: server.id.clone(),
                }
                .to_json();
                let res = invoke_command(args).await;

                if let CommandResult::Ok(CommandResponse::GetMembers(members)) = res {
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
