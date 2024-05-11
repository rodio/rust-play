use leptos::*;

fn main() {
    leptos::mount_to_body(App);
}

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: RwSignal<i32>,
}

#[component]
pub fn App() -> impl IntoView {
    let (data, _) = create_signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: create_rw_signal(10),
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: create_rw_signal(20),
        },
        DatabaseEntry {
            key: "baz".to_string(),
            value: create_rw_signal(30),
        },
    ]);

    view! {
        <button on:click=move |_| {
            data.with(|data| {
                for row in data {
                    row.value.update(|value| *value *= 2)
                }
            });
        }>
            "Update Vlaues"
        </button>

        <For
            each = data
            key = |state| state.key.clone()
            let:child
        >
            <p>{move || child.value}</p>
        </For>
    }
}
