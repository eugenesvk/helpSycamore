use sycamore::prelude::*;

#[component] // Components are simply functions slapped with the #[component] attribute...
fn P0HelloWorld<G:Html>(cx:Scope) -> View<G> { // ... that take a argument of type Scope (a reactive scope) and only run once
  // Should be named PascalCase to let view! macro distinguish between regular elements and components
  view! { cx, p{"Hello, Component World!"} }
}

fn main() { // Trunk automatically uses fn main as your project's entrypoint, no need for any #[wasm_bindgen(start)] here
  sycamore::render( // ← provided by Sycamore to render your app to the DOM (browser window)
    // render accepts a closure (lambda function) which should return a view to be rendered
    |cx| view! { // The view! macro allows creating complex user interfaces ergonomically in HTML. In this case, we want to render the following HTML: <p>Hello World!</p>
      cx, // The cx variable represents the reactive scope. This is what basically keeps track of resources and makes reactivity work
      p { // The p { ... } creates a new <p> tag
        "Hello, World!" // creates a new text node that is nested within the <p> tag
      } // Trunk just needs one thing to turn this into a website: a html source file to inject the view into
      P0HelloWorld() // source the component declared above
  });
}
