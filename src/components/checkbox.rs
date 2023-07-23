use crate::classes::*;
use crate::util::*;
use leptos::*;

/// A checkbox element.
#[component]
pub fn Checkbox(
    cx: Scope,
    /// Checkbox state.
    state: ReadSignal<bool>,
    /// Checkbox state setter.
    set_state: WriteSignal<bool>,
    /// The checkbox label.
    #[prop(into, optional)]
    label: MaybeSignal<String>,
    /// Whether the checkbox is disabled.
    #[prop(into, optional, default = MaybeSignal::Static(false))]
    disabled: MaybeSignal<bool>,
) -> impl IntoView {
    let label_class = move || {
        classes!(
            "leptonic-checkbox",
            disabled().then_some("leptonic-checkbox-disabled")
        )
    };

    view! { cx,
        <div class="leptonic-checkbox-container">
            <label class=label_class>
                <span class="leptonic-checkbox-label">{label}</span>
                <input
                    prop:checked=state
                    on:click=move |ev| set_state(checkbox_checked(ev))
                    type="checkbox"
                    {disabled}
                    class="leptonic-checkbox-input"
                />
                <span class="leptonic-checkmark">
                    // <img src="assets/svg/check-solid.svg" class="leptonic-checkmark-icon" />
                    <span class="leptonic-checkmark-icon"></span>
                </span>
            </label>
        </div>
    }
}
