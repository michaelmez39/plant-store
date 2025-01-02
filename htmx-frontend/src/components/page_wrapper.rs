use crate::{components::navbar, Auth};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header::COOKIE, request::Parts, HeaderMap, StatusCode},
};
use maud::{html, Markup, DOCTYPE};
use tracing::info;

const DEFAULT_THEME: &'static str = "dark";

pub struct PageWrapper {
    auth: Auth,
    theme: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for PageWrapper
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth = Auth::from_request_parts(parts, state).await?;
        let theme: String = HeaderMap::from_request_parts(parts, state)
            .await
            .as_ref()
            .map(get_theme)
            .map(|t| t.unwrap_or(DEFAULT_THEME))
            .unwrap_or(DEFAULT_THEME)
            .to_owned();
        info!("{theme}");
        Ok(Self { auth, theme })
    }
}

fn get_theme(headers: &HeaderMap) -> Option<&str> {
    Some(
        headers
            .get(COOKIE)?
            .to_str()
            .ok()?
            .split_once("=")?
            .1
            .trim(),
    )
}

impl PageWrapper {
    pub fn render(self, template: Markup) -> Markup {
        html!(
            (header("Rocks and Plants!"))
            html data-theme=(&self.theme) {
                body {
                    (navbar(&self.auth))
                    div id="notifications" {}
                    (template)
                    (footer())
                    (htmx_script())
                }
            }
        )
    }
}

fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        title { (page_title) }
        link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bulma@1.0.0/css/bulma.min.css";
        link rel="stylesheet" href="/assets/stylesheets/global.css";
        link rel="stylesheet" href="/assets/stylesheets/switch.css";
    }
}

fn footer() -> Markup {
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

fn htmx_script() -> Markup {
    html! {
    script src="https://unpkg.com/htmx.org@1.9.11"
        integrity="sha384-0gxUXCCR8yv9FM2b+U3FDbsKthCI66oH5IA9fHppQq9DDMHuMauqq1ZHBpJxQ0J0"
        crossorigin="anonymous" { }
    }
}
