use leptos::*;

fn main() {
    leptos::mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = create_signal("Controlled".to_string());

    view! {
        <input
            type="text"
            on:input=move |ev| { set_name(event_target_value(&ev)) }
            prop:value=name
        />
        <p>"Name is " {name}</p>
    }
}
