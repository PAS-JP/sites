use leptos::prelude::*;

#[component]
pub fn FinalCTA() -> impl IntoView {
    view! {
        <section class="pt-48 pb-24 flex flex-col items-center px-4">
            <h2 class="text-6xl font-black text-white mb-6 text-center tracking-tighter">
                "Pronto para " <span class="text-emerald-400">"Evoluir?"</span>
            </h2>
            
            <p class="text-slate-400 text-xl text-center max-w-2xl mb-12 leading-relaxed">
                "Garanta sua Creatina Growth 250g original e leve seus treinos para o próximo nível."
            </p>

            <a href="https://mercadolivre.com/sec/2z7u4or" target="_blank"
               class="group relative flex items-center gap-3 bg-emerald-500 hover:bg-emerald-400 text-slate-950 font-black text-2xl py-6 px-12 rounded-2xl shadow-[0_0_50px_-10px_rgba(16,185,129,0.6)] transition-all hover:scale-105 active:scale-95 mb-24">
                "Comprar no Mercado Livre"
                <svg class="w-7 h-7 transition-transform group-hover:translate-x-1" fill="none" stroke="currentColor" stroke-width="3" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M17 8l4 4m0 0l-4 4m4-4H3" />
                </svg>
            </a>

            <div class="grid grid-cols-1 md:grid-cols-3 gap-12 w-full max-w-4xl">
                <TrustItem 
                    title="Compra Segura" 
                    subtitle="Mercado Pago"
                    icon=move || view! {
                        <path fill-rule="evenodd" d="M12.516 2.17a.75.75 0 0 0-1.032 0 11.209 11.209 0 0 1-7.877 3.08.75.75 0 0 0-.722.515A12.74 12.74 0 0 0 2.25 9.75c0 5.942 4.064 10.933 9.563 12.348a.749.749 0 0 0 .374 0c5.499-1.415 9.563-6.406 9.563-12.348 0-1.39-.223-2.73-.635-3.985a.75.75 0 0 0-.722-.516l-.143.001c-2.996 0-5.717-1.17-7.734-3.08Zm3.094 8.016a.75.75 0 1 0-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 0 0-1.06 1.06l2.25 2.25a.75.75 0 0 0 1.14-.094l3.75-5.25Z" clip-rule="evenodd" />
                    }
                />
                <TrustItem 
                    title="Entrega Rápida" 
                    subtitle="Para todo Brasil"
                    icon=move || view! {
                        <path d="M3.375 4.5C2.339 4.5 1.5 5.34 1.5 6.375V13.5h12V6.375c0-1.036-.84-1.875-1.875-1.875h-8.25ZM13.5 15h-12v2.625c0 1.035.84 1.875 1.875 1.875h.375a3 3 0 1 1 6 0h3a.75.75 0 0 0 .75-.75V15Z" />
                        <path d="M8.25 19.5a1.5 1.5 0 1 0-3 0 1.5 1.5 0 0 0 3 0ZM15.75 6.75a.75.75 0 0 0-.75.75v11.25c0 .087.015.17.042.248a3 3 0 0 1 5.958.464c.853-.175 1.522-.935 1.464-1.883a18.659 18.659 0 0 0-3.732-10.104 1.837 1.837 0 0 0-1.47-.725H15.75Z" />
                        <path d="M19.5 19.5a1.5 1.5 0 1 0-3 0 1.5 1.5 0 0 0 3 0Z" />
                    }
                />
                <TrustItem 
                    title="Parcelamento" 
                    subtitle="Em até 12x"
                    icon=move || view! {
                        <path d="M4.5 3.75a3 3 0 0 0-3 3v.75h21v-.75a3 3 0 0 0-3-3h-15Z" />
                        <path fill-rule="evenodd" d="M22.5 9.75h-21v7.5a3 3 0 0 0 3 3h15a3 3 0 0 0 3-3v-7.5Zm-18 3.75a.75.75 0 0 1 .75-.75h6a.75.75 0 0 1 0 1.5h-6a.75.75 0 0 1-.75-.75Zm.75 2.25a.75.75 0 0 0 0 1.5h3a.75.75 0 0 0 0-1.5h-3Z" clip-rule="evenodd" />
                    }
                />
            </div>
        </section>
    }
}

#[component]
fn TrustItem<F, IV>(title: &'static str, subtitle: &'static str, icon: F) -> impl IntoView 
where 
    F: Fn() -> IV + 'static,
    IV: IntoView
{
    view! {
        <div class="flex flex-col items-center group cursor-default">
            <div class="w-14 h-14 rounded-full bg-emerald-500/10 border border-emerald-500/20 flex items-center justify-center text-emerald-500 mb-4 transition-all duration-300 group-hover:bg-emerald-500/20 group-hover:scale-110 group-hover:border-emerald-500/40 shadow-[0_0_15px_rgba(16,185,129,0)] group-hover:shadow-[0_0_15px_rgba(16,185,129,0.2)]">
                <svg class="w-7 h-7" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                    {icon()}
                </svg>
            </div>
            <h2 class="text-white font-bold text-xl tracking-tight mb-1 transition-colors group-hover:text-emerald-400">{title}</h2>
            <p class="text-slate-500 text-sm font-medium uppercase tracking-widest">{subtitle}</p>
        </div>
    }
}