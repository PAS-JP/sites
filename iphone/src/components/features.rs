use leptos::prelude::*;

#[component]
pub fn FeaturesSection() -> impl IntoView {
    view! {
        <section class="w-full bg-gradient-to-b from-black to-[#050816] text-white py-24">
            <div class="max-w-7xl mx-auto px-6">

                // HEADER
                <div class="text-center max-w-3xl mx-auto mb-16 space-y-6">
                    <h2 class="text-4xl md:text-5xl font-extrabold tracking-tight">
                        "Por que escolher o "
                        <span class="bg-gradient-to-r from-blue-500 via-indigo-500 to-purple-500 bg-clip-text text-transparent">
                            "iPhone 16 Pro Max?"
                        </span>
                    </h2>

                    <p class="text-gray-400 text-lg leading-relaxed">
                        "Tecnologia de ponta em cada detalhe. Conheça os recursos
                        que fazem deste o iPhone mais avançado."
                    </p>
                </div>

                // GRID
                <div class="grid gap-8 sm:grid-cols-2 lg:grid-cols-3">

                    <FeatureCard
                        icon="📷"
                        title="Câmera 48MP"
                        description="Sistema de câmera profissional com zoom óptico 5x"
                    />

                    <FeatureCard
                        icon="🧠"
                        title="Chip A18 Pro"
                        description="O chip mais potente já criado para um smartphone"
                    />

                    <FeatureCard
                        icon="🔋"
                        title="Bateria o dia todo"
                        description="Até 33h de reprodução de vídeo"
                    />

                    <FeatureCard
                        icon="📱"
                        title="Tela Super Retina"
                        description="Display OLED de 6.9\" com ProMotion 120Hz"
                    />

                    <FeatureCard
                        icon="🛡"
                        title="Titânio"
                        description="Design premium com acabamento em titânio"
                    />

                    <FeatureCard
                        icon="⚡"
                        title="USB-C rápido"
                        description="Carregamento e transferência ultrarrápidos"
                    />

                </div>
            </div>
        </section>
    }
}

#[component]
fn FeatureCard(
    icon: &'static str,
    title: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="
            group relative p-8 rounded-2xl
            bg-white/5 border border-white/10
            backdrop-blur-md
            transition-all duration-300
            hover:-translate-y-1
            hover:border-indigo-500/40
            hover:shadow-[0_0_40px_rgba(99,102,241,0.25)]
        ">

            <div class="mb-6">
                <div class="
                    w-12 h-12 flex items-center justify-center
                    rounded-xl
                    bg-indigo-500/10
                    text-indigo-400 text-2xl
                ">
                    {icon}
                </div>
            </div>

            <h3 class="text-xl font-semibold mb-3">
                {title}
            </h3>

            <p class="text-gray-400 leading-relaxed">
                {description}
            </p>

        </div>
    }
}
