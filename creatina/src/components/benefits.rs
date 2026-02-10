use leptos::prelude::*;
use super::title::*;

use leptos::prelude::*;

#[component]
pub fn BenefitsCard(title: &'static str, text: &'static str, children: Children) -> impl IntoView {
    view! {
        <div class="group transition-all duration-300 hover:border-emerald-400/50 bg-linear-to-tl from-slate-950 to-slate-900 w-full md:w-1/3 shadow-black shadow-xl p-6 rounded-xl border border-slate-800 m-4 flex flex-col">
            <div class="bg-emerald-500/20 rounded-lg p-4 w-14 h-14 text-emerald-500 mb-2 flex items-center justify-center transition-transform duration-300 group-hover:scale-110">
                {children()}
            </div>
            <div class="mt-4">
                <h2 class="text-2xl font-bold text-emerald-500">
                    {title}
                </h2>
                <p class="mt-4 text-slate-300">
                    {text}
                </p>
            </div>
        </div>
    }
}

#[component]
pub fn Benefits() -> impl IntoView {
    view! {
        <section class="text-lg flex justify-center items-center flex-col pt-48" id="benefits">
            <Title
                headline="Por que usar"
                green_text="Creatina"
                first_describe="O suplemento mais estudado e comprovado cientificamente para"
                second_describe="ganho de força e massa muscular."
            />
            <div class="flex flex-wrap gap-5 justify-center items-center mt-20">
                <BenefitsCard
                    title="Mais Força Muscular"
                    text="Aumento comprovado de força e potência para treinos mais intensos e eficazes."
                >
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="size-6"
                        viewBox="0 0 24 24"
                        fill="currentColor"
                    >
                        <path d="M4 7a1 1 0 0 1 1 1v8a1 1 0 0 1 -2 0v-3h-1a1 1 0 0 1 0 -2h1v-3a1 1 0 0 1 1 -1"/>
                        <path d="M20 7a1 1 0 0 1 1 1v3h1a1 1 0 0 1 0 2h-1v3a1 1 0 0 1 -2 0v-8a1 1 0 0 1 1 -1"/>
                        <path d="M16 5a2 2 0 0 1 2 2v10a2 2 0 1 1 -4 0v-4h-4v4a2 2 0 1 1 -4 0v-10a2 2 0 1 1 4 0v4h4v-4a2 2 0 0 1 2 -2"/>
                    </svg>
                </BenefitsCard>
                <BenefitsCard
                    title="Alto Desempenho"
                    text="Melhora significativa em exercícios de alta intensidade e curta duração."
                >
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="size-6">
                        <path fill-rule="evenodd" d="M12.963 2.286a.75.75 0 0 0-1.071-.136 9.742 9.742 0 0 0-3.539 6.176 7.547 7.547 0 0 1-1.705-1.715.75.75 0 0 0-1.152-.082A9 9 0 1 0 15.68 4.534a7.46 7.46 0 0 1-2.717-2.248ZM15.75 14.25a3.75 3.75 0 1 1-7.313-1.172c.628.465 1.35.81 2.133 1a5.99 5.99 0 0 1 1.925-3.546 3.75 3.75 0 0 1 3.255 3.718Z" clip-rule="evenodd" />
                    </svg>
                </BenefitsCard>
                <BenefitsCard
                    title="Mais Energia ATP"
                    text="Aumenta os estoques de fosfocreatina, fornecendo energia rápida para os músculos."
                >
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="size-6">
                        <path fill-rule="evenodd" d="M14.615 1.595a.75.75 0 0 1 .359.852L12.982 9.75h7.268a.75.75 0 0 1 .548 1.262l-10.5 11.25a.75.75 0 0 1-1.272-.71l1.992-7.302H3.75a.75.75 0 0 1-.548-1.262l10.5-11.25a.75.75 0 0 1 .913-.143Z" clip-rule="evenodd" />
                    </svg>
                </BenefitsCard>
                <BenefitsCard
                    title="Função Cognitiva"
                    text="Estudos mostram benefícios para memória e processamento mental."
                >
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" 
                    stroke-linejoin="round" class="size-6"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M15.5 13a3.5 3.5 0 0 0 -3.5 3.5v1a3.5 3.5 0 0 0 7 0v-1.8" /><path d="M8.5 13a3.5 3.5 0 0 1 3.5 3.5v1a3.5 3.5 0 0 1 -7 0v-1.8" /><path d="M17.5 16a3.5 3.5 0 0 0 0 -7h-.5" /><path d="M19 9.3v-2.8a3.5 3.5 0 0 0 -7 0" /><path d="M6.5 16a3.5 3.5 0 0 1 0 -7h.5" /><path d="M5 9.3v-2.8a3.5 3.5 0 0 1 7 0v10" /></svg>
                </BenefitsCard>
                <BenefitsCard
                    title="Recuperação Rápida"
                    text="Acelera a recuperação muscular entre séries e treinos."
                >
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="size-6">
                        <path fill-rule="evenodd" d="M12 2.25c-5.385 0-9.75 4.365-9.75 9.75s4.365 9.75 9.75 9.75 9.75-4.365 9.75-9.75S17.385 2.25 12 2.25ZM12.75 6a.75.75 0 0 0-1.5 0v6c0 .414.336.75.75.75h4.5a.75.75 0 0 0 0-1.5h-3.75V6Z" clip-rule="evenodd" />
                    </svg>
                </BenefitsCard>
                <BenefitsCard
                    title="100% Pura"
                    text="Sem sabor, sem aditivos, sem ingredientes desnecessários. Só creatina."
                >
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="size-6">
                        <path fill-rule="evenodd" d="M12.516 2.17a.75.75 0 0 0-1.032 0 11.209 11.209 0 0 1-7.877 3.08.75.75 0 0 0-.722.515A12.74 12.74 0 0 0 2.25 9.75c0 5.942 4.064 10.933 9.563 12.348a.749.749 0 0 0 .374 0c5.499-1.415 9.563-6.406 9.563-12.348 0-1.39-.223-2.73-.635-3.985a.75.75 0 0 0-.722-.516l-.143.001c-2.996 0-5.717-1.17-7.734-3.08Zm3.094 8.016a.75.75 0 1 0-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 0 0-1.06 1.06l2.25 2.25a.75.75 0 0 0 1.14-.094l3.75-5.25Z" clip-rule="evenodd" />
                    </svg>
                </BenefitsCard>
            </div>
        </section>
    }
}
