use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use tokio;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 5);

    view! { cx,
        <h1>"Welcome to WASM!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <AboutMe />
        <InfiniteCounter />
    }
}

#[component]
fn AboutMe(cx: Scope) -> impl IntoView {
    let (expanded, set_expanded) = create_signal(cx, false);
    let on_click = move |_| set_expanded.update(|expanded| *expanded = !*expanded);

    view! { cx,
        <h2>"About Me"</h2>
        <button on:click=on_click>"Expand for more"</button>
        <Show
            when=move || expanded() == true
            fallback=|cx| view! { cx, "" }
        >
            <AboutMeDetails />
        </Show>
    }
}

#[component]
fn AboutMeDetails(cx: Scope) -> impl IntoView {
    view! { cx,
        <h3>"I am a WASM Leptos project built by Kyle Nissley"</h3>
        <p>"The purpose of this app is just to build something with Leptos for the first time, and try out some of the apis. I wanted to see how viable building something in Leptos is right now. It's pretty cool!"</p>
    }
}

#[component]
fn InfiniteCounter(cx: Scope) -> impl IntoView {
    let (count, set_count): (ReadSignal<u64>, WriteSignal<u64>) = create_signal(cx, 1);
    let on_click = move |_| set_count.update(|count| *count *= 99);

    view! { cx,
        <p on:click=on_click>{count}</p>
    }
}
