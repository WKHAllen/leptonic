use crate::classes::*;
use leptos::*;

/// The style of a button.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ButtonStyle {
    /// Primary style.
    #[default]
    Primary,
    /// Secondary style.
    Secondary,
    /// Transparent style.
    Transparent,
    /// Danger style.
    Danger,
}

impl ButtonStyle {
    /// Gets the name of the button style.
    pub fn style_name(&self) -> &'static str {
        match *self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Transparent => "transparent",
            Self::Danger => "danger",
        }
    }
}

/// An abstraction over a button's click event callback.
#[derive(Default)]
pub struct OnClickCallback(Option<Box<dyn Fn() + 'static>>);

impl OnClickCallback {
    /// Call the callback.
    fn call(&self) {
        if let Some(callback) = &self.0 {
            (**callback)()
        }
    }
}

impl<F> From<F> for OnClickCallback
where
    F: Fn() + 'static,
{
    fn from(value: F) -> Self {
        Self(Some(Box::new(value)))
    }
}

impl<F> From<Option<F>> for OnClickCallback
where
    F: Fn() + 'static,
{
    fn from(value: Option<F>) -> Self {
        Self(match value {
            Some(callback) => Some(Box::new(callback)),
            None => None,
        })
    }
}

/// A button element.
#[component]
pub fn Button(
    cx: Scope,
    /// The text on the button.
    #[prop(into)]
    text: MaybeSignal<String>,
    /// The button click callback.
    #[prop(into, optional)]
    on_click: OnClickCallback,
    /// The button style.
    #[prop(into, optional)]
    style: MaybeSignal<ButtonStyle>,
    /// Whether the button is disabled.
    #[prop(into, optional, default = MaybeSignal::Static(false))]
    disabled: MaybeSignal<bool>,
) -> impl IntoView {
    let button_class = move || {
        classes!(
            "leptonic-button",
            format!("leptonic-button-{}", style.with(|s| s.style_name()))
        )
    };

    view! { cx,
        <button
            on:click=move |_| on_click.call()
            class=button_class
            type="button"
            disabled=disabled
        >
            {text}
        </button>
    }
}
