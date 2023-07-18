use super::error::*;
use crate::classes::*;
use crate::util::*;
use leptos::*;

/// The type of input element.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum InputType {
    /// Standard text input.
    #[default]
    Text,
    /// Email address input.
    Email,
    /// Telephone number input.
    Tel,
    /// URL input.
    Url,
    /// Password input.
    Password,
}

impl InputType {
    /// Gets the HTML input element type corresponding to the current input type.
    pub fn html_input_type(&self) -> &'static str {
        match *self {
            Self::Text => "text",
            Self::Email => "email",
            Self::Tel => "tel",
            Self::Url => "url",
            Self::Password => "password",
        }
    }
}

/// An input element.
#[component]
pub fn Input(
    cx: Scope,
    /// Input state.
    state: ReadSignal<String>,
    /// Input state setter.
    set_state: WriteSignal<String>,
    /// Input type.
    #[prop(optional)]
    input_type: InputType,
    /// Input label.
    #[prop(into, optional)]
    label: MaybeSignal<String>,
    /// Input placeholder text.
    #[prop(into, optional)]
    placeholder: MaybeSignal<String>,
    /// The maximum number of characters allowed.
    #[prop(into, optional, default = MaybeSignal::Static(524288))]
    max_length: MaybeSignal<usize>,
    /// Whether the input is required.
    #[prop(into, optional, default = MaybeSignal::Static(false))]
    required: MaybeSignal<bool>,
    /// Whether the input is disabled.
    #[prop(into, optional, default = MaybeSignal::Static(false))]
    disabled: MaybeSignal<bool>,
    /// An optional error message.
    #[prop(into, optional)]
    error: MaybeSignal<Option<String>>,
) -> impl IntoView {
    let id = new_id();
    let html_input_type = input_type.html_input_type();
    let container_class = move || {
        classes!(
            "leptonic-input-container",
            disabled().then_some("leptonic-input-container-disabled")
        )
    };
    let input_class = {
        let error = error.clone();
        move || classes!("leptonic-input", error().map(|_| "leptonic-input-invalid"))
    };

    view! { cx,
        <div class=container_class>
            <label for=id.clone()>
                {label}
                <span class="leptonic-required-mark">{required().then_some(" *").unwrap_or_default()}</span>
            </label>
            <input
                prop:value=state
                on:input=move |ev| set_state(input_event_value(&ev))
                id=id
                class=input_class
                type=html_input_type
                placeholder=placeholder
                maxlength=max_length
                required=required
                disabled=disabled
            />
            <Error message=error size=ErrorSize::Small />
        </div>
    }
}
