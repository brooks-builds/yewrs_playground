use graphql_client::{GraphQLQuery, Response};
use yew::prelude::*;

use crate::graphql_queries::{get_games, GetGames};

// Using https://crates.io/crates/graphql_client as the graphql library

#[function_component(GraphQL)]
pub fn graphql() -> Html {
    let games_state = use_state(|| vec![]);
    {
        let games_state = games_state.clone();
        use_effect(|| {
            if games_state.is_empty() {
                wasm_bindgen_futures::spawn_local(async move {
                    let variables = get_games::Variables;
                    let games_query = GetGames::build_query(variables);
                    let response =
                        gloo::net::http::Request::post("http://localhost:8000/v1/graphql")
                            .header("x-hasura-admin-secret", "keyboardxilbe")
                            .json(&games_query)
                            .unwrap()
                            .send()
                            .await
                            .unwrap()
                            .json::<Response<get_games::ResponseData>>()
                            .await
                            .unwrap();
                    let mut games = response.data.unwrap().games;
                    games_state.set(games);
                });
            }

            || {}
        });
    }

    html! {
      <div>
        <h1>{"GraphQL"}</h1>
        <ul>
          {
            (*games_state).iter().map(|game| {
              html! { <li key={game.id}>{game.name.clone().unwrap_or_default()}</li>}
            }).collect::<Html>()
          }
        </ul>
      </div>
    }
}
