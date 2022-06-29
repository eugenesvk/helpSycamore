#![allow(dead_code)]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use sycamore::prelude::*;

mod p0; // Hello world
mod b1; // Basic 1 view!
mod b2; // Basic 2 Reactivity
mod b3; // Basic 3 Component
mod b4; // Basic 4 Control Flow
mod b5; // Basic 5 Iteration
mod b6; // Basic 6 Data Binding
mod adv1; // Advanced 1 NodeRef
mod adv2; // Advanced 2 Tweened

fn main() { // Trunk automatically uses fn main as your project's entrypoint, no need for any #[wasm_bindgen(start)] here
  sycamore::render( // ← provided by Sycamore to render your app to the DOM (browser window)
    // render accepts a closure (lambda function) which should return a view to be rendered
    |cx| view! { // The view! macro allows creating complex user interfaces ergonomically in HTML. In this case, we want to render the following HTML: <p>Hello World!</p>
      cx, // The cx variable represents the reactive scope. This is what basically keeps track of resources and makes reactivity work
      p { // The p { ... } creates a new <p> tag
        "Hello, World!" // creates a new text node that is nested within the <p> tag
      } // Trunk just needs one thing to turn this into a website: a html source file to inject the view into
      // p0::HelloWorld() // source the component declared in P0.rs
      // b1::View{}
      // b2::React{}
      // b3::Component{name:"John Doe".to_string(), email:"john@doe.com".to_string()}
      // b3::ComponentReactDataTest{}
      // b4::ControlFlow{}
      // b5::Iteration{}
      // b6::DataBinding{}
      // adv1::NodeRef{}
      adv2::Tweened{}
  });
}
