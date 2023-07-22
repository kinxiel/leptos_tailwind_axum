# OWDDM Leptos Example

## Introduction

This is a very basic example of a Leptos app presented in OWDDM meetup on July 2023. It uses a template that includes `Tailwindcss` for the styling and `Axum` as the server. Viewers already familiar with Rust can jump right into the examples and figure it out from there. However, since one of the motivations of this talk was to introduce some ways Rust is used in web development, there are detailed instructions on how to run the example even without previous Rust knowledge. Also included are some detailed explanations and instructions on how to run the examples. Although this isn't a material to learn Rust (the [book](https://doc.rust-lang.org/book/) is used for that), we do offer some helpful information on how the examples work.

Clone this repository and follow the steps below to get started.

## Environment setup

> If you already use Rust, skip this part.

1. Install Rust by following the instructions [here](https://www.rust-lang.org/tools/install). You can do this through your terminal.
2. Once you have Rust installed, it will make `cargo` package manager available (like `npm`). We need to install a few other things to work with web assembly. Add the wasm target by typing the following in your terminal:

```shell
rustup target add wasm32-unknown-unknown
```

3. Leptos uses `nightly` Rust (which is not the stable version of Rust), so we need to install and switch to that version. Run the following:

```shell
rustup toolchain install nightly
rustup default nightly
```

> If you need to go back to stable later on, simply run `rustup default stable`.

4. Now we need to install Leptos and some of its tools. We will install `leptos` as a dependency later on, but we need to install `cargo-leptos`, a tool that manages the Leptos backend.

```
cargo install cargo-leptos
```

## Starting the project

Once everything is installed, navigate to the root of the repository. You will find a bunch of config files here. `package.json` is used for Tailwindcss, so go ahead and install it.

```shell
npm install
```

then

```shell
npm run watch
```

You will also find `Cargo.toml` which is like `package.json` but for Rust. The dependencies will be installed once we start the app. In another terminal, you can start the app with:

```
cargo leptos watch
```

> You need to wait a while for the dependencies to be installed. After the dependencies have been installed, head over to localhost:3000 in your favorite major browser to see the home page.

## Examples

You can find all the examples in `src/app.rs`. Each example is available as a separate route, so you can check each example by changing the URL. To reduce the number of examples, I tried to combine some concepts into a single example to show how they work together.

### Basic example

The app should open in the root route. This is a simple counter with some additional stuff going on. As mentioned earlier, each example is accessible via separate routes. Leptos includes a built in `Router` component that is used like the below example.

```rust
<Route
    path=""
    view=move |cx| {
        view! { cx, <Home/> }
    }
/>
```

Basically the `path` is the URL path and the component you want to display on that route is passed to the `view!` macro. In this case `<Home />`. The `view!` macro basically allows you to write a JSX like syntax and takes care of creating the appropriate Rust code for you.

The first example consists of 3 different parts.
- You have a basic counter which demonstrates how signals and signal setters work.
- There is a `<ProgressBar />` component which demonstrates how props are used in Leptos.
- Then you have the `<ItsMeMario />` component which demonstrates how to work with signals and some particularities of working with different signal types.

> Detailed explanation can be found in the code comments.

Some additional explanation on working with `<ItsMeMario />` component. In this part of the code, we show how to handle derived signals. For the most common use case, which is explained in the `<ProgressBar />` component, assigning an attribute directly to the signal value of type `<ReadSignal>` just works. However, when working with derived signals the type signature changes so we need to do some adjustments.

First try to break the app, by uncommenting the following line

```rust
// <ItsMeMario width= pixel_value />
```

You should get the following compiler error

```
expected `ReadSignal<i32>`, found closure
```

This basically means that the `width` attribute is expecting something of type `<ReadSignal<i32>`, which `pixel_value` is not. `pixel_value` is a closure type. So obviously they are not the same. There are various ways to solve this problem.

First solution. We can use a generic type and tell it to accept a closure type. Comment out the `Naive Mario` component and uncomment the `Generic Mario` component, this should fix the problem.

Second solution. We can use special `#[prop(into)] width: Signal<i32>` syntax on width. This implements the `into` trait for the value which satisfies the requirements. A `Signal` is a more generic version of a signal. The trade-off though is that you need to do some additional work on the parent component. Comment out `<ItsMeMario width= pixel_value />` and uncomment `<ItsMeMario width= Signal::derive(cx, pixel_value) />`. The app should work like before.

### Example 2: Text Input

Navigate to:

> http://localhost:3000/text_input

The text input value (a reactive signal) is bound to the display text. Changing the input value will trigger a state change. Also in this example we explore how to do some conditional class rendering. If you type `blue` in the input, the background of the text should turn blue. We basically have a boolean toggle that adds or removes a class based on the text value.

> Read the comments in the code for details.

### Example 3: Control Flow

Navigate to:

> http://localhost:3000/control_flow

This is simple example of a calendar input. Depending on the date, we return whether it is a weekday or weekend using some simple logic. The point in this example is to show that control flow using something like `if` and `else` is done using normal Rust code, no special syntax necessary.

> Read the comments in the code for details.

### Example 4: Parent Child Communication

Navigate to:

> http://localhost:3000/parent_child_communication

In this example we basically have a parent component with several child components and we want the event triggered by the child component to affect something in the parent component.

We basically setup 3 different signals which are booleans.

```rust
    let (object_cover, set_object_cover) = create_signal(cx, false);
    let (object_scale_down, set_object_scale_down) = create_signal(cx, false);
    let (object_contain, set_object_contain) = create_signal(cx, false);
```

Then on the parent component's `img` attribute we set several conditional classes.

```rust
class=("object-cover", object_cover)
class=("object-scale-down", object_scale_down)
class=("object-contain", object_contain)
```

So basically, clicking on each child should toggle each of these classes on the parent.

There are several methods to achieve this:

- Pass signal to the child
- Set an event listener on a component. Basically the component becomes a simple placeholder.
- Use `provide context` and `use_context` to facilitate parent-child communication

> Read the comments in the code for details.

### Example 5: Pass Children in Parent

Navigate to:

> http://localhost:3000/pass_children

In this example we have a parent component that accepts some `children` and we want some way to let the parent render the `children`.

```rust
<AcceptsChildren>
    <p>"Item 1"</p>
    <p>"Item 2"</p>
</AcceptsChildren>
```

This can be achieved by using Leptos's `Children` type. You can see the code to see this in action.

> Read the comments in the code for details.

### Example 6: Fetch

Navigate to:

> http://localhost:3000/fetch

Here we are using a publicly available API (Amiibo API) to fetch some data. The process is rather straightforward.

We first need to create a `struct` (or object) that matches the structure of the data returned by the API.

```rust
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
    amiibo: Vec<Amiibo>,
}
```

You can check the API url for the actual structure of the data, but basically we have data in JSON format, where the first element is an array called `amiibo` which contains a couple of entries for the data, each with a pattern like our struct `Amiibo`.

> Read the comments in the code for details.