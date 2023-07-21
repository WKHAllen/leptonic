use super::error::*;
use crate::classes::*;
use crate::util::*;
use leptos::*;

/// Textarea resize options.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum TextAreaResize {
    /// No resize.
    #[default]
    None,
    /// Horizontal resize only.
    Horizontal,
    /// Vertical resize only.
    Vertical,
    /// Both horizontal and vertical resize.
    Both,
}

impl TextAreaResize {
    /// Gets the name of the resize option.
    pub fn resize_option_name(&self) -> &'static str {
        match *self {
            Self::None => "none",
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
            Self::Both => "both",
        }
    }
}

/// A textarea element.
#[component]
pub fn TextArea(
    cx: Scope,
    /// Textarea state.
    state: ReadSignal<String>,
    /// Textarea state setter.
    set_state: WriteSignal<String>,
    /// Textarea label.
    #[prop(into, optional)]
    label: MaybeSignal<String>,
    /// Textarea placeholder text.
    #[prop(into, optional)]
    placeholder: MaybeSignal<String>,
    /// The maximum number of characters allowed.
    #[prop(into, optional, default = MaybeSignal::Static(524288))]
    max_length: MaybeSignal<usize>,
    /// In what way the textarea can be resized.
    #[prop(into, optional)]
    resize: MaybeSignal<TextAreaResize>,
    /// Whether the textarea is required.
    #[prop(into, optional, default = MaybeSignal::Static(false))]
    required: MaybeSignal<bool>,
    /// Whether the textarea is disabled.
    #[prop(into, optional, default = MaybeSignal::Static(false))]
    disabled: MaybeSignal<bool>,
    /// An optional error message.
    #[prop(into, optional)]
    error: MaybeSignal<Option<String>>,
) -> impl IntoView {
    let id = new_id();
    let container_class = move || {
        classes!(
            "leptonic-textarea-container",
            disabled().then_some("leptonic-textarea-container-disabled")
        )
    };
    let textarea_class = {
        let error = error.clone();
        move || {
            classes!(
                "leptonic-textarea",
                format!(
                    "leptonic-textarea-resize-{}",
                    resize.with(|r| r.resize_option_name())
                ),
                error().map(|_| "leptonic-textarea-invalid")
            )
        }
    };

    view! { cx,
        <div class=container_class>
            <label for=id.clone() class="leptonic-textarea-label">
                {label}
                <span class="leptonic-required-mark">{required().then_some(" *").unwrap_or_default()}</span>
            </label>
            <textarea
                prop:value=state
                on:input=move |ev| set_state(textarea_event_value(&ev))
                id=id
                class=textarea_class
                placeholder=placeholder
                maxlength=max_length
                required=required
                disabled=disabled
                rows=3
            />
            <Error message=error size=ErrorSize::Small />
        </div>
    }
}
