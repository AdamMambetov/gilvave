use gilvave_core::{
    dto::{channel::ChannelView, message::MessageView, server::MemberView},
    ids::ChannelId,
};
use sycamore::prelude::*;

#[derive(Clone)]
pub struct MemberContext(pub Signal<Vec<MemberView>>);

#[derive(Clone)]
pub struct ChannelContext {
    pub text: Signal<Vec<ChannelView>>,
    pub voice: Signal<Vec<ChannelView>>,
    pub current: Signal<Option<ChannelId>>,
    pub messages: Signal<Vec<MessageView>>,
}
