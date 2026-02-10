use leptos::prelude::*;

#[component]
pub fn Title(headline: &'static str, green_text: &'static str, first_describe: &'static str, second_describe: &'static str) -> impl IntoView {
    view! {
        <h2 class="text-5xl mb-5 font-extrabold leading-tight text-center">
            {headline}<br/>
            <span class="text-green-400">{green_text}</span>?
        </h2>
        <div class="text-center">
            <span class="block text-gray-400">
               {first_describe}
            </span>
            <strong>
               {second_describe}
            </strong>
        </div>
    }
}