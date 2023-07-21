use leptonic::{Button, ButtonStyle};
use leptos::*;

#[component]
pub fn Demo(cx: Scope) -> impl IntoView {
    let (state, set_state) = create_signal(cx, ButtonStyle::Primary);

    view! { cx,
        <Button
            text="Primary"
            on_click=move || set_state(ButtonStyle::Primary)
        />
        <Button
            text="Secondary"
            style=ButtonStyle::Secondary
            on_click=move || set_state(ButtonStyle::Secondary)
        />
        <Button
            text="Transparent"
            style=ButtonStyle::Transparent
            on_click=move || set_state(ButtonStyle::Transparent)
        />
        <Button
            text="Danger"
            style=ButtonStyle::Danger
            on_click=move || set_state(ButtonStyle::Danger)
        />
        <Button
            text="Disabled"
            style=state
            disabled=true
        />
        <span>"Last clicked: "{move || state.with(|s| s.style_name())}</span>
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
