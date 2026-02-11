use super::cta::*;
use leptos::prelude::*;

#[component]
pub fn FinalCTASection() -> impl IntoView {
    view! {
        <section class="w-full bg-gradient-to-b from-[#050816] to-black text-white py-24">
            <div class="max-w-6xl mx-auto px-6 space-y-20">

                // TRUST BADGES
                <div class="grid gap-6 md:grid-cols-3">
                    <TrustCard
                        icon="🛡"
                        title="Mercado Pago"
                        description="Seu dinheiro protegido até a entrega"
                    />
                    <TrustCard
                        icon="↩"
                        title="Devolução Grátis"
                        description="30 dias para devolver sem custo"
                    />
                    <TrustCard
                        icon="✔"
                        title="Vendedor Confiável"
                        description="Milhares de vendas com reputação verde"
                    />
                </div>

                // MAIN CTA CARD
                <div class="
                    relative overflow-hidden
                    rounded-3xl p-12 md:p-16
                    bg-white/5 border border-white/10
                    backdrop-blur-xl
                    text-center
                ">

                    // subtle background glow
                    <div class="absolute inset-0 bg-gradient-to-r from-indigo-500/5 via-purple-500/5 to-transparent pointer-events-none"></div>

                    <div class="relative space-y-8">

                        <div class="text-red-400 font-semibold tracking-wide text-sm">
                            "⚡ ESTOQUE QUASE ESGOTADO"
                        </div>

                        <h2 class="text-4xl md:text-5xl font-extrabold leading-tight">
                            "Não perca essa "
                            <span class="bg-gradient-to-r from-blue-500 via-indigo-500 to-purple-500 bg-clip-text text-transparent">
                                "oportunidade"
                            </span>
                        </h2>

                        <p class="text-gray-400 max-w-2xl mx-auto text-lg leading-relaxed">
                            "Milhares de pessoas já compraram. Garanta o seu iPhone 16 Pro Max
                            com o melhor preço e condições exclusivas."
                        </p>

                        <div class="pt-4">
                            <CTA>
                                "QUERO O MEU AGORA"
                            </CTA>
                        </div>

                        <p class="text-sm text-gray-500">
                            "Você será redirecionado para o Mercado Livre"
                        </p>
                    </div>
                </div>

                // DISCLAIMER
                <div class="text-center text-sm text-gray-600 leading-relaxed max-w-3xl mx-auto">
                    "Este site não é afiliado à Apple Inc. iPhone é marca registrada da Apple Inc.
                    Preços e disponibilidade sujeitos a alteração sem aviso prévio."
                </div>

            </div>
        </section>
    }
}

#[component]
fn TrustCard(icon: &'static str, title: &'static str, description: &'static str) -> impl IntoView {
    view! {
        <div class="
            p-6 rounded-2xl
            bg-white/5 border border-white/10
            backdrop-blur-md
            transition-all duration-300
            hover:-translate-y-1
            hover:border-indigo-500/40
            hover:shadow-[0_0_30px_rgba(99,102,241,0.2)]
        ">
            <div class="text-indigo-400 text-2xl mb-4">
                {icon}
            </div>

            <h3 class="font-semibold text-lg mb-2">
                {title}
            </h3>

            <p class="text-gray-400 text-sm leading-relaxed">
                {description}
            </p>
        </div>
    }
}
