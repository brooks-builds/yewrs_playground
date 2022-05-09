use std::env;

use yew::prelude::*;

const BASE_URI: &str = env!("BASE_URI");

#[function_component(Stripe)]
pub fn stripe() -> Html {
    let checkout_uri = env!("STRIPE_CHECKOUT_URI");
    let cancel_uri = format!("{BASE_URI}/stripe/cancel");
    let success_uri = format!("{BASE_URI}/stripe/success");

    html! {
      <div>
        <h1>{"Stripe"}</h1>
        <form action={checkout_uri} method="POST">
          <input type="text" name="cancel_uri" value={cancel_uri} disabled={true} />
          <input type="text" name="success_uri" value={success_uri} disabled={true} />
          <button>{"Let's go to stripe!"}</button>
        </form>
      </div>
    }
}
