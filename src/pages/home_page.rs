use leptos::prelude::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);
    // let on_click2 = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <h2>"This is the home page."</h2>
        <button class="bg-green-400 px-2 py-1 ml-4 rounded-md text-sm" on:click=on_click>"Click Me: " {count}</button>
        <button class="bg-blue-700 px-2 py-1 ml-4 rounded-md text-sm cursor-pointer shadow shadow-gray-400">"Click Me: " {count}</button>
        // <Button on:click=on_click>"Shadcn Button: " {count}</Button>
    }
}