use leptos::prelude::*;

use crate::components::prelude::*;
use leptos::wasm_bindgen::{JsCast, closure::Closure};
use leptos::web_sys::Event;

#[component]
pub fn AppCSR() -> impl IntoView {
    let is_scrolling = RwSignal::new(false);

    Effect::new(move |_| {
        let document = document();

        let setup_scroll = |trigger_id: &'static str, target_id: &'static str| {
            let el = document.get_element_by_id(trigger_id);
            let target_el = document.get_element_by_id(target_id);

            if let (Some(trigger), Some(_)) = (el, target_el) {
                let doc = document.clone();
                
                let closure = Closure::<dyn FnMut(Event)>::new(move |_e: Event| {
                    if is_scrolling.get() { return; }

                    if let Some(target) = doc.get_element_by_id(target_id) {
                        is_scrolling.set(true);
                        target.scroll_into_view();

                        set_timeout(move || {
                            is_scrolling.set(false);
                        }, std::time::Duration::from_millis(100));
                    }
                });

                trigger
                    .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                    .unwrap();
                
                closure.forget();
            }
        };

        setup_scroll("to-benefits", "benefits");
        setup_scroll("to-how-use", "how-use");
        setup_scroll("to-product", "product");
        setup_scroll("to-FAQ", "faq");
    });

    view! {}
}

#[component]
pub fn AppSSR() -> impl IntoView {
    view! {
       <Navbar />
       <Card />
       <Benefits />
       <HowUse />
       <ProductTechnical />
       <FAQSection />
       <FinalCTA />
       <Footer />
    }
}
