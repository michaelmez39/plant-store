use maud::{Markup, html};

use crate::components::page_wrapper;

pub async fn shopping() -> Markup {
    page_wrapper(page().await, false).await
}

async fn page() -> Markup {
    html!{
        .container {
            .level.mb-1 {
                .level-left {
                    h2.title.is-size-4 { "Shopping Cart" }
                }
                .level-right {
                    button.button.is-warning { "Remove All"}
                }
            }
            .columns {
                .column.is-two-thirds {
                    h3.is-size-5 { "Your Cart" }
                    .box {
                        "Item in your cart"
                        .remove {}
                    }
                }
                .column {
                    h3.is-size-5 { "Payment Info" }
                    .box {
                        "Card Info"
                        (payment_form().await)
                    }
                }
            }
        }
    }
}

async fn text_field(label: &'static str, help: Option<&'static str>) -> Markup {
    html! {
        .field {
            label.label { (label) }
            .control {
                input.input type="text";
            }
            @if let Some(help) = help {
                p.help { (help) }
            }
        }
    }
}

async fn payment_form() -> Markup {
    html! {
        form {
            (text_field("Name", None).await)
            (text_field("Billing Address", None).await)
            .field {
                .control {
                    button.button.is-link { "Review Order" }
                }
            }
        }
    }
}
