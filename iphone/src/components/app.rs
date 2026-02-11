use super::prelude::*;
use leptos::prelude::*;

#[component]
pub fn AppCSR() -> impl IntoView {
    view! {}
}

#[component]
pub fn AppSSR() -> impl IntoView {
    view! {
       <HeroSection/>
       <FeaturesSection/>
       <FinalCTASection/>
    }
}
