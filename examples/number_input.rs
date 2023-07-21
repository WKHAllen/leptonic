use leptonic::NumberInput;
use leptos::*;

#[component]
pub fn Demo(cx: Scope) -> impl IntoView {
    let (int_state, set_int_state) = create_signal(cx, 3u16);
    let (float_state, set_float_state) = create_signal(cx, 1.618f64);
    let int_error = Signal::derive(cx, move || {
        (int_state() == 3).then_some("How about something other than 3".to_owned())
    });
    let float_error = Signal::derive(cx, move || {
        (float_state() == 1.618).then_some("No phi, please".to_owned())
    });

    view! { cx,
        <NumberInput
            state=int_state
            set_state=set_int_state
            label="Int number input label"
            placeholder="Placeholder!"
            min=0u16
            max=100u16
            required=true
            error=int_error
        />
        <span>"Value: "{int_state}</span>
        <NumberInput
            state=float_state
            set_state=set_float_state
            label="Float number input label"
            placeholder="Placeholder!"
            min=-5.0
            max=5.0
            decimals=5u16
            required=true
            error=float_error
        />
        <span>"Value: "{float_state}</span>
        <NumberInput
            state=int_state
            set_state=set_int_state
            label="Disabled number input"
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
