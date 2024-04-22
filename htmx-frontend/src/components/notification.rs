use maud::{html, Markup};
use super::Color;


fn get_color(color: Color) -> &'static str {
    match color {
        Color::Default => "",
        Color::Primary => "is-primary",
        Color::Link => "is-link",
        Color::Info => "is-info",
        Color::Success => "is-success",
        Color::Warning => "is-warning",
        Color::Danger => "is-danger"
    }
}
pub async fn notification(message: &str, color: Color, light: bool) -> Markup {
    let light  = if light { "is-light" } else { "" };
    let class = format!("notification {} {}", get_color(color), light);
    html!(
        div class=(class) hx-on:click="this.remove()" {
            (message)
            button.delete;
        }
    )
}
