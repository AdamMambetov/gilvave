use gilvave_core::ids::ServerId;
use gilvave_core::{
    dto::{
        command::{CommandArgs, CommandResponse, CommandResult},
        server::{Server, ServerCreateInfo, ServerSmallPart},
    },
    ids::UserId,
};
use sycamore::{futures::spawn_local_scoped, prelude::*};

use crate::components::common::{CreateServerContext, ModalView, ServerContext};
use crate::{
    components::common::{ActiveScreen, ChannelContext, ScreenWrapper, classes},
    utils::invoke_command,
};

fn make_server_card(server: Server, expanded_id: Signal<Option<ServerId>>) -> View {
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
        let d = server.description.clone();
        let truncated: String = d.chars().take(60).collect();
        if truncated.len() < d.len() {
            format!("{}...", truncated)
        } else {
            truncated
        }
    };

    let first_char_c = first_char.clone();

    let toggle_expand = move |_| {
        let current = expanded_id.get();
        if current == Some(id) {
            expanded_id.set(None);
        } else {
            expanded_id.set(Some(id));
        }
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
                img(src=server.icon_url, alt="")
            }
            div(
                class=classes(vec![
                    "card-collapsed-overlay".into(),
                    ("hidden", { expanded_id.get() == Some(id) }.into()).into(),
                ]),
            ) {
                div(class="card-icon-bottom") {
                    span { (first_char) }
                }
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
                div(class="card-icon-centered") {
                    span { (first_char_c) }
                }
                div(class="card-server-name") { (name) }
                div(class="card-description") { (server.description) }
                div(class="card-members") {
                    svg(
                        xmlns="http://www.w3.org/2000/svg",
                        width="14",
                        height="14",
                        viewBox="0 0 24 24",
                        fill="none",
                        stroke="currentColor",
                        stroke-width="2",
                        stroke-linecap="round",
                        stroke-linejoin="round",
                    ) {
                        path(d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2")
                        circle(cx="9", cy="7", r="4")
                        path(d="M23 21v-2a4 4 0 0 0-3-3.87")
                        path(d="M16 3.13a4 4 0 0 1 0 7.75")
                    }
                    span { (format!("{} участников", server.members_count)) }
                }
                div(class="card-join-row") {
                    button(class="card-join-btn") { "Присоединиться" }
                }
            }
        }
    }
}

#[component(inline_props)]
fn JoinServerModal(
    public_servers: Signal<Vec<Server>>,
    expanded_id: Signal<Option<ServerId>>,
    is_visible: ReadSignal<bool>,
    on_back: impl Fn(web_sys::MouseEvent) + 'static,
    on_close: impl Fn(web_sys::MouseEvent) + 'static,
) -> View {
    view! {
        div(
            class=classes(vec![
                "server-modal-overlay".into(),
                "large".into(),
                ("hidden", { is_visible.map(|v| !v) }.into()).into(),
            ]),
            on:click=on_close,
        ) {
            div(
                class="server-modal join-modal",
                on:click=move |e: web_sys::MouseEvent| e.stop_propagation(),
            ) {
                div(class="server-modal-header") {
                    span { "Присоединиться к серверу" }
                    div(class="join-modal-search") {
                        input(
                            r#type="text",
                            placeholder="Поиск серверов...",
                        )
                    }
                }
                div(class="join-modal-body") {
                    div(class="join-modal-grid") {
                        Indexed(
                            list=public_servers,
                            view=move |server| {
                                make_server_card(server, expanded_id)
                            },
                        )
                    }
                }
                div(class="join-modal-footer") {
                    button(class="back-btn", on:click=on_back) { "← Назад" }
                }
            }
        }
    }
}

#[component(inline_props)]
pub fn ServerSidebar() -> View {
    let screen_wrapper = use_context::<ScreenWrapper>();

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
        screen_wrapper.set(ActiveScreen::Login);
        screen_wrapper.set(ActiveScreen::Home);
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
                on_create_server(server_context.list, info);
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
            div(
                class="server-icon",
                on:click=handle_home_click,
            ) { "🏠" }
            div(class="separator")
            Indexed(
                list=server_context.list,
                view=|server| {
                    let server_name = server.name;
                    view! {
                        div(
                            class="server-icon",
                            on:click=move |_| on_click_server(server.id),
                        ) { (server_name) }
                    }
                },
            )
            div(
                class="server-icon new",
                on:click=on_plus_click,
            ) { "+" }
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
            public_servers=context.public_servers,
            expanded_id=context.expanded_id,
            is_visible=is_join_visible,
            on_back=back_to_home,
            on_close=close,
        )
    }
}

fn on_click_server(server_id: ServerId) {
    let context = use_context::<ServerContext>();
    if let Some(server) = context.current.get_clone()
        && server.id == server_id
    {
        return;
    }

    spawn_local_scoped(async move {
        let args = CommandArgs::GetServerById {
            server_id: server_id.clone(),
        }
        .to_json();
        let res = invoke_command(args).await;

        context.members.set(vec![]);
        if let CommandResult::Ok(CommandResponse::GetServerById(server)) = res {
            context.current.set(Some(server));
        }
    });
    spawn_local_scoped(async move {
        let args = CommandArgs::GetMembers {
            server_id: server_id.clone(),
        }
        .to_json();
        let res = invoke_command(args).await;

        context.members.set(vec![]);
        if let CommandResult::Ok(CommandResponse::GetMembers(members)) = res {
            context.members.set(members);
        }
    });
}

fn on_create_server(server_list: Signal<Vec<ServerSmallPart>>, server_info: ServerCreateInfo) {
    spawn_local_scoped(async move {
        let args = CommandArgs::CreateServer { server_info }.to_json();
        let res = invoke_command(args).await;
        if let CommandResult::Ok(CommandResponse::CreateServer(server)) = res {
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
