use leptos::*;
use tracing::debug;

fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    mount_to_body(|cx| {
        view! { cx,
            <div
                style="height: 500px; width: 500px; background: black;"
                // This works as expected
                on:click=move |_| debug!("Clicked!")
                // This doesn't work
                on:mouseenter=move |_| debug!("Mouse entered!")
                // This doesn't work either
                on:mouseleave=move |_| debug!("Mouse left!")
            />
        }
    })
}
