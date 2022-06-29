use sycamore::prelude::*;

#[component]
pub fn Iteration<'a, G:Html>(cx:Scope) -> View<G> {
  // Sycamore uses components for rendering lists. This is to prevent recreating DOM nodes every time the list changes. The components serve as wrappers to make rendering lists more ergonomic

  // 1. Keyed component renders a list of items with a key
    // a key for each item is used to identify the item to prevent re-rendering views twice
    // every time the list changes, a diffing algorithm is used to determine which items need to be re-rendered according to the associated key
  let count1 = create_signal(cx, vec![1, 2]);
  let view1 = view!{cx, h3{"Keyed"} ul {
    Keyed {
      iterable  	: count1    	, // &'a ReadSignal<Vec<T>>,
      view      	: |cx, el|  	  // map function that renders ...
        view!{cx	, li{(el)} }	, // ... a [`View`] for each element in `iterable`
      key       	: |k| *k    	, // key function that assigns each element in `iterable` an unique key
    }
  }};

  // 2. Indexed component renders a list of items that is keyed by index (generally less efficient)
  let count2 = create_signal(cx, vec![1, 2]);
  let view2 = view!{cx, h3{"Indexed"} ul {
    Indexed {
      iterable  	: count2    	, // &'a ReadSignal<Vec<T>>,
      view      	: |cx, el|  	  // map function that renders ...
        view!{cx	, li{(el)} }	, // ... a [`View`] for each element in `iterable`
    }
  }};

  // 3. .iter().map() from std to render a static list (that never changes)
    // if NOT static, single node will be re-rendered every time the list changes!!!
  let count = vec![1, 2];
  let views = View::new_fragment(
    count.iter().map(|&x| view!{cx, li {(x)} }).collect()
  );
  let view3 = view!{cx, h3{".iter().map()"} ul {
    (views)
  }};

  view!{cx, p{(view1)} p{(view2)} p{(view3)} }
}
