use std::fmt;

use sycamore::prelude::*;

// A good habit to use the new type idiom (doc.rust-lang.org/rust-by-example/generics/new_types.html) when describing the type of the data to be passed

// 0. Set types: create a global dark mode state. We can define the following DarkMode struct
#[derive(Clone, Copy, PartialEq, Eq)]
struct DarkMode(bool);
impl DarkMode { fn is_enabled(self) -> bool { self.0 } }
impl fmt::Display for DarkMode {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "DarkModeFmt: {}", self.0)
  }
}

#[component]
fn ChildComponent<G: Html>(cx: Scope) -> View<G> {
  let dark_mode = use_context::<Signal<DarkMode>>(cx);
  view!{cx, p(class="value")
    { "@ChildComponent dark_mode.get: " (dark_mode.get()) }
  }
}
#[component]
pub fn Context<'a, G:Html>(cx:Scope) -> View<G> {
  // An easy way to share data between components without drilling props through multiple levels of the component hierarchy

  // 1. Provide: to make a context value accessible, we need to use the `provide_context_ref` method. Since we want the context value to be reactive, we actually want a `Signal<DarkMode>` to be provided
  let dark_mode = create_signal(cx, DarkMode(false));
  provide_context_ref(cx, dark_mode);

  // `provide_context` is a wrapper around `create_ref` and `provide_context_ref` (to provide a value)
  let value = 123;
  let value_ref = create_ref(cx, value); provide_context_ref(cx, value_ref); // same as ↓
  //                                     provide_context(    cx, value);

  // 2. Use: can be used in any nested scope (including the scope where the context value was provided) via `use_context` method
  let view1 = view!{cx, p{"@Context component, providing a context ref for `dark_mode` (false) and then showing ChildComponent that uses this context ref"} };
  let view2 = view!{cx, ChildComponent{} };
  let view3 = view!{cx, p };
  let view4 = view!{cx, p };

  // NB!: unlike contexts in React, the context is not reactive by itself since components only run once. To make a context value reactive, you need to use a Signal or other reactive data structure

  view! {cx, p{(view1)} p{(view2)} p{(view3)} p{(view4)}}
}
