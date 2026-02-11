use super::cta::*;
use leptos::prelude::*;

#[component]
pub fn HeroSection() -> impl IntoView {
    view! {
        <section class="w-full min-h-screen bg-linear-to-br from-black via-[#0b1220] to-[#020617] text-white flex items-center">
            <div class="max-w-7xl mx-auto px-6 py-20 grid lg:grid-cols-2 gap-16 items-center">

                // LEFT SIDE
                <div class="space-y-8">

                    // Badge
                    <div class="inline-flex items-center gap-2 px-4 py-2 rounded-full
                                bg-red-500/10 border border-red-500/20 text-red-400 text-sm font-medium">
                        <span class="w-2 h-2 rounded-full bg-red-500 animate-pulse"></span>
                        "ÚLTIMAS UNIDADES — Oferta por tempo limitado!"
                    </div>

                    // Title
                    <div class="space-y-4">
                        <h1 class="text-5xl md:text-6xl font-extrabold tracking-tight">
                            "iPhone 16"
                            <br/>
                            "Pro Max"
                        </h1>

                        <h2 class="text-4xl md:text-5xl font-bold bg-linear-to-r from-blue-500 via-indigo-500 to-purple-500 bg-clip-text text-transparent">
                            "pelo menor preço do Brasil."
                        </h2>
                    </div>

                    // Description
                    <p class="text-gray-400 text-lg max-w-xl leading-relaxed">
                        "Produto "
                        <span class="font-semibold text-white">"100% original"</span>
                        " com nota fiscal e garantia Apple. Entrega expressa para todo o Brasil."
                    </p>

                    // Installments
                    <div class="inline-flex items-center gap-3 px-6 py-4 rounded-xl
                                bg-white/5 border border-white/10 backdrop-blur-md">
                        <span class="text-blue-400 text-xl">"⏱"</span>
                        <div class="text-sm text-gray-400">
                            "Parcele em até "
                            <span class="text-blue-400 font-semibold text-lg">
                                "18x sem juros"
                            </span>
                        </div>
                    </div>

                    // CTA
                    <CTA>
                        "COMPRAR AGORA NO MERCADO LIVRE"
                    </CTA>

                    // Footer Info
                    <div class="flex flex-wrap gap-6 text-sm text-gray-500 pt-4">
                        <span>"🛡 Compra 100% Segura"</span>
                        <span>"🚚 Frete Grátis"</span>
                        <span>"💳 Todos os cartões"</span>
                    </div>
                </div>

                // RIGHT SIDE
                <div class="relative flex justify-center">
                    <div class="absolute w-72 h-72 bg-purple-600/30 blur-[120px] rounded-full"></div>

                    <img
                        src="/public/assets/iphone.webp"
                        alt="iPhone 16 Pro Max"
                        class="relative w-[320px] md:w-[420px] drop-shadow-[0_40px_80px_rgba(99,102,241,0.5)]"
                    />
                </div>

            </div>
        </section>
    }
}
