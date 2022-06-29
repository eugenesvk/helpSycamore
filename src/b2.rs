use sycamore::prelude::*;
use log::{Level, info};

#[component] #[allow(dead_code)]
pub fn React<G:Html>(cx:Scope) -> View<G> { // fine-grained reactivity to keep the DOM and state in sync
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
