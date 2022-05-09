use yew::prelude::*;

#[function_component(StripeCancel)]
pub fn stripe_cancel() -> Html {
    html! {
      <div>
        <h1>{"You cancelled your Stripe payment, have a nice day!"}</h1>
      </div>
    }
}
