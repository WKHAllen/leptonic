use leptonic::{TextArea, TextAreaResize};
use leptos::*;

#[component]
pub fn Demo(cx: Scope) -> impl IntoView {
    let (state1, set_state1) = create_signal(cx, "Textarea value".to_owned());
    let (state2, set_state2) = create_signal(cx, String::new());
    let (state3, set_state3) = create_signal(cx, String::new());
    let error = Signal::derive(cx, move || {
        state1.with(|value| {
            value
                .is_empty()
                .then_some("Please enter a value".to_owned())
        })
    });

    view! { cx,
        <TextArea
            state=state1
            set_state=set_state1
            label="Textarea label"
            placeholder="Placeholder!"
            required=true
            error
        />
        <span>"Value: "{state1}</span>
        <TextArea
            state={state1}
            set_state={set_state1}
            label="Disabled textarea"
            disabled=true
            resize=TextAreaResize::Horizontal
        />
        <TextArea
            state={state2}
            set_state={set_state2}
            label="Vertical resize"
            resize=TextAreaResize::Vertical
        />
        <TextArea
            state={state3}
            set_state={set_state3}
            label="Full resize"
            resize=TextAreaResize::Both
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
