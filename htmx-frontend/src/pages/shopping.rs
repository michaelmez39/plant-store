use std::collections::HashMap;

use crate::{components::PageWrapper, utils::display_decimal};

use maud::{html, Markup};
use store_lib::{
    cart::{Cart, CartItem},
    store::Product,
};
use uuid::Uuid;

pub async fn shopping(page: PageWrapper) -> Markup {
    page.render(order_page().await)
}

async fn order_page() -> Markup {
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
        .level {
            .level-left {
                h2.title.is-3 { "Shopping Cart" }
            }
            .level-right {
                button.button.is-warning.is-outlined { "Remove All"}
            }
        }
        a.is-link.is-outlined href="/" { "Continue Shopping" }
        (shopping_cart_item(Product::random()).await)
        (shopping_cart_item(Product::random()).await)
        (shopping_cart_item(Product::random()).await)
    }
}

pub async fn order_summary() -> Markup {
    let cart = Cart {
        items: HashMap::from([
            (
                Uuid::new_v4(),
                CartItem {
                    listing: Product::random(),
                    number: 3,
                },
            ),
            (
                Uuid::new_v4(),
                CartItem {
                    listing: Product::random(),
                    number: 2,
                },
            ),
            (
                Uuid::new_v4(),
                CartItem {
                    listing: Product::random(),
                    number: 1,
                },
            ),
        ]),
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
                        (display_decimal(&cart.subtotal()))
                    }
                }
                a.button.is-fullwidth href="/checkout" {"Proceed to Checkout"}
        }
    }
}

async fn shopping_cart_item(item: Product) -> Markup {
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
