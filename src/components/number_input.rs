use super::error::*;
use crate::classes::*;
use crate::number::*;
use crate::util::*;
use leptos::*;

/// Counts the number of trailing zeros at the end of a decimal value.
fn count_trailing_zeros(value_str: &str) -> usize {
    match value_str.find('.') {
        Some(index) => {
            value_str[index..]
                .chars()
                .rev()
                .fold((true, 0usize), |(again, count), chr| {
                    if again && chr == '0' {
                        (true, count + 1)
                    } else {
                        (false, count)
                    }
                })
                .1
        }
        None => 0,
    }
}

/// Shortens a number to a specified number of decimal places.
fn shorten_to(value_str: &str, decimals: u16) -> String {
    match value_str.find('.') {
        Some(index) => {
            if index + (decimals as usize) < value_str.len() {
                (value_str[..=index + (decimals as usize)]).to_owned()
            } else {
                value_str.to_owned()
            }
        }
        None => value_str.to_owned(),
    }
}

/// Transforms a string representation of a number as needed.
fn transform_number(value_str: &str, decimals: u16) -> String {
    let mut value_str = value_str.to_owned();

    if value_str == "-" {
        value_str = "".to_owned();
    }

    if value_str.ends_with('-') {
        if value_str.starts_with('-') {
            value_str = (value_str[1..value_str.len() - 1]).to_owned()
        } else {
            value_str = format!("-{}", &value_str[..value_str.len() - 1])
        }
    }

    value_str = shorten_to(&value_str, decimals);

    if value_str.len() > 1 && value_str.starts_with('0') && !value_str.starts_with("0.") {
        value_str = (value_str[1..]).to_owned();
    }

    if value_str.len() > 2 && value_str.starts_with("-0") && !value_str.starts_with("-0.") {
        value_str = format!("-{}", (&value_str[2..]));
    }

    value_str
}

/// Parses the value of a string representation of a number in a text input box.
fn parse_number_value<N: Number>(value_str: &str, min: N, max: N) -> Option<(N, bool)> {
    match value_str.parse::<N>() {
        Ok(value) => {
            if value < min {
                Some((min, true))
            } else if value > max {
                Some((max, true))
            } else {
                Some((value, false))
            }
        }
        Err(_) => None,
    }
}

/// Parses a string representation of a number in a text input box.
fn parse_number<N: Number>(value_str: &str, min: N, max: N) -> Option<(N, bool)> {
    if value_str.is_empty() {
        Some((N::default(), true))
    } else if N::DECIMAL && value_str.ends_with('.') && value_str.matches('.').count() == 1 {
        parse_number_value(&value_str[..value_str.len() - 1], min, max)
    } else {
        parse_number_value(value_str, min, max)
    }
}

/// A wrapper around a number state.
#[derive(Debug, Clone, PartialEq)]
struct NumberState<N: Number> {
    /// The inner state string.
    state: String,
    /// The minimum value.
    min: N,
    /// The maximum value.
    max: N,
    /// The maximum number of digits after the decimal.
    decimals: u16,
}

impl<N: Number> NumberState<N> {
    /// Creates a new number state.
    pub fn new(value: N, min: N, max: N, decimals: u16) -> Self {
        Self {
            state: value.to_string(),
            min,
            max,
            decimals,
        }
    }

    /// Gets the inner value.
    pub fn get(&self) -> N {
        parse_number(&self.state, self.min, self.max).unwrap().0
    }

    /// Sets the inner state.
    pub fn set(&mut self, new_value_str: &str) {
        let new_value_transformed = transform_number(new_value_str, self.decimals);
        let maybe_new_value = parse_number(&new_value_transformed, self.min, self.max);

        if let Some((new_value, update_repr)) = maybe_new_value {
            if !update_repr {
                self.state = new_value_transformed;
            } else {
                self.state = new_value.to_string();
            }
        }
    }
}

impl<N: Number> ToString for NumberState<N> {
    fn to_string(&self) -> String {
        if self.state.is_empty() {
            N::default().to_string()
        } else {
            self.state.clone()
        }
    }
}

impl<N: Number> Default for NumberState<N> {
    fn default() -> Self {
        Self {
            state: String::new(),
            min: N::NUMBER_MIN,
            max: N::NUMBER_MAX,
            decimals: u16::MAX,
        }
    }
}

/// A number input element.
#[component]
pub fn NumberInput<N>(
    cx: Scope,
    /// Number input state.
    state: ReadSignal<N>,
    /// Number input state setter.
    set_state: WriteSignal<N>,
    /// Number input label.
    #[prop(into, optional)]
    label: MaybeSignal<String>,
    /// Number input placeholder text.
    #[prop(into, optional)]
    placeholder: MaybeSignal<String>,
    /// The minimum value.
    #[prop(into, optional, default = MaybeSignal::Static(N::NUMBER_MIN))]
    min: MaybeSignal<N>,
    /// The maximum value.
    #[prop(into, optional, default = MaybeSignal::Static(N::NUMBER_MAX))]
    max: MaybeSignal<N>,
    /// The maximum number of decimal places.
    #[prop(into, optional, default = MaybeSignal::Static(u16::MAX))]
    decimals: MaybeSignal<u16>,
    /// Whether the input is required.
    #[prop(into, optional, default = MaybeSignal::Static(false))]
    required: MaybeSignal<bool>,
    /// Whether the input is disabled.
    #[prop(into, optional, default = MaybeSignal::Static(false))]
    disabled: MaybeSignal<bool>,
    /// An optional error message.
    #[prop(into, optional)]
    error: MaybeSignal<Option<String>>,
) -> impl IntoView
where
    N: Number + 'static,
{
    let id = new_id();
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

    let (trailing_decimal, set_trailing_decimal) = create_signal(cx, false);
    let (trailing_zeros, set_trailing_zeros) = create_signal(cx, 0usize);
    let number_state = Signal::derive(cx, move || {
        let mut num = NumberState::new(state(), min(), max(), decimals());
        if trailing_decimal() {
            num.state.push('.');
        }
        if trailing_zeros() > 0 {
            if !num.state.contains('.') {
                num.state.push('.');
            }
            num.state.push_str(&"0".repeat(trailing_zeros()));
        }
        num
    });
    let on_input = move |ev| {
        let new_value_str = input_event_value(&ev);
        let mut current_number = number_state();
        current_number.set(&new_value_str);
        set_state(current_number.get());
        if current_number.state.ends_with('.') {
            set_trailing_decimal(true);
        } else {
            set_trailing_decimal(false);
        }
        set_trailing_zeros(count_trailing_zeros(&current_number.state));
    };

    view! { cx,
        <div class=container_class>
            <label for=id.clone() class="leptonic-input-label">
                {label}
                <span class="leptonic-required-mark">{required().then_some(" *").unwrap_or_default()}</span>
            </label>
            <input
                prop:value=move || number_state.with(|n| n.to_string())
                on:input=on_input
                id=id
                class=input_class
                type="text"
                placeholder=placeholder
                required=required
                disabled=disabled
            />
            <Error message=error size=ErrorSize::Small />
        </div>
    }
}
