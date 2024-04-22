use maud::{Markup, html};
use crate::components::icons::SHOPPING_CART;

pub async fn navbar() -> Markup {
    html! {
        nav.navbar {
            .navbar-brand {
                .navbar-item {
                    figure.image.mr-1 {
                        img src="/assets/golden-pothos.png" alt="Logo, agolden pothos icon";
                    }
                    "Rocks and Plants"
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
                    a.navbar-item href="/rocks" { "Your Rocks" }
                }
                .navbar-end {
                    .navbar-item {
                        .buttons {
                            a title="Shopping Cart" href="/shopping-cart" {
                                span.icon-text {
                                    span.icon.is-medium {
                                        i { (SHOPPING_CART) }
                                    }
                                    span {
                                        b {"My Cart"}
                                        br;
                                        "3 items"
                                    }
                                }
                            }
                            a.button.is-primary { "Sign Up" }
                            a.button.is-secondary { "Login" }
                        }
                    }
                }
            }
        }
    }
}

