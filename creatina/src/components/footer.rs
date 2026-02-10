use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="mt-48 pb-12 px-4 border-t border-slate-900 bg-slate-950/20">
            <div class="max-w-4xl mx-auto pt-16 flex flex-col items-center text-center">
                
                <div class="space-y-4 max-w-2xl">
                    <p class="text-slate-500 text-sm leading-relaxed">
                        "Conteúdo informativo. Consulte um profissional de saúde antes de usar suplementos."
                    </p>
                    <p class="text-slate-600 text-xs font-medium uppercase tracking-widest">
                        "Não somos afiliados à Growth Supplements. Produto vendido através do Mercado Livre."
                    </p>
                </div>

                <div class="mt-12 pt-8 border-t border-slate-900/50 w-full">
                    <p class="text-slate-700 text-[10px] uppercase tracking-widest">
                        "© 2026 • Todos os direitos reservados"
                    </p>
                </div>
            </div>
        </footer>
    }
}