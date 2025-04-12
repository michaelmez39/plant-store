use crate::{components::navbar, Auth};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

use axum_extra::extract::cookie::CookieJar;
use maud::{html, Markup, DOCTYPE};

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
        let cookies = CookieJar::from_request_parts(parts, state).await.unwrap(); // SAFETY: infallible

        let theme: String = cookies
            .get("theme")
            .map(|t| t.value())
            .unwrap_or(DEFAULT_THEME)
            .to_owned();

        Ok(Self { auth, theme })
    }
}

impl PageWrapper {
    pub fn render(self, template: Markup) -> Markup {
        html!(
            html data-theme=(&self.theme) {
                (header("Rocks and Plants!"))
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
        link rel="preconnect" href="https://fonts.googleapis.com";
        link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
        link href="https://fonts.googleapis.com/css2?family=Inter:ital,opsz,wght@0,14..32,100..900;1,14..32,100..900&family=Playfair+Display+SC:ital,wght@0,400;0,700;0,900;1,400;1,700;1,900&display=swap" rel="stylesheet";

        link rel="stylesheet" href="/assets/stylesheets/global.css";
        link rel="stylesheet" href="/assets/stylesheets/switch.css";
    }
}

fn footer() -> Markup {
    html! {
        footer.footer {
            "Plantomics 2025 &copy;"
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
