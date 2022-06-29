use sycamore::prelude::*;

#[component]
pub fn DataBinding<'a, G:Html>(cx:Scope) -> View<G> {
  // You can bind your Signal to a DOM property with the `bind:` directive. DOM property update → Signal update

  let sig_val = create_signal(cx, String::new());
  let view1 = view!{cx, input(placeholder="bind:value=sig_val", bind:value=sig_val) };
    // when the user types into the input, the value signal will automatically be synced by listening to specific events on the DOM node according to the property. For instance, value uses the on:input event.
  // Supported properties and events that are listened to (github.com/sycamore-rs/sycamore/blob/99ca4f63f1e560dab8ae090afd3dc2eec52264de/packages/sycamore-macro/src/view/codegen.rs#L388)
    // Property	Event name	Signal type
    // value   	input     	String
    // checked 	change    	bool

  let view2 = view!{cx, "sig_val.get= " (sig_val.get()) };
  let view3 = view!{cx, p };
  let view4 = view!{cx, p };

  view! {cx, p{(view1)} p{(view2)} p{(view3)} p{(view4)}}
}
