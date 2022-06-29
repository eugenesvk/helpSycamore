use sycamore::prelude::*;
use log::{Level, info};

#[allow(dead_code)]
#[component] // Components are simply functions slapped with the #[component] attribute...
fn P0HelloWorld<G:Html>(cx:Scope) -> View<G> { // ... that take a argument of type Scope (a reactive scope) and only run once
  // Should be named PascalCase to let view! macro distinguish between regular elements and components
  view! { cx, p{"Hello, Component World!"} }
}

#[component] #[allow(dead_code)]
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

#[component] #[allow(dead_code)]
fn B2React<G:Html>(cx:Scope) -> View<G> { // fine-grained reactivity to keep the DOM and state in sync
  // Reactive scopes: a MUST; provided by functions such as sycamore::render/render_to as an argument to the render closure ('cx' in sycamore::render(|cx| {});

  // Signal. Reactivity is based on reactive primitives, e.g. Signal (a wrapper around a type that can be read and written to and which can be listened on whenever its wrapped value is mutated)
  let state = create_signal(cx, 0);	// 1. Create a reactive atom with an initial value of `0`
    // retval of type &Signal, not Signal, since it's allocated on the reactive scope and therefore has its lifetime tied with the scope. Plus allows Rust’s lifetime system to make sure signals are not accessed once its enclosing scope has been destroyed
  let state0 = state.get(); // 2. get Signal's value (`0`)
  state.set(1);            	// 3. set Signal's value
  let state1 = state.get();	// should now be `1`
  let view1 = view! {cx, "The state was: " (state0) };
  let view2 = view! {cx, "The state  is: " (state1) };

  // Effects listen on whenever its wrapped value is mutated. Runs at least once
    // create_effect creates a new “listener scope” (not to be confused with reactive scope) and calling state.get() inside this listener scope adds itself as a dependency. Now, when state.set(...) is called, it automatically calls all its dependents. In this case, whenever state is updated, the new value will be printed!
  console_error_panic_hook::set_once();
  console_log::init_with_level(Level::Debug).unwrap();
  // create_effect(cx, || println!("The state changed. New value: {}", state.get()));
  create_effect(cx, || info!("The state changed. New value: {}", state.get()));
  // let s = "It works!"; info!("{}", s);
  state.set(2);	// 3. set Signal's value to `2` and watch the effect run with `2`

  // Derived signals (functions referencing signals)
    // lazy
    // don't keep track of the result of the computation
    // => the computation will not be executed until needed
    // => calling the derived signal twice will result in the same computation twice
    // let double	= || *state.get() * 2; // create Derived signal
    // let _ = double(); // closure named double is called again (wasted work)

  // Memo. Create a derived state (aka derive stores), memoized computation from some signals
    // eagerly evaluated
    // will only run the computation when one of its dependencies change
    // incur a slightly higher performance penalty than simple derived signals

  let state_single	= create_signal(cx, 0);
  let memo_dbl    	= create_memo(cx, || *state_single.get() * 2);
  info!("*memo_dbl.get() when state_single=0: {}", *memo_dbl.get()); assert_eq!(*memo_dbl.get(),0);
  state_single.set(1);
  info!("*memo_dbl.get() when state_single=1: {}", *memo_dbl.get()); assert_eq!(*memo_dbl.get(),2);

  // Reactivity is automatically built-in into the view! macro
  let view_state = create_signal(cx, 0);
  let view3 = view!{cx, "The view_state is: " (view_state.get()) }; // equivalent to ↓
    // let element	= GenericNode::element(p);
    // let text   	= GenericNode::text(String::new() /* placeholder */);
    // create_effect(cx, move || { text.update_text(Some(&state.get()));}); // Update text when `state` changes
    // element.append(&text);
    // element
  let view4 = view!{cx, button(on:click=|_| { view_state.set(2) }) {"Click me to set view_state=2"} };
  // view_state.set(2);// autoupdates the view3

  // Common pitfalls. 1. Dependency tracking is topological, which means that reactive dependencies (like a Signal) must be accessed (and thus recorded as reactive dependencies) before the listener scope (like the one in a create_effect) returns.
  let _pitfall = create_effect(cx, move || {
	  // 2. Signals we track in the create_effect won’t be tracked properly in the wasm_bindgen_futures::spawn_local
    state.track(); // 2. fix: accessing reactive dependencies as needed before going into a future (same as calling `.get()` but without returning a value)
    // wasm_bindgen_futures::spawn_local(async move {}) // ← 1. scope not tracked because spawn_local runs on the next microtask tick (some times later) once the effect closure has returned already
    // 1. Everything accessed until here is tracked. Once this closure returns, nothing is tracked
  });

  view! {cx, p{(view1)} p{(view2)} p{(view3)} p{(view4)}}
}

#[derive(Prop)] #[allow(dead_code)]
struct MyProps {
  name 	: String,
  email	: String,
}
#[component] #[allow(dead_code)]
fn B3Component<'a, G:Html>(cx:Scope, p:MyProps) -> View<G> { // see P0HelloWorld on the basics
  // Properties. Allow accepting and rendering data from the parent.
  // To allow your component to accept properties, take a second argument with a type that implements the Prop trait. For convenience, you can automatically derive the Prop trait with a derive-macro
  let name  = p.name; 	//
  let email = p.email;	//
  let view1 = view! {cx, "Got properties in a MyProps struct with name `" (name) "` and email `" (email) "`"};

  view! {cx, p{(view1)} }
}

#[derive(Prop)] #[allow(dead_code)]
struct MyPropsReactData<'a> { value:&'a ReadSignal<i32> }
#[component] #[allow(dead_code)]
fn B3ComponentReactData<'a, G:Html>(cx:Scope<'a>, p:MyPropsReactData<'a>) -> View<G> {
  // Reactive Properties
  // For components to automatically react to prop changes, they should accept a signal. Most of the times, you’ll want a &ReadSignal unless you want mutable access to the data in which case you should use a &Signal. This way, updating the signal will automatically update whatever is listening to it, even if it is inside the child component

  let view1 = view! {cx, div(class="my-component") {
	  "@B3Component: Got properties in a MyPropsReactData struct with value:&ReadSignal `" (p.value.get()) }};
  view! {cx, p{(view1)} }
}
#[component] #[allow(dead_code)]
fn B3ComponentReactDataTest<G:Html>(cx:Scope) -> View<G> {
  let state = create_signal(cx, 0);

  let view1 = view! {cx, B3ComponentReactData { value:state }};
  let view2 = view!{cx, button(on:click=|_| { state.set(2) }) {"@B3ComponentReactDataTest: Click me to set state that was passed to B3ComponentReactData =2"} };
  let view3 = view!{cx, p };
  let view4 = view!{cx, p };
  view! {cx, p{(view1)} p{(view2)} p{(view3)} p{(view4)}}

  // Component lifecycle is strongly tied to the reactive system, since, under the hood, components are simply functions that are run inside an untracked scope.
  // This means that we can use the same helpers in the reactive system to attach callbacks to the component lifecycle.

  // on_cleanup method schedules a callback to be called when the reactive scope is destroyed. This can also be used to schedule a callback when the component is destroyed.
    // on_cleanup(cx, || { /* Perform cleanup. */    });
}


fn main() { // Trunk automatically uses fn main as your project's entrypoint, no need for any #[wasm_bindgen(start)] here
  sycamore::render( // ← provided by Sycamore to render your app to the DOM (browser window)
    // render accepts a closure (lambda function) which should return a view to be rendered
    |cx| view! { // The view! macro allows creating complex user interfaces ergonomically in HTML. In this case, we want to render the following HTML: <p>Hello World!</p>
      cx, // The cx variable represents the reactive scope. This is what basically keeps track of resources and makes reactivity work
      p { // The p { ... } creates a new <p> tag
        "Hello, World!" // creates a new text node that is nested within the <p> tag
      } // Trunk just needs one thing to turn this into a website: a html source file to inject the view into
      // P0HelloWorld() // source the component declared above
      // B1View{}
      // B2React{}
      B3Component{name:"John Doe".to_string(), email:"john@doe.com".to_string()}
      B3ComponentReactDataTest{}
  });
}
