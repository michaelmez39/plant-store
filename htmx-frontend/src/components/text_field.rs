use maud::{html, Markup};

pub async fn text_field(label: &str, help: Option<&'static str>) -> Markup {
    html! {
        .field {
            label.label { (label) }
            .control.is-expanded {
                input.input type="text";
            }
            @if let Some(help) = help {
                p.help { (help) }
            }
        }
    }
}
