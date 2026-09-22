use gilvave_core::{
    dto::{
        command::{CommandArgs, CommandResponse, CommandResult},
        server::{ServerCreateInfo, ServerSmallPart},
    },
    ids::ServerId,
};
use sycamore::{futures::spawn_local_scoped, prelude::*};

use crate::{
    components::common::{ChannelContext, ServerContext},
    utils::invoke_command,
};

pub(super) fn select_server(server_id: ServerId) {
    let context = use_context::<ServerContext>();
    let ch_ctx = use_context::<ChannelContext>();
    if let Some(server) = context.current.get_clone()
        && server.id == server_id
    {
        ch_ctx.current.set(None);
        return;
    }

    ch_ctx.current.set(None);
    ch_ctx.messages.set(vec![]);

    if let Some(small) = context.list.with(|l| l.iter().find(|s| s.id == server_id).cloned()) {
        let epoch = time::OffsetDateTime::from_unix_timestamp(0).unwrap();
        let current_srv = gilvave_core::dto::server::Server {
            id: small.id,
            owner_id: gilvave_core::ids::UserId::default(),
            name: small.name,
            description: String::new(),
            icon_url: small.icon_url,
            cover: String::new(),
            is_public: true,
            members_count: 1,
            created_at: epoch,
        };
        context.current.set(Some(current_srv));
    }

    context.members.set(vec![]);

    spawn_local_scoped(async move {
        let response = invoke_command(
            CommandArgs::GetServerById {
                server_id: server_id.clone(),
            }
            .to_json(),
        )
        .await;
        if let CommandResult::Ok(CommandResponse::GetServerById(server)) = response {
            context.current.set(Some(server));
        }
    });
    spawn_local_scoped(async move {
        let response = invoke_command(CommandArgs::GetMembers { server_id }.to_json()).await;
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
