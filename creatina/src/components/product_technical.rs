use leptos::prelude::*;

#[component]
pub fn ProductTechnical() -> impl IntoView {
    view! {
        <section id="product" class="pt-48 flex justify-center px-4">
            <div class="w-full max-w-6xl flex flex-wrap justify-between items-start gap-12">
                
                <div class="flex-1 min-w-[320px]">
                    <h2 class="text-5xl font-black mb-6 uppercase tracking-tighter">
                        "Ficha " <span class="text-emerald-400">"Técnica"</span>
                    </h2>
                    <p class="text-slate-400 mb-10 text-lg leading-relaxed max-w-md">
                        "Creatina monohidratada pura da Growth Supplements. Sem aditivos, sem corantes, sem saborizantes artificiais."
                    </p>

                    <div class="bg-slate-900/40 border border-slate-800 rounded-2xl overflow-hidden backdrop-blur-sm">
                        <TechnicalRow label="Produto" value="Creatina Monohidratada" />
                        <TechnicalRow label="Marca" value="Growth Supplements" />
                        <TechnicalRow label="Peso Líquido" value="250g" />
                        <TechnicalRow label="Porção" value="5g (1 colher medidora)" />
                        <TechnicalRow label="Rendimento" value="50 doses" />
                        <TechnicalRow label="Sabor" value="Sem sabor" />
                        <TechnicalRow label="Pureza" value="100% Creatina Monohidratada" last=true />
                    </div>
                </div>

                <div class="flex-1 min-w-[320px]">
                    <div class="group relative p-10 rounded-3xl bg-slate-900/50 border border-slate-800 shadow-2xl transition-all duration-500 hover:border-emerald-500/30">
                        <div class="absolute inset-0 bg-emerald-500/5 rounded-3xl opacity-0 group-hover:opacity-100 transition-opacity"></div>
                        
                        <div class="relative z-10 flex flex-col items-center text-center">
                            <span class="text-emerald-500 font-bold tracking-widest text-xs uppercase mb-4">"Produto Original"</span>
                            <h3 class="text-5xl font-black text-white mb-6 tracking-tight">"Creatina Growth 250g"</h3>
                            
                            <p class="text-slate-400 mb-10 text-lg">
                                "Compra 100% segura pelo Mercado Livre. Produto original com nota fiscal."
                            </p>

                            <a href="https://mercadolivre.com/sec/2z7u4or" target="_blank"
                               class="w-full py-6 bg-emerald-500 hover:bg-emerald-400 text-slate-950 font-black text-xl rounded-2xl shadow-[0_20px_50px_-15px_rgba(16,185,129,0.5)] transition-all flex items-center justify-center gap-3 hover:scale-[1.03] active:scale-[0.98]">
                                "Comprar Agora"
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" stroke-width="3" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M17 8l4 4m0 0l-4 4m4-4H3" />
                                </svg>
                            </a>

                            <div class="mt-10 flex flex-wrap justify-center gap-6 text-xs font-bold text-slate-500 uppercase tracking-widest">
                                <span class="flex items-center gap-2">"✓ Frete rápido"</span>
                                <span class="flex items-center gap-2">"✓ Mercado Pago"</span>
                                <span class="flex items-center gap-2">"✓ Garantia"</span>
                            </div>
                        </div>
                    </div>
                </div>

            </div>
        </section>
    }
}

#[component]
fn TechnicalRow(label: &'static str, value: &'static str, #[prop(default = false)] last: bool) -> impl IntoView {
    let border_class = if last { "" } else { "border-b border-slate-800/50" };
    view! {
        <div class=format!("flex justify-between items-center p-6 text-sm md:text-base hover:bg-white/5 transition-colors {}", border_class)>
            <span class="text-slate-400 font-medium tracking-wide">{label}</span>
            <span class="text-white font-black text-right">{value}</span>
        </div>
    }
}