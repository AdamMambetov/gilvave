use gilvave_core::{
    dto::{
        command::{CommandArgs, CommandResponse, CommandResult},
        server::{Server, ServerCreateInfo},
    },
    ids::{ServerId, UserId},
};
use sycamore::{futures::spawn_local_scoped, prelude::*};

use crate::components::{
    common::{CreateServerContext, ModalView, ServerContext},
    ui::icons::{ServerIcon, ServerIconCustom},
};
use crate::{
    components::common::{ChannelContext, classes},
    utils::invoke_command,
};

use super::{
    join_server_modal::JoinServerModal,
    server_actions::{create_server, select_server},
};

#[component(inline_props)]
pub fn ServerSidebar() -> View {
    let context = CreateServerContext {
        is_modal_open: create_signal(false),
        modal_view: create_signal(ModalView::Home),
        server_name: create_signal(String::new()),
        is_public: create_signal(true),
        public_servers: create_signal::<Vec<Server>>(vec![]),
        expanded_id: create_signal::<Option<ServerId>>(None),
    };
    provide_context(context.clone());

    let server_context = use_context::<ServerContext>();

    let handle_home_click = move |_| {
        spawn_local_scoped(async {
            let context = use_context::<ChannelContext>();
            if context.current_id.get().is_some() {
                let args = CommandArgs::LeftChannel {
                    channel_id: context.current_id.get().unwrap(),
                }
                .to_json();
                context.current_id.set(None);
                invoke_command(args).await;
            }
        });
        let context = use_context::<ServerContext>();
        context.current.set(None);
    };

    let on_plus_click = move |_| {
        let context = use_context::<CreateServerContext>();
        context.modal_view.set(ModalView::Home);
        context.is_modal_open.set(true);
    };

    let close = move |_| {
        let context = use_context::<CreateServerContext>();
        context.is_modal_open.set(false);
        context.modal_view.set(ModalView::Home);
        context.server_name.set(String::new());
        context.public_servers.set(vec![]);
    };

    let open_create = move |_| {
        let context = use_context::<CreateServerContext>();
        context.modal_view.set(ModalView::Create);
    };

    let open_join = move |_| {
        let context = use_context::<CreateServerContext>();
        context.modal_view.set(ModalView::Join);
        context.expanded_id.set(None);
        let epoch = time::OffsetDateTime::from_unix_timestamp(0).unwrap();
        let hardcoded = vec![
            Server {
                id: ServerId::try_from("550e8400-e29b-41d4-a716-446655440001").unwrap(),
                name: "Rust Developers".to_string(),
                icon_url: "".to_string(),
                created_at: epoch,
                description: "Сообщество разработчиков на Rust. Обсуждаем код, делимся проектами и помогаем новичкам.".to_string(),
                members_count: 1247,
                cover: "https://images.unsplash.com/photo-1515879218367-8466d910auj4?w=600".to_string(),
                is_public: true,
                owner_id: UserId::default(),
            },
            Server {
                id: ServerId::try_from("550e8400-e29b-41d4-a716-446655440002").unwrap(),
                name: "Гейм-дев".to_string(),
                icon_url: "".to_string(),
                created_at: epoch,
                description: "Разработка игр на всех движках. Unity, Unreal, Godot — всё обсуждаем здесь.".to_string(),
                members_count: 834,
                cover: "https://images.unsplash.com/photo-1511512578047-dfb367046420?w=600".to_string(),
                is_public: true,
                owner_id: UserId::default(),
            },
            Server {
                id: ServerId::try_from("550e8400-e29b-41d4-a716-446655440003").unwrap(),
                name: "Музыка".to_string(),
                icon_url: "".to_string(),
                created_at: epoch,
                description: "Делимся музыкой, обсуждаем альбомы и находим единомышленников по вкусам.".to_string(),
                members_count: 2103,
                cover: "https://images.unsplash.com/photo-1511379938547-c1f69419868d?w=600".to_string(),
                is_public: true,
                owner_id: UserId::default(),
            },
            Server {
                id: ServerId::try_from("550e8400-e29b-41d4-a716-446655440004").unwrap(),
                name: "Аниме клуб".to_string(),
                icon_url: "".to_string(),
                created_at: epoch,
                description: "Обсуждаем аниме, мангу и всё связанное с японской культурой. Новости, обзоры, рекомендации.".to_string(),
                members_count: 3562,
                cover: "https://images.unsplash.com/photo-1578632767115-351597cf2477?w=600".to_string(),
                is_public: true,
                owner_id: UserId::default(),
            },
            Server {
                id: ServerId::try_from("550e8400-e29b-41d4-a716-446655440005").unwrap(),
                name: "Memes & Chill".to_string(),
                icon_url: "".to_string(),
                created_at: epoch,
                description: "Лучшие мемы, тёплая атмосфера и 좋은 настроение каждый день.".to_string(),
                members_count: 5891,
                cover: "https://images.unsplash.com/photo-1533738363-b7f9aef128ce?w=600".to_string(),
                is_public: true,
                owner_id: UserId::default(),
            },
        ];
        context.public_servers.set(hardcoded);
        spawn_local_scoped(async move {
            let args = CommandArgs::GetPublicServers { page: 1 }.to_json();
            let res = invoke_command(args).await;
            if let CommandResult::Ok(CommandResponse::GetPublicServers((servers, _has_more))) = res
            {
                context.public_servers.set(servers);
            }
        });
    };

    let handle_create = move |_| {
        let context = use_context::<CreateServerContext>();
        let name = context.server_name.with(|v| v.clone());
        if name.trim().is_empty() {
            return;
        }
        let info = ServerCreateInfo {
            name: name,
            is_public: context.is_public.get(),
        };
        spawn_local_scoped(async move {
            let args = CommandArgs::CreateServer {
                server_info: info.clone(),
            }
            .to_json();
            let res = invoke_command(args).await;
            if let CommandResult::Ok(CommandResponse::CreateServer(_server)) = res {
                create_server(server_context.list, info);
            }
        });
        context.is_modal_open.set(false);
        context.modal_view.set(ModalView::Home);
        context.server_name.set(String::new());
    };

    let back_to_home = move |_| {
        let context = use_context::<CreateServerContext>();
        context.modal_view.set(ModalView::Home);
    };

    let is_home_visible = create_memo(move || {
        let context = use_context::<CreateServerContext>();
        context.is_modal_open.get() && context.modal_view.get() == ModalView::Home
    });
    let is_create_visible = create_memo(move || {
        let context = use_context::<CreateServerContext>();
        context.is_modal_open.get() && context.modal_view.get() == ModalView::Create
    });
    let is_join_visible = create_memo(move || {
        let context = use_context::<CreateServerContext>();
        context.is_modal_open.get() && context.modal_view.get() == ModalView::Join
    });

    view! {
        div(class="discord-sidebar") {
            ServerIconCustom(
                icon="🏠".into(),
                on:click=handle_home_click,
            )
            div(class="separator")
            Indexed(
                list=server_context.list,
                view=|server| {
                    let server_id = server.id;
                    view! {
                        ServerIcon(
                            server=server,
                            on:click=move |_| select_server(server_id),
                        )
                    }
                },
            )
            ServerIconCustom(
                icon="+".into(),
                on:click=on_plus_click,
            )
        }

        div(
            class=classes(vec![
                "server-modal-overlay".into(),
                ("hidden", { !is_home_visible.get() }.into()).into(),
            ]),
            on:click=close,
        ) {
            div(
                class="server-modal",
                on:click=move |e: web_sys::MouseEvent| e.stop_propagation(),
            ) {
                div(class="server-modal-header") {
                    span { "Серверы" }
                }
                div(class="server-modal-body") {
                    div(class="server-modal-card create", on:click=open_create) {
                        div(class="server-modal-icon create") {
                            svg(
                                xmlns="http://www.w3.org/2000/svg",
                                width="32",
                                height="32",
                                viewBox="0 0 24 24",
                                fill="none",
                                stroke="currentColor",
                                stroke-width="2",
                                stroke-linecap="round",
                                stroke-linejoin="round",
                            ) {
                                line(x1="12", y1="5", x2="12", y2="19")
                                line(x1="5", y1="12", x2="19", y2="12")
                            }
                        }
                        span(class="server-modal-label") { "Создать сервер" }
                    }
                    div(class="server-modal-card join", on:click=open_join) {
                        div(class="server-modal-icon join") {
                            svg(
                                xmlns="http://www.w3.org/2000/svg",
                                width="32",
                                height="32",
                                viewBox="0 0 24 24",
                                fill="none",
                                stroke="currentColor",
                                stroke-width="2",
                                stroke-linecap="round",
                                stroke-linejoin="round",
                            ) {
                                path(d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4")
                                polyline(points="10 17 15 12 10 7")
                                line(x1="15", y1="12", x2="3", y2="12")
                            }
                        }
                        span(class="server-modal-label") { "Присоединиться" }
                    }
                }
            }
        }

        div(class=classes(vec![
            "server-modal-overlay".into(),
            ("hidden", { !is_create_visible.get() }.into()).into(),
        ]), on:click=close) {
            div(
                class="server-modal create-form",
                on:click=move |e: web_sys::MouseEvent| e.stop_propagation(),
            ) {
                div(class="server-modal-header") {
                    span { "Создать сервер" }
                }
                div(class="create-form-body") {
                    div(class="input-group") {
                        label { "Название сервера" }
                        input(
                            r#type="text",
                            placeholder="Мой сервер",
                            bind:value=context.server_name,
                        )
                    }
                    div(class="create-form-tabs") {
                        div(class="toggle-wrapper") {
                            div(class="toggle-tabs") {
                                div(
                                    class=classes(vec![
                                        "toggle-tab".into(),
                                        ("active", { !context.is_public.get() }.into()).into(),
                                    ]),
                                    on:click=move |_| context.is_public.set(false),
                                ) { "🔐 Приватный" }
                                div(
                                    class=classes(vec![
                                        "toggle-tab".into(),
                                        ("active", { context.is_public.get() }.into()).into(),
                                    ]),
                                    on:click=move |_| context.is_public.set(true),
                                ) { "🌍 Публичный" }
                                div(
                                    class=classes(vec![
                                        "floating-bg".into(),
                                        ("public", { context.is_public.get() }.into()).into(),
                                    ]),
                                )
                            }
                        }
                        span(class="checkbox-hint") {
                            (if context.is_public.get() {
                                "🌍 Публичные серверы видны всем пользователям"
                            } else {
                                "🔐 Только приглашённые пользователи"
                            })
                        }
                    }
                    div(class="create-form-actions") {
                        button(
                            class="submit-btn create-submit",
                            on:click=handle_create,
                        ) { "Создать" }
                    }
                }
            }
        }

        JoinServerModal(
            is_visible=is_join_visible,
            on_back=back_to_home,
            on_close=close,
        )
    }
}
