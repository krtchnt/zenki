use codee::string::FromToStringCodec;
use leptos::prelude::*;
use leptos_meta::Title;

/// Renders the home page of your application.
#[component]
pub fn Home() -> impl IntoView {
    // TODO: update this

    // Creates a reactive value to update the button
    let (counter, set_counter) = leptos_use::use_cookie::<u32, FromToStringCodec>("counter");

    let reset = move || set_counter.set(Some(0));

    if counter.get().is_none() {
        reset();
    }

    let increase = move || {
        set_counter.set(counter.get().map(|c| c + 1));
    };

    view! {
        <Title text="Welcome to Leptos"/>
        <p>Counter: {move || counter.get().map_or("—".to_string(), |c| c.to_string())}</p>
        <button on:click=move |_| reset()>Reset</button>
        <button on:click=move |_| increase()>+</button>
    }
}
