use std::time::Duration;

use sycamore::easing;
use sycamore::motion::create_tweened_signal;
use sycamore::prelude::*;

#[component]
pub fn Tweened<'a, G:Html>(cx:Scope) -> View<G> {
  // Tweened states update their values over a period of time, e.g. interpolate a value from 0 to 100 over a period of 5s250ms
  let tweened = create_tweened_signal(cx, 0.0f32, Duration::from_millis(5250), easing::quad_out);
  tweened.set(100.0);

  let view1 = view!{cx, "tweened: " (tweened.get()) };
  let view2 = view!{cx, p };
  let view3 = view!{cx, p };
  let view4 = view!{cx, p };

  view! {cx, p{(view1)} p{(view2)} p{(view3)} p{(view4)}}
}
