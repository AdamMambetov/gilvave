use sycamore::prelude::*;

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
    condition: &MaybeDyn<bool>,
    if_class: &str,
    else_class: &str,
) -> String {
    format!(
        "{class} {}",
        if condition.get() {
            if_class
        } else {
            else_class
        }
    )
}

/// Правило добавления класса
pub enum ClassRule {
    /// Всегда включать этот класс
    Always(&'static str),
    /// Включать класс только если условие истинно
    IfTrue(&'static str, MaybeDyn<bool>),
    /// Включать один из двух классов в зависимости от условия
    Ternary(MaybeDyn<bool>, &'static str, &'static str),
}

// Эргономичные конверсии
impl From<&'static str> for ClassRule {
    fn from(class: &'static str) -> Self {
        ClassRule::Always(class)
    }
}

impl From<(&'static str, MaybeDyn<bool>)> for ClassRule {
    fn from((class, cond): (&'static str, MaybeDyn<bool>)) -> Self {
        ClassRule::IfTrue(class, cond)
    }
}

/// Создаёт реактивное замыкание для генерации классов.
/// Возвращает `impl Fn() -> String`, который Sycamore автоматически конвертирует в `StringAttribute`.
pub fn classes(rules: Vec<ClassRule>) -> impl Fn() -> String {
    move || {
        rules
            .iter()
            .filter_map(|rule| match rule {
                ClassRule::Always(class) => Some(*class),
                ClassRule::IfTrue(class, cond) => {
                    if cond.get() {
                        Some(*class)
                    } else {
                        None
                    }
                }
                ClassRule::Ternary(cond, true_class, false_class) => {
                    if cond.get() {
                        Some(*true_class)
                    } else {
                        Some(*false_class)
                    }
                }
            })
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    }
}
