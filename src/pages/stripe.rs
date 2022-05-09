use std::env;

use yew::prelude::*;

#[function_component(Stripe)]
pub fn stripe() -> Html {
    let checkout_uri = env!("STRIPE_CHECKOUT_URI");
    html! {
      <div>
        <h1>{"Stripe"}</h1>
        <form action={checkout_uri} method="POST">
          <button>{"Let's go to stripe!"}</button>
        </form>
      </div>
    }
}
