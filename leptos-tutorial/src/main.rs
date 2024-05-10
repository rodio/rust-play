use leptos::*;

fn main() {
    leptos::mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
    let values = vec![0, 1, 2];

    view! {
        <p>{values.clone()}</p>
        // or we can wrap them in li
        <ul>{values.into_iter().map(|n| view! { <li>{n}</li> }).collect::<Vec<_>>()}</ul>
    }
}
