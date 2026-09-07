use gilvave_core::{
    dto::{
        command::{CommandArgs, CommandResponse, CommandResult},
        server::{ServerCreateInfo, ServerSmallPart},
    },
    ids::ServerId,
};
use sycamore::{futures::spawn_local_scoped, prelude::*};

use crate::{components::common::ServerContext, utils::invoke_command};

pub(super) fn select_server(server_id: ServerId) {
    let context = use_context::<ServerContext>();
    if let Some(server) = context.current.get_clone()
        && server.id == server_id
    {
        return;
    }

    spawn_local_scoped(async move {
        let response = invoke_command(
            CommandArgs::GetServerById {
                server_id: server_id.clone(),
            }
            .to_json(),
        )
        .await;
        context.members.set(vec![]);
        if let CommandResult::Ok(CommandResponse::GetServerById(server)) = response {
            context.current.set(Some(server));
        }
    });
    spawn_local_scoped(async move {
        let response = invoke_command(CommandArgs::GetMembers { server_id }.to_json()).await;
        context.members.set(vec![]);
        if let CommandResult::Ok(CommandResponse::GetMembers(members)) = response {
            context.members.set(members);
        }
    });
}

pub(super) fn create_server(
    server_list: Signal<Vec<ServerSmallPart>>,
    server_info: ServerCreateInfo,
) {
    spawn_local_scoped(async move {
        let response = invoke_command(CommandArgs::CreateServer { server_info }.to_json()).await;
        if let CommandResult::Ok(CommandResponse::CreateServer(server)) = response {
            server_list.update(|list| {
                list.push(ServerSmallPart {
                    id: server.id,
                    name: server.name,
                    icon_url: server.icon_url,
                })
            });
        }
    });
}
