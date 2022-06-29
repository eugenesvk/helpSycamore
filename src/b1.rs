use sycamore::prelude::*;

#[component]
pub fn View<G:Html>(cx:Scope) -> View<G> {
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
