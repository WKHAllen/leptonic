use crate::util::*;
use leptos::*;

/// The size of an error message.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ErrorSize {
    /// A very small message.
    Smaller,
    /// A small message.
    Small,
    /// A medium sized message.
    #[default]
    Medium,
    /// A large message.
    Large,
    /// A very large message.
    Larger,
}

impl ErrorSize {
    pub fn size_name(&self) -> &'static str {
        match *self {
            Self::Smaller => "smaller",
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::Larger => "larger",
        }
    }
}

/// An error element.
#[component]
pub fn Error(
    cx: Scope,
    /// The error message.
    #[prop(into, optional)]
    message: MaybeSignal<Option<String>>,
    /// The size of the error message.
    #[prop(into, optional)]
    size: MaybeSignal<ErrorSize>,
) -> impl IntoView {
    let class = move || {
        classes!(
            "leptonic-error",
            format!("leptonic-text-{}", size.with(|s| s.size_name()))
        )
    };

    view! { cx,
        <span class=class>{move || message().unwrap_or_default()}</span>
    }
}
