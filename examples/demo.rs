use leptos::*;

mod error;
mod input;
mod theme;

macro_rules! demo_views {
    ( $cx:expr, [$( $demo:ident ),*] ) => {{
        let cx = $cx;
        view! { cx,
            $({
                use crate::$demo::Demo;
                view! { cx,
                    <Demo />
                }
            })*
        }
    }};
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|cx| {
        let demos = demo_views!(cx, [theme, error, input]);

        view! { cx,
            <div>
                {demos}
            </div>
        }
    })
}
