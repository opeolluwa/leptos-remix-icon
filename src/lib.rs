use leptos::{component, view, IntoView};
pub mod pkg;

/// The Icon takes three props
/// icon - an equivalent of remix icon without the "ri-" prefix
/// style - custom css rules
/// class - tailwind or custom css classes
///
///
/// ### Example (tailwind)
/// ```rust
/// <Icon class="text-2xl" icon="github-line"/>
///  ```
///
/// ### Example (with style)
///  <Icon class="text-2xl" icon="github-line" style="background-color-red; border:1px solid green;"/>
///
///
#[component]
pub fn Icon(
    /// additional tailwind or custom css classes
    #[prop(default = "")]
    class: &'static str,
    /// css style rules
    #[prop(default = "")]
    style: &'static str,
    /// the remix icon class without the "ri-" prefix
    #[prop(default = "")]
    icon: &'static str,
    /// remix icon class
    #[prop(default = "")]
    size: &'static str,
) -> impl IntoView {
    let icon_size = pkg::IconSize::from_str(&size.trim())
        .unwrap_or_default()
        .as_str();
    view! { <i class=format!("ri-{icon} {class} {icon_size}") style=style></i> }
}
