use crate::{components::page_wrapper, utils::display_decimal};

use maud::{html, Markup};
use store_lib::{Cart, CartItem, ItemListing};

pub async fn shopping() -> Markup {
    page_wrapper(page().await, false).await
}

async fn page() -> Markup {
    html! {
        .section {
            .container {
                .columns {
                    .column.is-two-thirds {
                        (cart().await)
                    }
                    .column {
                        (order_summary().await)
                    }
                }
            }
        }
    }
}

async fn cart() -> Markup {
    html! {
        .level.mb-5 {
            .level-left {
                h2.title.is-size-4 { "Shopping Cart" }
            }
            .level-right {
                button.button.is-warning.is-outlined { "Remove All"}
            }
        }
        (shopping_cart_item(ItemListing::random()).await)
        (shopping_cart_item(ItemListing::random()).await)
        (shopping_cart_item(ItemListing::random()).await)
        a.is-link.is-outlined href="/" { "Continue Shopping" }
    }
}

pub async fn order_summary() -> Markup {
    let cart = Cart {
        items: vec![
            CartItem {
                listing: ItemListing::random(),
                number: 3,
            },
            CartItem {
                listing: ItemListing::random(),
                number: 2,
            },
            CartItem {
                listing: ItemListing::random(),
                number: 1,
            },
        ],
    };

    html! {
        h2.is-size-4 { "Order Summary" }
            .box {
                .field.has-addons {
                    .control {
                        input.input "type"="text" placeholder="Promo Code" {}
                    }
                    .control {
                        button.button { "Apply Promo" }
                    }
                }
                hr;
                .level.is-mobile.is-size-5 {
                    .level-left {
                        (format!("Subtotal ( {} items)", cart.items.len()))
                    }
                    .level-right {
                        (display_decimal(cart.subtotal()))
                    }
                }
                a.button.is-fullwidth href="/checkout" {"Checkout"}
        }
    }
}

async fn shopping_cart_item(item: ItemListing) -> Markup {
    html! {
        .box {
            .media {
                .media-left {
                    .image.is-96x96.is-flex.is-align-items-center {
                        img src=(format!("/assets/images/{}", item.image));
                    }
                }
                .media-content.columns {
                    .column.is-third {
                        b { (item.name) }
                        br;
                        (item.description)
                    }
                    .column {
                        b { "Quantity" }
                        br;
                        .field.is-grouped {
                            button.button {"-"}
                            input.counter.input.shrink.has-text-centered value="1" type="text";
                            button.button {"+"}
                        }
                    }
                    .column {
                        b { "Price" }
                        br;
                        "$"(item.price)
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
