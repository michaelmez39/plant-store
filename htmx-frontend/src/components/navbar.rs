use crate::components::icons::SHOPPING_CART;
use maud::{html, Markup};

pub async fn navbar() -> Markup {
    html! {
        nav.navbar {
            .navbar-brand {
                .navbar-item {
                    figure.image {
                        img src="/assets/images/plantomics.webp" alt="Logo, agolden pothos icon";
                    }
                }
                a.navbar-burger {
                    span aria-hidden="true" {}
                    span aria-hidden="true" {}
                    span aria-hidden="true" {}
                    span aria-hidden="true" {}
                }
            }
            .navbar-menu {
                .navbar-start {
                    a.navbar-item href="/" { "Store" }
                }

                .navbar-end {
                    (navbar_end().await)
                }
            }
        }
    }
}

async fn navbar_end() -> Markup {
    let toggle_light = "document.querySelector('html').setAttribute('data-theme', this.checked ? 'dark' : 'light')";

    html! {
        .navbar-item {
            div {
                input.switch.is-rounded id="lightModeToggle" "type"="checkbox" onchange=(toggle_light) checked="checked" { }
                label "for"="lightModeToggle" {}
            }
        }
        .navbar-item {
            a.shopping-cart.has-text-dark-light.has-text-light-dark title="Shopping Cart" href="/shopping-cart" {
                span.pr-1 { (SHOPPING_CART) }
                span {
                    b {"Cart"}
                    br;
                    "3 items"
                }
            }
        }
        .navbar-item {
            .buttons {
                a.button.is-primary { "Sign Up" }
                a.button.is-secondary { "Login" }
            }
        }
    }
}
