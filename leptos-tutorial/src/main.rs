use leptos::{svg::view, *};

fn main() {
    leptos::mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <DynamicList initial_length=2/>
    }
}

#[component]
fn DynamicList(initial_length: usize) -> impl IntoView {
    let mut next_counter_id = initial_length;
    let initial_counters = (0..initial_length)
        .map(|id| (id, create_signal(id + 1)))
        .collect::<Vec<_>>();

    // now we store that initial list in a signal
    // this way, we'll be able to modify the list over time,
    // adding and removing counters, and it will change reactively
    let (counters, set_counters) = create_signal(initial_counters);
    let add_counter = move |_| {
        let sig = create_signal(next_counter_id + 1);
        set_counters.update(move |counters| counters.push((next_counter_id, sig)));
        next_counter_id += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>"Add counter"</button>
            <ul>
                <For
                    each=counters
                    key=|counter| counter.0
                    children=move |(id, (count, set_count))| {
                        view! {
                            <li>
                                <button on:click = move |_| set_count.update(| n | * n += 1)>
                                    { count }
                                </button>
                                <button on:click = move|_| {
                                    set_counters
                                        .update(|counters| {
                                            counters.retain(|(counter_id, (signal, _))| {
                                                if counter_id == &id {
                                                    signal.dispose();
                                                }
                                                counter_id != &id

                                            })
                                        })
                                } >
                            "Remove" </ button > </ li >
                        }
                    }
                />

            </ul>
        </div>
    }
}
