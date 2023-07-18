use leptonic::use_theme;
use leptos::*;

#[component]
pub fn Demo(cx: Scope) -> impl IntoView {
    let (_theme, _set_theme) = use_theme(cx);

    view! { cx,
        <div>"Theme customization placeholder"</div>
    }
}

#[allow(dead_code)]
fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|cx| {
        view! { cx, <Demo /> }
    })
}
