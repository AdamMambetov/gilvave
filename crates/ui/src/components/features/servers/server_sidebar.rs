use std::rc::Rc;

use gilvave_core::dto::{
    command::{CommandArgs, CommandResponse, CommandResult},
    server::{ServerCreateInfo, ServerView},
};
use gilvave_core::ids::ServerId;
use sycamore::{futures::spawn_local_scoped, prelude::*};

use crate::{
    components::common::{classes, ActiveScreen, ChannelContext, MemberContext, ScreenWrapper},
    utils::invoke_command,
};

#[derive(Clone, Copy, PartialEq)]
enum ModalView {
    Home,
    Create,
    Join,
}

fn make_server_card(server: ServerView, expanded_id: Signal<Option<ServerId>>) -> View {
    let sid = server.id;
    let sname = server.name.clone();
    let first_char = sname.chars().next().unwrap_or('?').to_uppercase().to_string();
    let sdesc_short = {
        let d = server.description.clone();
        let truncated: String = d.chars().take(60).collect();
        if truncated.len() < d.len() { format!("{}...", truncated) } else { truncated }
    };
    let sdesc_full = server.description.clone();
    let sicon = server.icon_url.clone();
    let smembers = server.member_count;

    let first_char_c = first_char.clone();
    let sname_c = sname.clone();

    let toggle_expand = move |_| {
        let current = expanded_id.get();
        if current == Some(sid) {
            expanded_id.set(None);
        } else {
            expanded_id.set(Some(sid));
        }
    };

    view! {
        div(
            class=classes(vec![
                "server-browser-card".into(),
                ("expanded", MaybeDyn::from(move || expanded_id.get() == Some(sid))).into(),
            ]),
            on:click=toggle_expand,
        ) {
            div(class="card-cover") {
                img(src=sicon, alt="")
            }
            div(class=classes(vec![
                "card-collapsed-overlay".into(),
                ("hidden", MaybeDyn::from(move || expanded_id.get() == Some(sid))).into(),
            ])) {
                div(class="card-icon-bottom") {
                    span { (first_char) }
                }
                div(class="card-collapsed-text") {
                    div(class="card-server-name-sm") { (sname) }
                    div(class="card-desc-short") { (sdesc_short) }
                }
            }
            div(class=classes(vec![
                "card-expanded-body".into(),
                ("hidden", MaybeDyn::from(move || expanded_id.get() != Some(sid))).into(),
            ])) {
                div(class="card-icon-centered") {
                    span { (first_char_c) }
                }
                div(class="card-server-name") { (sname_c) }
                div(class="card-description") { (sdesc_full) }
                div(class="card-members") {
                    svg(xmlns="http://www.w3.org/2000/svg", width="14", height="14", viewBox="0 0 24 24", fill="none", stroke="currentColor", stroke-width="2", stroke-linecap="round", stroke-linejoin="round") {
                        path(d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2")
                        circle(cx="9", cy="7", r="4")
                        path(d="M23 21v-2a4 4 0 0 0-3-3.87")
                        path(d="M16 3.13a4 4 0 0 1 0 7.75")
                    }
                    span { (format!("{} участников", smembers)) }
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
    public_servers: Signal<Vec<ServerView>>,
    expanded_id: Signal<Option<ServerId>>,
    is_visible: ReadSignal<bool>,
    on_back: impl Fn(web_sys::MouseEvent) + 'static,
    on_close: impl Fn(web_sys::MouseEvent) + 'static,
) -> View {
    view! {
        div(class=classes(vec![
            "server-modal-overlay".into(),
            "large".into(),
            ("hidden", MaybeDyn::Signal(is_visible.map(|v| !v))).into(),
        ]), on:click=on_close) {
            div(class="server-modal join-modal", on:click=move |e: web_sys::MouseEvent| e.stop_propagation()) {
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
pub fn ServerSidebar(
    server_list: Signal<Vec<ServerView>>,
    on_create_server: impl Fn() + 'static,
) -> View {
    let screen_wrapper = use_context::<ScreenWrapper>();
    let modal_view = create_signal(ModalView::Home);
    let is_modal_open = create_signal(false);
    let server_name = create_signal(String::new());
    let is_public = create_signal(true);
    let public_servers = create_signal::<Vec<ServerView>>(vec![]);
    let expanded_id = create_signal::<Option<ServerId>>(None);
    let on_create_server_rc: Rc<dyn Fn()> = Rc::new(on_create_server);

    let handle_home_click = move |_| {
        spawn_local_scoped(async {
            let context = use_context::<ChannelContext>();
            if context.current.get().is_some() {
                let args = CommandArgs::LeftChannel {
                    channel_id: context.current.get().unwrap(),
                }
                .to_json();
                context.current.set(None);
                invoke_command(args).await;
            }
        });
        screen_wrapper.set(ActiveScreen::Login);
        screen_wrapper.set(ActiveScreen::Home);
    };

    let on_plus_click = move |_| {
        modal_view.set(ModalView::Home);
        is_modal_open.set(true);
    };

    let sm = is_modal_open.clone();
    let mv = modal_view.clone();
    let sn = server_name.clone();
    let ps = public_servers.clone();
    let close = move |_| {
        sm.set(false);
        mv.set(ModalView::Home);
        sn.set(String::new());
        ps.set(vec![]);
    };

    let oc = modal_view.clone();
    let open_create = move |_| {
        oc.set(ModalView::Create);
    };

    let oj = modal_view.clone();
    let ps2 = public_servers.clone();
    let ej = expanded_id.clone();
    let open_join = move |_| {
        oj.set(ModalView::Join);
        ej.set(None);
        let epoch = time::OffsetDateTime::from_unix_timestamp(0).unwrap();
        let hardcoded = vec![
            ServerView {
                id: ServerId::try_from("550e8400-e29b-41d4-a716-446655440001").unwrap(),
                name: "Rust Developers".to_string(),
                icon_url: "https://images.unsplash.com/photo-1515879218367-8466d910auj4?w=600".to_string(),
                created_at: epoch,
                description: "Сообщество разработчиков на Rust. Обсуждаем код, делимся проектами и помогаем новичкам.".to_string(),
                member_count: 1247,
            },
            ServerView {
                id: ServerId::try_from("550e8400-e29b-41d4-a716-446655440002").unwrap(),
                name: "Гейм-дев".to_string(),
                icon_url: "https://images.unsplash.com/photo-1511512578047-dfb367046420?w=600".to_string(),
                created_at: epoch,
                description: "Разработка игр на всех движках. Unity, Unreal, Godot — всё обсуждаем здесь.".to_string(),
                member_count: 834,
            },
            ServerView {
                id: ServerId::try_from("550e8400-e29b-41d4-a716-446655440003").unwrap(),
                name: "Музыка".to_string(),
                icon_url: "https://images.unsplash.com/photo-1511379938547-c1f69419868d?w=600".to_string(),
                created_at: epoch,
                description: "Делимся музыкой, обсуждаем альбомы и находим единомышленников по вкусам.".to_string(),
                member_count: 2103,
            },
            ServerView {
                id: ServerId::try_from("550e8400-e29b-41d4-a716-446655440004").unwrap(),
                name: "Аниме клуб".to_string(),
                icon_url: "https://images.unsplash.com/photo-1578632767115-351597cf2477?w=600".to_string(),
                created_at: epoch,
                description: "Обсуждаем аниме, мангу и всё связанное с японской культурой. Новости, обзоры, рекомендации.".to_string(),
                member_count: 3562,
            },
            ServerView {
                id: ServerId::try_from("550e8400-e29b-41d4-a716-446655440005").unwrap(),
                name: "Memes & Chill".to_string(),
                icon_url: "https://images.unsplash.com/photo-1533738363-b7f9aef128ce?w=600".to_string(),
                created_at: epoch,
                description: "Лучшие мемы, тёплая атмосфера и 좋은 настроение каждый день.".to_string(),
                member_count: 5891,
            },
        ];
        ps2.set(hardcoded);
    };

    let cb = on_create_server_rc.clone();
    let sn_c = server_name.clone();
    let ip = is_public.clone();
    let sm_c = is_modal_open.clone();
    let mv_c = modal_view.clone();
    let sn_c2 = server_name.clone();
    let handle_create = move |_| {
        let name = sn_c.with(|v| v.clone());
        if name.trim().is_empty() {
            return;
        }
        let pub_flag = ip.get();
        let info = ServerCreateInfo {
            name,
            icon_url: None,
            is_public: pub_flag,
        };
        let cb2 = cb.clone();
        spawn_local_scoped(async move {
            let args = CommandArgs::CreateServer { server_info: info }.to_json();
            let res = invoke_command(args).await;
            if let CommandResult::Ok(CommandResponse::CreateServer(_server)) = res {
                cb2();
            }
        });
        sm_c.set(false);
        mv_c.set(ModalView::Home);
        sn_c2.set(String::new());
    };

    let back_to_home = move |_| {
        modal_view.set(ModalView::Home);
    };

    let is_home_visible = create_memo(move || is_modal_open.get() && modal_view.get() == ModalView::Home);
    let is_create_visible = create_memo(move || is_modal_open.get() && modal_view.get() == ModalView::Create);
    let is_join_visible = create_memo(move || is_modal_open.get() && modal_view.get() == ModalView::Join);

    view! {
        div(class="discord-sidebar") {
            div(class="server-icon", on:click=handle_home_click) { "🏠" }
            div(class="separator") {}
            Indexed(
                list=server_list,
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

        div(class=classes(vec![
            "server-modal-overlay".into(),
            ("hidden", MaybeDyn::from(move || !is_home_visible.get())).into(),
        ]), on:click=close) {
            div(class="server-modal", on:click=move |e: web_sys::MouseEvent| e.stop_propagation()) {
                div(class="server-modal-header") {
                    span { "Серверы" }
                }
                div(class="server-modal-body") {
                    div(class="server-modal-card create", on:click=open_create) {
                        div(class="server-modal-icon create") {
                            svg(xmlns="http://www.w3.org/2000/svg", width="32", height="32", viewBox="0 0 24 24", fill="none", stroke="currentColor", stroke-width="2", stroke-linecap="round", stroke-linejoin="round") {
                                line(x1="12", y1="5", x2="12", y2="19")
                                line(x1="5", y1="12", x2="19", y2="12")
                            }
                        }
                        span(class="server-modal-label") { "Создать сервер" }
                    }
                    div(class="server-modal-card join", on:click=open_join) {
                        div(class="server-modal-icon join") {
                            svg(xmlns="http://www.w3.org/2000/svg", width="32", height="32", viewBox="0 0 24 24", fill="none", stroke="currentColor", stroke-width="2", stroke-linecap="round", stroke-linejoin="round") {
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
            ("hidden", MaybeDyn::from(move || !is_create_visible.get())).into(),
        ]), on:click=close) {
            div(class="server-modal create-form", on:click=move |e: web_sys::MouseEvent| e.stop_propagation()) {
                div(class="server-modal-header") {
                    span { "Создать сервер" }
                }
                div(class="create-form-body") {
                    div(class="input-group") {
                        label { "Название сервера" }
                        input(
                            r#type="text",
                            placeholder="Мой сервер",
                            bind:value=server_name,
                        )
                    }
                    div(class="create-form-options") {
                        label(class="checkbox") {
                            input(
                                r#type="checkbox",
                                bind:checked=is_public,
                            )
                            span { "Публичный сервер" }
                        }
                        span(class="checkbox-hint") { "Публичные серверы видны всем пользователям" }
                    }
                    div(class="create-form-actions") {
                        button(class="submit-btn create-submit", on:click=handle_create) { "Создать" }
                    }
                }
            }
        }

        JoinServerModal(
            public_servers=public_servers,
            expanded_id=expanded_id,
            is_visible=is_join_visible,
            on_back=back_to_home,
            on_close=close,
        )
    }
}

fn on_click_server(server_id: gilvave_core::ids::ServerId) {
    spawn_local_scoped(async move {
        let context = use_context::<MemberContext>();
        let args = CommandArgs::GetMembers {
            server_id: server_id.clone(),
        }
        .to_json();
        let res = invoke_command(args).await;

        context.0.set(vec![]);
        if let CommandResult::Ok(CommandResponse::GetMembers(members)) = res {
            context.0.set(members);
        }
    });
    spawn_local_scoped(async move {
        let context = use_context::<ChannelContext>();
        if context.current.get().is_some() {
            let args = CommandArgs::LeftChannel {
                channel_id: context.current.get().unwrap(),
            }
            .to_json();
            context.current.set(None);
            invoke_command(args).await;
        }

        let args = CommandArgs::GetServerChannels {
            server_id: server_id.clone(),
        }
        .to_json();
        let res = invoke_command(args).await;

        context.text.set(vec![]);
        context.voice.set(vec![]);
        if let CommandResult::Ok(CommandResponse::GetServerChannels(channels)) = res {
            for channel in channels {
                match channel.r#type {
                    gilvave_core::dto::channel::ChannelType::TEXT => {
                        context.text.update(|list| list.push(channel))
                    }
                    gilvave_core::dto::channel::ChannelType::VOICE => {
                        context.voice.update(|list| list.push(channel))
                    }
                }
            }
        }
    })
}
