use yew::prelude::*;

// Using https://crates.io/crates/graphql_client as the graphql library

#[function_component(GraphQL)]
pub fn graphql() -> Html {
    html! {
      <div>
        <h1>{"GraphQL"}</h1>
      </div>
    }
}
