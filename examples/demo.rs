use convert_case::{Case, Casing};
use leptos::*;

mod button;
mod checkbox;
mod error;
mod input;
mod number_input;
mod switch;
mod textarea;
mod theme;

macro_rules! demo_views {
    ( $cx:expr, [$( $demo:ident ),*] ) => {{
        let cx = $cx;
        view! { cx,
            $({
                use crate::$demo::Demo;
                let name = stringify!($demo);
                let name_title = name.to_case(Case::Title);
                view! { cx,
                    <div class="leptonic-demo-item">
                        <span class="leptonic-demo-item-label">{name_title}</span>
                        <Demo />
                    </div>
                }
            })*
        }
    }};
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|cx| {
        let demos = demo_views!(
            cx,
            [
                theme,
                error,
                input,
                textarea,
                number_input,
                button,
                checkbox,
                switch
            ]
        );

        view! { cx,
            <div class="leptonic-demo">
                {demos}
            </div>
        }
    })
}
