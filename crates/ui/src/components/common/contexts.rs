use gilvave_core::{
    dto::{
        channel::ChannelView,
        message::MessageView,
        server::{MemberView, Server, ServerSmallPart},
    },
    ids::{ChannelId, ServerId},
};
use sycamore::prelude::*;

#[derive(Clone)]
pub struct ChannelContext {
    pub text: Signal<Vec<ChannelView>>,
    pub voice: Signal<Vec<ChannelView>>,
    pub current_id: Signal<Option<ChannelId>>,
    pub messages: Signal<Vec<MessageView>>,
}

#[derive(Clone)]
pub struct ServerContext {
    pub current: Signal<Option<Server>>,
    pub list: Signal<Vec<ServerSmallPart>>,
    pub members: Signal<Vec<MemberView>>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ModalView {
    Home,
    Create,
    Join,
}

#[derive(Clone)]
pub struct CreateServerContext {
    pub is_modal_open: Signal<bool>,
    pub modal_view: Signal<ModalView>,
    pub server_name: Signal<String>,
    pub is_public: Signal<bool>,
    pub public_servers: Signal<Vec<Server>>,
    pub expanded_id: Signal<Option<ServerId>>,
}
