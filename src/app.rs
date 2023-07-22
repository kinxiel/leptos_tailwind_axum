use chrono::{Datelike, NaiveDate};
use leptos::{error::Result, *};
use leptos::{html::Li, *};
use leptos_meta::{provide_meta_context, Link, Stylesheet, Title};
use leptos_router::{Route, Router, Routes};
use log::info;
use log::Level;

// This is part is used in the example for parent child communication.
// We create a new type (something like a type alias in TypeScript). It is not completely necessary
// but it hel[s for organization].

// The part below that starts with a `#` is what is called an attribute, and inside it we use
// the `derive` marco to automatically implement the `Copy` and `Clone` traits for us. You can manually implement
// it but it will make the code more verbose.

// Traits are like interfaces or protocols in other languages. `WriteSignal` is a type in Leptos that is
// a setter for the signal.
#[derive(Copy, Clone)]
struct ObjectContainContext(WriteSignal<bool>);

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route
                    path=""
                    view=move |cx| {
                        view! { cx, <Home/> }
                    }
                />
                <Route
                    path="text_input"
                    view=move |cx| {
                        view! { cx, <TextInput/> }
                    }
                />
                <Route
                    path="control_flow"
                    view=move |cx| {
                        view! { cx, <ControlFlow/> }
                    }
                />
                <Route
                    path="parent_child_communication"
                    view=move |cx| {
                        view! { cx, <ParentChildCommunication/> }
                    }
                />
                <Route
                    path="pass_children"
                    view=move |cx| {
                        view! { cx, <PassChildren/> }
                    }
                />
                <Route
                    path="fetch"
                    view=move |cx| {
                        view! { cx, <Fetch/> }
                    }
                />
            </Routes>
        </Router>
    }
}

// Topics: signals, derived signals, props (default, signals)
// The component attribute marks your code to be a component. Leptos does the heavy lifting of making your Rust code work as a component.
#[component]
fn Home(cx: Scope) -> impl IntoView {
    // This is how you create a typical signal in Leptos. It looks like a typical React state hook if you know React.
    // You basically pattern match (or destructure) `value` which refers the value of the actual signal and `set_value` which
    // refers to the setter function.
    let (value, set_value) = create_signal(cx, 0);
    // You can create a derived signal which is a variable that depends on a signal. When the signal variable updates, this
    // also updates. But as we will see later, they do not have the same type so you will need to some type assignments to make it work.
    let pixel_value = move || value.get() * 5;

    view! { cx,
        <Title text="Leptos Examples"/>
        <main>
            <div class="flex flex-col min-h-screen font-mono text-white bg-gradient-to-tl from-blue-800 to-blue-100">
                <div class="flex flex-col m-auto">
                    // Check the ProgressBar component definition below to see how this works. Basically we are passing two props
                    // the `max` prop which is a static value that determines the max value of the progress bar and `progress`, which
                    // well determines the progress value of the progress bar, progress is set to a reactive signal `value`. which we defined above.
                    // Try passing in a different value for `max` to see the change. Clicking on the `+` or `-` buttons changes the state value
                    // for `value`, which in turn changes the value of the progress bar.
                    <div class="mx-auto">
                        <ProgressBar max=200 progress=value/>
                    </div>
                    <div class="mx-auto">
                        <button
                            // This is how setters are triggered. You basically add an event listener to the element and
                            // each time the event is fired, it triggers a closure (you can think of it as an anonymous function) that
                            // runs and updates `value` using the `set_value()` setter.

                            // The * in *value dereferences value, which means we access the actual value and not the pointer to the value.
                            on:click=move |_| set_value.update(|value| *value -= 5)
                            class="px-3 py-2 m-1 text-white bg-blue-700 border-b-4 border-l-2 border-blue-800 rounded shadow-lg"
                        >
                            "-"
                        </button>
                        <button class="px-3 py-2 m-1 text-white bg-blue-800 border-b-4 border-l-2 border-blue-900 rounded shadow-lg">
                            {value}
                        </button>
                        <button
                            on:click=move |_| set_value.update(|value| *value += 5)
                            class="px-3 py-2 m-1 text-white bg-blue-700 border-b-4 border-l-2 border-blue-800 rounded shadow-lg"
                        >
                            "+"
                        </button>
                    </div>
                    <div class="mx-auto">
                        <ItsMeMario width= value />
                        // <ItsMeMario width= pixel_value />
                        // <ItsMeMario width= Signal::derive(cx, pixel_value) />
                    </div>
                </div>
            </div>
        </main>
    }
}

#[component]
// cx is has a type of Scope, which is used to manage memory within the Reactive system. You can think of it like "context"
fn ProgressBar(
    cx: Scope,
    // Adding this prop default attribute, allows you to set a default value for a prop. There is also an `option` variant to
    // to make prop optional.
    #[prop(default = 100)] max: u16,
    // `ReadSignal` is the type of the signal that is crated to hold the state of this component. Because it is a signal it is reactive.
    progress: ReadSignal<i32>,
    // IntoView is a trait, but it basically converts the value into a `View`.
) -> impl IntoView {
    // The `view!` macro lets you write JSX like syntax.
    view! { cx, <progress max=max value=progress></progress> }
}

// Naive Mario
#[component]
fn ItsMeMario(cx: Scope, width: ReadSignal<i32>) -> impl IntoView {
    view! { cx,
        <img src="https://upload.wikimedia.org/wikipedia/en/thumb/a/a9/MarioNSMBUDeluxe.png/220px-MarioNSMBUDeluxe.png" width=width />
    }
}

// Generic Mario: First method for dealing with derived signals
// There are several differences here. First we add the generic type `<F>` and assign that type to `width`, so `width` is now generic, which means
// it can take other types. What type to take in is defined in the `where` clause right below the first line. This basically says, we have a type
// that is a closure (Fn) and it returns an integer of type i32.
// #[component]
// fn ItsMeMario<F>(cx: Scope, width: F) -> impl IntoView
// where
//     F: Fn() -> i32 + 'static,
// {
//     view! { cx,
//         <img src="https://upload.wikimedia.org/wikipedia/en/thumb/a/a9/MarioNSMBUDeluxe.png/220px-MarioNSMBUDeluxe.png" width=width />
//     }
// }

// Into Mario using #[prop(into)]: Second method for dealing with derived signals
// #[component]
// fn ItsMeMario(cx: Scope, #[prop(into)] width: Signal<i32>) -> impl IntoView {
//     view! { cx,
//         <img src="https://upload.wikimedia.org/wikipedia/en/thumb/a/a9/MarioNSMBUDeluxe.png/220px-MarioNSMBUDeluxe.png" width=width />
//     }
// }

/// Text input example (binding, conditional classes)
#[component]
fn TextInput(cx: Scope) -> impl IntoView {
    // Create a basic signal like before, but instead of an integer use text.
    let (text, set_text) = create_signal(cx, "Enter some text".to_string());

    view! { cx,
        <main class="max-w-2xl mx-auto mt-12">
            <div>
                <h1 class="text-3xl">"Input binding and conditional classes"</h1>
            </div>
            // This is where we do the conditional class logic. This basically says, of the value of `text` is exactly `blue`,
            // add the class `bg-blue-100` to this HTML element.
            <div class="mt-4 text-2xl shadow" class=("bg-blue-100", move || text.get() == "blue")>
                <h2>{text}</h2>
            </div>
            <div>
                <input
                    type="text"
                    class="mt-4 border"
                    // event_target_value is basically a helper function that does something like event.target.value in JavaScript.
                    on:input=move |event| set_text(event_target_value(&event))
                    // Here we assign the value of the input to be `text` (the signal)
                    prop:value=text
                />
            </div>
            <p class="text-xs">"Change the text to `blue` to make the background turn blue."</p>
        </main>
    }
}

/// Control flow example
fn ControlFlow(cx: Scope) -> impl IntoView {
    // Implement control with using native Rust code
    // * Because the chrono crate supports different time formats you need to be particular
    // * in which format to pass to the browser, because browsers only accept dates in "%Y-%m-%d" format.
    let (date, set_date) = create_signal(cx, NaiveDate::from_ymd_opt(2023, 07, 22).unwrap());

    view! { cx,
        <div class="max-w-2xl mx-auto mt-10">
            <h1 class="text-3xl">"Control Flow"</h1>
            <input
                type="date"
                class="mt-6"
                // Don't have pay special attention to this, it is just some date conversion to the appropriate format that can
                // be accepted by the browser.
                on:change=move |event| set_date(
                    NaiveDate::parse_from_str(&event_target_value(&event), "%Y-%m-%d").unwrap(),
                )
                prop:value=move || date.get().format("%Y-%m-%d").to_string()
            />
            <p class="text-2xl font-bold">
                // This is where the control flow logic occurs. A bit naive but it gets the work done for this example.
                // Like many things in Leptos, you need to handle this in a closure.
                {move || {
                    // Basically if the date value's weekday() method returns somethings that is either "Sat" or "Sun" return
                    // weekend else return weekday.
                    if date.get().weekday().to_string().contains("Sat")
                        || date.get().weekday().to_string().contains("Sun")
                    {
                        date.get().format("%Y-%m-%d").to_string() + " is a weekend"
                    } else {
                        date.get().format("%Y-%m-%d").to_string() + " is a weekday"
                    }
                }}
            </p>
        </div>
    }
}

/// ParentChildCommunication
#[component]
fn ParentChildCommunication(cx: Scope) -> impl IntoView {
    let (object_cover, set_object_cover) = create_signal(cx, false);
    let (object_scale_down, set_object_scale_down) = create_signal(cx, false);
    let (object_contain, set_object_contain) = create_signal(cx, false);

    // * To use context to facilitate parent child communication. New type defined above.
    provide_context(cx, ObjectContainContext(set_object_contain));

    view! { cx,
        <div class="mx-auto mt-24 border shadow w-44">
            <img
                class="w-44 h-44"
                class=("object-cover", object_cover)
                class=("object-scale-down", object_scale_down)
                class=("object-contain", object_contain)
                src="https://upload.wikimedia.org/wikipedia/en/8/85/New_Super_Mario_Bros._U_Gameplay_2.jpg"
            />
            <div>
                <div class="mt-4 text-center border shadow">
                    <ObjectCover setter=set_object_cover/>
                </div>
                <div class="mt-4 text-center border shadow">
                    <ObjectScaleDown on:click=move |_| set_object_scale_down.update(|value| *value = !*value)/>
                </div>
                <div class="mt-4 text-center border shadow">
                    <ObjectContain/>
                </div>
            </div>
        </div>
    }
}

#[component]
// Pass signal to the child
pub fn ObjectCover(
    cx: Scope,
    /// Signal that will be toggled when the button is clicked.
    setter: WriteSignal<bool>,
) -> impl IntoView {
    view! { cx,
        <button on:click=move |_| setter.update(|value| *value = !*value)>"Object Cover"</button>
    }
}

#[component]
// Set an event listener on a component. Basically the component becomes a simple placeholder.
pub fn ObjectScaleDown(cx: Scope) -> impl IntoView {
    view! { cx, <button>"Object Scale Down"</button> }
}

#[component]
// Use `provide context` and `use_context` to facilitate parent-child communication
pub fn ObjectContain(cx: Scope) -> impl IntoView {
    // The ObjectContainContext type is an alias we defined at the very top of this file.
    // `use_context` basically allows you to reach into context and pull out a value.
    let setter: WriteSignal<bool> = use_context::<ObjectContainContext>(cx).unwrap().0;

    view! { cx,
        <button on:click=move |_| setter.update(|value| *value = !*value)>"Object Contain"</button>
    }
}

/// Pass children in a component
#[component]
pub fn PassChildren(cx: Scope) -> impl IntoView {
    view! { cx,
        <AcceptsChildren>
            <p>"Item 1"</p>
            <p>"Item 2"</p>
        </AcceptsChildren>
    }
}

#[component]
pub fn AcceptsChildren(cx: Scope, children: ChildrenFn) -> impl IntoView {
    // childrenFn is short of Box<dyn Fn(Scope) -> Fragment>
    let children_context = children(cx);

    // Since this is still Rust code, Rust rules apply so you need to clone here (read more on Rust's ownership concept for more info).
    // But basically, if you have an owned value, passing that value into a function consumes that value so without creating a copy or clone
    // there will be no value left for another function to use.
    // HTMLElement and Li are Leptos helper HTML types.
    let children: Vec<HtmlElement<Li>> = children_context
        .clone()
        .nodes
        .into_iter()
        .map(|child| view! { cx, <li>{child}</li> })
        .collect::<Vec<_>>();

    // We use children_context here, but render something slightly different using the same data.
    let styled_children: Vec<HtmlElement<Li>> = children_context
        .nodes
        .into_iter()
        .map(|child| {
            view! { cx,
                <li class="pl-4 mt-4 tracking-widest bg-blue-100 border rounded-lg shadow">
                    {child}
                </li>
            }
        })
        .collect::<Vec<_>>();

    view! { cx,
        <div class="max-w-3xl mx-auto mt-24">
            // Rust ownership and borrowing means we need to clone this value if we use it more than once.
            <ul>{children.clone()}</ul>
            <ul>{children}</ul>
            <ul>{styled_children}</ul>
        </div>
    }
}

/// Fetch Example (struct mapping, logging)
// https://www.amiiboapi.com/api/amiibo/?name=mario
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct Amiibo {
    amiiboSeries: String,
    character: String,
    gameSeries: String,
    head: String,
    image: String,
    name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Data {
    // A Vec is basically like a dynamic array, list in Rust. There are arrays in Rust but they are static (length cannot change)
    amiibo: Vec<Amiibo>,
}

// An async fetch function we call later.
// The Result type is very common Rust enum which either results in an OK (the thing were trying worked so give me back my data) or an Error
// in that case send me an error.
async fn fetch_character() -> Result<Vec<Amiibo>> {
    // reqwasm is a HTTP request library for WASM apps. Provides Rust binding for the web_sys
    // fetch and WebSocket API.
    // Async await looks similar to how it is in JavaScript.
    let res =
        reqwasm::http::Request::get(&format!("https://www.amiiboapi.com/api/amiibo/?name=mario",))
            .send()
            .await?
            // convert it to JSON
            .json::<Data>()
            .await?;

    // From our response, give me back the amiibo "array". I just want the "array" because I want to loop through the elements later.
    let response = res.amiibo;
    Ok(response)
}

#[component]
pub fn Fetch(cx: Scope) -> impl IntoView {
    // Bonus: This is how we log to the browser's console. We use the `console_log` crate to add this capability.
    let _ = console_log::init_with_level(Level::Debug);
    // Show up in the browser console as an info log.
    info!("This log will appear in the browser console");

    let character_series =
        // Just run once using a non-reactive empty source signal. You can create something that runs on every trigger by 
        // passing in a signal instead of something empty.
        // A resource is Leptos's way of handling data returned by an asynchronous task. 
        create_local_resource(cx, || (), |_| async move { fetch_character().await });

    // A big verbose, but basically we read the data that is contained in `character_series`, the resource defined earlier.
    // We then map through each data, which is basically an element with a structure of `Amiibo` then do some custom rendering.
    let character_series_view = move || {
        character_series.read(cx).map(|data| {
            data.map(|data| {
                data.into_iter()
                    .map(|s| view! { cx, <li>{s.gameSeries}</li> })
                    .collect_view(cx)
            })
        })
    };

    // * Uncomment the code below for another example.

    // let character_series_view = move || {
    //     character_series.read(cx).map(|data| {
    //         data.map(|data| {
    //             data.into_iter()
    //                 .map(|s| view! { cx, <img class="p-2 mx-auto mt-6 border shadow" src={s.image} /> })
    //                 .collect_view(cx)
    //         })
    //     })
    // };

    view! { cx,
        <div class="max-w-2xl mx-auto mt-12">
            <h1 class="text-2xl font-bold">"Results"</h1>
            <ul>{character_series_view}</ul>
        </div>
    }
}
