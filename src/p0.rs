use sycamore::prelude::*;

#[component] // Components are simply functions slapped with the #[component] attribute...
pub fn HelloWorld<G:Html>(cx:Scope) -> View<G> { // ... that take a argument of type Scope (a reactive scope) and only run once
  // Should be named PascalCase to let view! macro distinguish between regular elements and components
  view! { cx, p{"Hello, Component World!"} }
}
