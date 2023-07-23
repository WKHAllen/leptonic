use leptonic::Switch;
use leptos::*;

#[component]
pub fn Demo(cx: Scope) -> impl IntoView {
    let (state, set_state) = create_signal(cx, true);

    view! { cx,
        <Switch
            state
            set_state
            label="Switch label"
        />
        <span>"Value: "{state}</span>
        <Switch
            state
            set_state
            label="Disabled switch"
            disabled=true
        />
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
