use maud::{html, Markup};
use rust_decimal::Decimal;
use store_lib::{
    cart::{Cart, CartItem},
    store::ItemListing,
};

use crate::{
    components::{page_wrapper, text_field},
    utils::display_decimal,
};

pub async fn checkout() -> Markup {
    page_wrapper(page_body().await, false).await
}

async fn page_body() -> Markup {
    html! {
        .section{
            .container {
                .columns {
                    .column.is-two-thirds {
                        a href="/shopping-cart" { "← Review Your Order" }
                        h2.title.is-3 { "Shipping" }
                        .box {
                            (shipping_form().await)
                        }
                        h2.title.is-3 { "Payment" }
                        .box {
                            (payment_form().await)
                        }
                    }
                    .column {
                        (order_summary().await)
                    }
                }
            }
        }
    }
}

async fn review_item(item: &ItemListing) -> Markup {
    html! {
        .columns.is-mobile {
            .column {
                .image.is-48x48.is-flex.is-align-items-center {
                    img src=(format!("/assets/images/{}", item.image));
                }
            }
            .column.is-half {
                (item.name)
            }
            .column.has-text-right{
                (display_decimal(item.price))
            }
        }
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
                @for item in &cart.items {
                    (review_item(&item.listing).await)
                }
                hr;
                .level.is-mobile {
                    .level-left {
                        (format!("Subtotal ({} Items)", cart.items.len()))
                    }
                    .level-right {(display_decimal(cart.subtotal()))}
                }
                .level.is-mobile {
                    .level-left {"Fedex Standard Shipping"}
                    .level-right {"$6.99"}
                }
                hr;
                .level.is-mobile {
                    .level-left {"Total Cost:"}
                    .level-right {
                        (display_decimal(cart.subtotal() + Decimal::new(799, 2)))
                    }
                }
                button.button.is-link.is-fullwidth { "Place Order" }
        }
    }
}

async fn address_fields(label: &'static str) -> Markup {
    html! {
        (text_field(&format!("{} Address", label), None).await)
        .field.is-horizontal {
            .field-body {
                (text_field("City", None).await)
                (text_field("State", None).await)
                (text_field("Zip", None).await)
            }
        }
    }
}

async fn shipping_form() -> Markup {
    html! {
        form {
            .field.is-horizontal {
                .field-body {
                    (text_field("First Name", None).await)
                    (text_field("Last Name", None).await)
                }
            }
            (address_fields("Shipping").await)
            .field {
                .control {
                    label.label { "Shipping Method" }
                    div.is-flex {
                        div.radio-button {
                            input "type"="radio" name="shipping_method" id="uspsStandard";
                            label.button "for"="uspsStandard" { "Usps Standard" }
                        }
                        div.radio-button{
                            input "type"="radio" name="shipping_method" id="priority";
                            label.button "for"="priority" { "USPS Priority" }
                        }
                        div.radio-button {
                            input "type"="radio" name="shipping_method" id="expedited";
                            label.button "for"="expedited" { "FedEx Expedited" }
                        }
                    }
                }
            }
        }
    }
}

async fn payment_form() -> Markup {
    html! {
         form {
             .field.is-horizontal {
                 .field-body.columns {
                     div.column {
                         (text_field("Card Number", None).await)
                     }
                     div.column.is-one-fifth {
                         (text_field("CVV", None).await)
                     }
                 }
             }
             label.checkbox {
                 input "type"="checkbox" {}
                 "Same as mailing"
             }
             (address_fields("Billing").await)
         }
    }
}
