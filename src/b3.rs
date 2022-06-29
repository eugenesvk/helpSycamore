use sycamore::prelude::*;

#[derive(Prop)]
pub struct MyProps {
  name 	: String,
  email	: String,
}
#[component]
pub fn Component<'a, G:Html>(cx:Scope, p:MyProps) -> View<G> { // see P0HelloWorld on the basics
  // Properties. Allow accepting and rendering data from the parent.
  // To allow your component to accept properties, take a second argument with a type that implements the Prop trait. For convenience, you can automatically derive the Prop trait with a derive-macro
  let name  = p.name; 	//
  let email = p.email;	//
  let view1 = view! {cx, "Got properties in a MyProps struct with name `" (name) "` and email `" (email) "`"};

  view! {cx, p{(view1)} }
}

#[derive(Prop)]
struct MyPropsReactData<'a> { value:&'a ReadSignal<i32> }
#[component]
fn ComponentReactData<'a, G:Html>(cx:Scope<'a>, p:MyPropsReactData<'a>) -> View<G> {
  // Reactive Properties
  // For components to automatically react to prop changes, they should accept a signal. Most of the times, you’ll want a &ReadSignal unless you want mutable access to the data in which case you should use a &Signal. This way, updating the signal will automatically update whatever is listening to it, even if it is inside the child component

  let view1 = view! {cx, div(class="my-component") {
	  "@B3Component: Got properties in a MyPropsReactData struct with value:&ReadSignal `" (p.value.get()) }};
  view! {cx, p{(view1)} }
}
#[component]
pub fn ComponentReactDataTest<G:Html>(cx:Scope) -> View<G> {
  let state = create_signal(cx, 0);

  let view1 = view! {cx, ComponentReactData { value:state }};
  let view2 = view!{cx, button(on:click=|_| { state.set(2) }) {"@B3ComponentReactDataTest: Click me to set state that was passed to B3ComponentReactData =2"} };
  let view3 = view!{cx, p };
  let view4 = view!{cx, p };
  view! {cx, p{(view1)} p{(view2)} p{(view3)} p{(view4)}}

  // Component lifecycle is strongly tied to the reactive system, since, under the hood, components are simply functions that are run inside an untracked scope.
  // This means that we can use the same helpers in the reactive system to attach callbacks to the component lifecycle.

  // on_cleanup method schedules a callback to be called when the reactive scope is destroyed. This can also be used to schedule a callback when the component is destroyed.
    // on_cleanup(cx, || { /* Perform cleanup. */    });
}
