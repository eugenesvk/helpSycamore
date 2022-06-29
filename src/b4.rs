use sycamore::prelude::*;

#[component]
pub fn ControlFlow<'a, G:Html>(cx:Scope) -> View<G> {
  // Control flow can be achieved using the interpolation syntax

  let visible = create_signal(cx, true);
  let v_btn = view!{cx, button(on:click=|_| {
    if *visible.get()	{ visible.set(false) }
    else             	{ visible.set(true ) } }
  ) {"Click me to toggle Visible signal"} };
  let view1 = view!{cx, div {
    (if *visible.get()	{ view!{cx, "@b4::ControlFlow Now you see me" }
    } else            	{ view!{cx, }  }) // Now you don't
    } // Since the interpolated value subscribes to visible, the content inside the if else will be toggled when visible is changed
  };

  // more complex conditions: show the value of `name` when it is non-empty, otherwise show "World"
    // Selector creates a memoized value from some signals, but unlike Memo will not notify dependents of a change if the output is the same
    // It takes a comparison function to compare the old and new value, which returns `true` if they are the same and `false` otherwise
  let name = create_signal(cx, String::new());
  let handle_change = move |_| name.set("a".to_string());
  let vinput = view!{cx, div { p {"Enter anything to set name to 'a' (only works once)"}
    input(placeholder="Enter", on:input=handle_change)
  }};
  let view2 = view!{cx, h1 {
    // Selector to memoize the value of name.get().is_empty() and NOT recreate the  inner view! (inside the if block) every time name is changed, only when name becomes non-empty
    (if *create_selector(cx, || !name.get().is_empty()).get() {
            view!{cx, span { (name.get()) } } }
     else { view!{cx, span { "Name is empty" } } })
  }};
  view!{cx, p{(v_btn)} p{(view1)} p{(vinput)} p{(view2)} }
}
