use leptos::*;

fn main() {
    leptos::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let (x, set_x) = create_signal(0);
    let (count, set_count) = create_signal(0);
    let double_count = move || count() * 2;
    view! {
        <button
            on:click=move |_| {
                set_x.update(|n| *n += 10);
                set_count.update(|x| *x += 1);
            }

            // set the `style` attribute
            style="position:absolute"
            // and toggle individual CSS properties with `style`
            style:left=move || format!("{}px", x() + 100)
            style:background-color=move || format!("rgb({}, {}, 100)", x(), 100)
            style:max-width="400px"
            // Set a CSS variable for stylesheet use
            style=("--columns", x)
        >
            "Click to Move"
        </button>

        <br/>
        <br/>
        <progress
            max="10"
            // signals are functions, so `value=count` and `value=move || count.get()`
            // are interchangeable.
            value=count
        ></progress>

        <br/>
        <progress max="10" value=double_count></progress>

        <br/>
        <p>"Double Count: " {double_count}</p>
    }
}
