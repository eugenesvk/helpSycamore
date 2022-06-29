use sycamore::prelude::*;

#[component]
pub fn NodeRef<'a, G:Html>(cx:Scope) -> View<G> {
  // NodeRef to reference a node in the DOM directly created `create_node_ref`
  // Assigned to a node using the `ref` property in the `view!` macro

  let node_ref = create_node_ref(cx);
  let view1 = view!{cx, p(ref=node_ref) { "ref=node_ref Hello World!" } };
    // node_ref is now a reference to the p node
  // node_ref.get::<DomNode>() // access the raw node using the .get() method on NodeRef
  let view2 = view!{cx, p };
  let view3 = view!{cx, p };
  let view4 = view!{cx, p };

  view! {cx, p{(view1)} p{(view2)} p{(view3)} p{(view4)}}
}
