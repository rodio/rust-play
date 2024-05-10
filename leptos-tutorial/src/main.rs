use leptos::*;

fn main() {
    leptos::mount_to_body(App);
}

#[component]
fn ProgressBar(#[prop(optional)] progress: Option<Box<dyn Fn() -> i32>>) -> impl IntoView {
    progress.map(|progress| {
        view! { <progress max=100 value=progress></progress> }
    })
}

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    view! {
        <button on:click=move |_| {
            set_count.update(|n| *n += 1);
        }>"Click me"</button>
        <br/>
        <br/>
        <ProgressBar progress=Box::new(count)/>
    }
}
