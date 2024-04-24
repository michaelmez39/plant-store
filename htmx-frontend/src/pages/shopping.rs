use maud::{html, Markup};

use crate::{components::page_wrapper, ItemListing};

pub async fn shopping() -> Markup {
    page_wrapper(page().await, false).await
}

async fn page() -> Markup {
    html! {
        .container {
            .columns {
                .column.is-two-thirds {
                    .level.mb-4 {
                        .level-left {
                            h2.title.is-size-4 { "Shopping Cart" }
                        }
                        .level-right {
                            button.button.is-warning { "Remove All"}
                        }
                    }
                    (shopping_cart_item(ItemListing::random()).await)
                    (shopping_cart_item(ItemListing::random()).await)
                    (shopping_cart_item(ItemListing::random()).await)
                    a.button.is-link href="/" { "Continue Shopping" }
                }
                .column {
                    h2.is-size-4 { "Order Summary" }
                    .box {
                        .level.is-mobile {
                            .level-left {"Items: 3"}
                            .level-right {"$67.39"}
                        }
                        (text_field("Shipping", None).await)
                        (text_field("Promo Code", None).await)
                        hr;
                        .level.is-mobile {
                            .level-left {"Total Cost:"}
                            .level-right {"$77.39"}
                        }
                        button.button {"Checkout"}
                    }
                }
            }
        }
    }
}

async fn shopping_cart_item(item: ItemListing) -> Markup {
    html! {
        .box {
            .media {
                .media-left {
                    .image.is-96x96.is-flex.is-align-items-center {
                        img src=(format!("http://localhost:3000/assets/{}", item.image));
                    }
                }
                .media-content.columns {
                    .column.is-third{
                        b { (item.name) }
                        br;
                        "From the mountains. Very healthy."
                    }
                    .column {
                        b { "Quantity" }
                        br;
                        .field.is-grouped {
                            button.button {"-"}
                            input.is-rounded.shrink type="text";
                            button.button {"+"}
                        }
                    }
                    .column {
                        b { "Price" }
                        br;
                        (item.price)
                    }
                }
                .media-right {
                    a {
                        span.mr-2 {"Remove"}
                        .delete {}
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
                    button.button.is-link { "Place Order" }
                }
            }
        }
    }
}
