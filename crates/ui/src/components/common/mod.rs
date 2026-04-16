use sycamore::prelude::Signal;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LoginMode(pub Signal<bool>);

impl LoginMode {
    pub fn is_login(&self) -> bool {
        self.0.get()
    }

    pub fn toggle(&self) {
        self.0.set(!self.0.get());
    }
}

pub fn class_name(
    class: &str,
    condition: &dyn Fn() -> bool,
    if_class: &str,
    else_class: &str,
) -> String {
    format!(
        "{class} {}",
        if condition() { if_class } else { else_class }
    )
}
