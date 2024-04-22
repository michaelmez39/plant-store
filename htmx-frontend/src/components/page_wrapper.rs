use crate::components::navbar;
use maud::{html, Markup, DOCTYPE};

pub async fn page_wrapper(content: Markup, show_footer: bool) -> Markup {
    html!(
        (header("Rocks and Plants!").await)
        body {
            style {
                r#"
                    .is-full-cell{
                        grid-column: 1/-1;
                    }
                "#
            }
            (navbar().await)
            .container {
                #notifications {}
            }
            (content)
            @if show_footer {
                (footer().await)
            }
            (htmx_script().await)
        }
    )
}

async fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        title { (page_title) }
        link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma@1.0.0/css/bulma.min.css";
    }
}

async fn footer() -> Markup {
    html! {
        footer.footer {
            "You've reached " b { "the bottom" }
            br;
            a href="https://www.flaticon.com/free-icons/indoor-plants" title="indoor plants icons" {
                "Indoor plants icons created by Freepik - Flaticon"
            }
            br;
            "Font Awesome by Dave Gandy - https://fortawesome.github.com/Font-Awesome, CC BY-SA 3.0 <https://creativecommons.org/licenses/by-sa/3.0>, via Wikimedia Commons"
        }
    }
}

async fn htmx_script() -> Markup {
    html! {
    script src="https://unpkg.com/htmx.org@1.9.11"
        integrity="sha384-0gxUXCCR8yv9FM2b+U3FDbsKthCI66oH5IA9fHppQq9DDMHuMauqq1ZHBpJxQ0J0"
        crossorigin="anonymous" { }
    }
}
