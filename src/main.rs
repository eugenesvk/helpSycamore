use sycamore::prelude::*;

#[component] // Components are simply functions slapped with the #[component] attribute...
fn P0HelloWorld<G:Html>(cx:Scope) -> View<G> { // ... that take a argument of type Scope (a reactive scope) and only run once
  // Should be named PascalCase to let view! macro distinguish between regular elements and components
  view! { cx, p{"Hello, Component World!"} }
}

#[component]
fn B1View<G:Html>(cx:Scope) -> View<G> {
  let my_number = 123; // Anything with std::fmt::Display will be autoinserted as text into the DOM
  let user_input = "aaa";
  let inner_view = view! { cx, "Inside inner_view" }; // views can also be interpolated
  let attr_view = view! { cx, // set attributes (including classes and ids)
    p(class="my-class", id="my-paragraph", aria-label="My paragraph"){"my-class/par/aria-label"}
    button(disabled=true) {"Disabled button"} };
  let props = view! { cx, input(type="checkbox", prop:indeterminate=true) };
  let event = view! { cx, button(on:click=|_| { /* do something */ }) {"Click me"} };
  let fragments = view! { cx,
    p { "First child" }
    p { "Second child" }
  }; // views can also be fragments. You can create as many nodes as you want at the top-level.

  // Builder syntax (skipped): For those who dislike macro DSLs, we also provide an ergonomic builder API for constructing views. Add the builder prelude as well as the main sycamore prelude to your source file

  view! { cx, // Write your markup inside the view! macro and get a View expression
  div              	{"A simple div"}
  div(class="foo") 	{"A div with a class"}
  p                	// An empty paragraph
  my-custom-element	{"Custom elements!"}
  "Text nodes: string literal!"
  div {p {span{"Span Nested in p in a "} strong{"div!"}}}
  p{"Interpolated number: " (my_number)}
  p{"Interpolated view: " (inner_view)}
  p{"View with attributes: " (attr_view)}
  div(dangerously_set_inner_html="<span>Inner HTML set by dangerously_set_inner_html!</span>")
  div(dangerously_set_inner_html=user_input) // DO NOT DO THIS!!! XSS (Cross-Site Scripting) vulnerability
  p{"View with indeterminate attribute set as a prop:* directive: " (props)}
  p{"View with and event using the on:* directive : " (event)}
  p{"View with two fragmets : " (fragments)}
}}

fn main() { // Trunk automatically uses fn main as your project's entrypoint, no need for any #[wasm_bindgen(start)] here
  sycamore::render( // ← provided by Sycamore to render your app to the DOM (browser window)
    // render accepts a closure (lambda function) which should return a view to be rendered
    |cx| view! { // The view! macro allows creating complex user interfaces ergonomically in HTML. In this case, we want to render the following HTML: <p>Hello World!</p>
      cx, // The cx variable represents the reactive scope. This is what basically keeps track of resources and makes reactivity work
      p { // The p { ... } creates a new <p> tag
        "Hello, World!" // creates a new text node that is nested within the <p> tag
      } // Trunk just needs one thing to turn this into a website: a html source file to inject the view into
      P0HelloWorld() // source the component declared above
      B1View{}
  });
}
