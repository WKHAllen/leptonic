use leptonic::{Error, ErrorSize};
use leptos::*;

#[component]
pub fn Demo(cx: Scope) -> impl IntoView {
    view! { cx,
        <Error message=Some("The smallest error message".to_owned()) size=ErrorSize::Smaller />
        <Error message=Some("The small error message".to_owned()) size=ErrorSize::Small />
        <Error message=Some("The medium size error message".to_owned()) size=ErrorSize::Medium />
        <Error message=Some("The large error message".to_owned()) size=ErrorSize::Large />
        <Error message=Some("The largest error message".to_owned()) size=ErrorSize::Larger />
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
