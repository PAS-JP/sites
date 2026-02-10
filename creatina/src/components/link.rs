use leptos::prelude::*;

#[component]
pub fn Link(children: Children) -> impl IntoView {
    view! {
        <a
            href="https://mercadolivre.com/sec/2z7u4or"
            target="_blank"
            class="bg-linear-to-r from-emerald-500 to-teal-400 px-4 py-2.5 font-bold text-slate-950 rounded-2xl hover:opacity-70"
        >
            {children()}
        </a>
    }
}
