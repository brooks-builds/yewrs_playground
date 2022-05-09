use yew::prelude::*;

#[function_component(StripeSuccess)]
pub fn stripe_success() -> Hmtl {
    html! {
      <div>
        <h1>{"Your Payment was Successful!"}</h1>
      </div>
    }
}
