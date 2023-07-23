use crate::classes::*;
use crate::util::*;
use leptos::*;

/// A switch component.
#[component]
pub fn Switch(
    cx: Scope,
    /// Switch state.
    state: ReadSignal<bool>,
    /// Switch state setter.
    set_state: WriteSignal<bool>,
    /// The switch label.
    #[prop(into, optional)]
    label: MaybeSignal<String>,
    /// Whether the switch is disabled.
    #[prop(into, optional, default = MaybeSignal::Static(false))]
    disabled: MaybeSignal<bool>,
) -> impl IntoView {
    let label_class = move || {
        classes!(
            "leptonic-switch",
            disabled().then_some("leptonic-switch-disabled")
        )
    };

    view! { cx,
        <div class="leptonic-switch-container">
            <label class=label_class>
                <span class="leptonic-switch-label">{label}</span>
                <input
                    prop:checked=state
                    on:click=move |ev| set_state(checkbox_checked(ev))
                    type="checkbox"
                    {disabled}
                    class="leptonic-switch-input"
                />
                <span class="leptonic-switch-toggle"></span>
            </label>
        </div>
    }
}
