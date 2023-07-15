use leptonic::Input;
use leptos::*;

#[component]
pub fn Demo(cx: Scope) -> impl IntoView {
    let (state, set_state) = create_signal(cx, "Input value".to_owned());
    let error = Signal::derive(cx, move || {
        state.with(|value| {
            value
                .is_empty()
                .then_some("Please enter a value".to_owned())
        })
    });

    view! { cx,
        <div>
            <Input
                state
                set_state
                label="Input label"
                placeholder="Placeholder!"
                required=true
                error
            />
            <span>"Value: "{state}</span>
            <Input
                state
                set_state
                label="Disabled input"
                disabled=true
            />
        </div>
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
