use crate::{
    components::icons::{MOON, SHOPPING_CART},
    Auth,
};
use maud::{html, Markup};
use store_lib::account::User;

pub fn navbar(auth: &Auth) -> Markup {
    html! {
        nav.navbar {
            .navbar-brand {
                .navbar-item {
                    figure.image {
                        img.nav-logo src="/assets/images/plantomics.webp" alt="Logo, agolden pothos icon";
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
                    (account_buttons(&auth.user))
                }
            }
        }
    }
}

fn account_buttons(maybe_user: &Option<User>) -> Markup {
    html!(
        .navbar-item {
            @if let Some(ref user) = maybe_user {
                span { "Welcome, " (user.username)}
            }
        }
        .navbar-item.buttons {
            @if maybe_user.is_some() {
                form action="/logout" method="post" {
                    button.button.logout.is-warning type="submit"  { "Logout" }
                }
            } @else {
                a.button.is-link href="/signup" { "Sign Up" }
                a.button.is-secondary href="/login" { "Login" }
            }

            button.button.shopping-cart id="lightModeToggle" onclick=(TOGGLE_LIGHT) {
                span.icon { (MOON) }
            }

            (shopping_cart())
        }
    )
}

fn shopping_cart() -> Markup {
    html! (
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
    )
}

const TOGGLE_LIGHT: &'static str = r#"
    const htmlNode = document.querySelector('html')
    const flipped = htmlNode.getAttribute('data-theme') === 'light' ? 'dark' : 'light'
    htmlNode.setAttribute('data-theme', flipped)
    document.cookie = `theme=${flipped};`
"#;
