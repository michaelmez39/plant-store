pub mod icons;
mod navbar;
mod notification;
mod page_wrapper;
pub mod text_field;

#[allow(dead_code)]
pub enum Color {
    Primary,
    Info,
    Link,
    Success,
    Warning,
    Danger,
    Default,
}

pub use crate::components::navbar::navbar;
pub use crate::components::notification::notification;
pub use crate::components::page_wrapper::PageWrapper;
pub use crate::components::text_field::text_field;
