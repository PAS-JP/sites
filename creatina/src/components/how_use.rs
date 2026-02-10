use leptos::prelude::*;
use super::title::*;

#[component]
pub fn HowUse() -> impl IntoView {
    view! {
        <section id="how-use" class="pt-48 flex flex-col items-center">
            <Title
                headline="Como"
                green_text="Tomar"
                first_describe="A suplementação com creatina é simples, prática e eficiente."
                second_describe="Siga o protocolo de 5g diários para atingir a saturação muscular máxima."
            />

            <div class="flex flex-wrap gap-8 justify-center items-start mt-20 w-11/12 max-w-6xl">
                <div class="flex flex-col items-center text-center w-full md:w-72 group">
                    <div class="relative mb-6">
                        <div class="bg-emerald-500/10 border border-emerald-400/30 p-6 rounded-2xl backdrop-blur-sm group-hover:border-emerald-400 transition-colors">
                            <span class="absolute -top-3 -right-3 bg-emerald-500 text-slate-950 font-bold text-xs px-2.5 py-1 rounded-lg">"01"</span>
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" 
                            class="w-10 h-10 text-emerald-400"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M10.708 2.372a2.382 2.382 0 0 0 -.71 .686l-4.892 7.26c-1.981 3.314 -1.22 7.466 1.767 9.882c2.969 2.402 7.286 2.402 10.254 0c2.987 -2.416 3.748 -6.569 1.795 -9.836l-4.919 -7.306c-.722 -1.075 -2.192 -1.376 -3.295 -.686z" /></svg>
                        </div>
                    </div>
                    <h3 class="text-xl font-bold text-white mb-3">"Misture com água"</h3>
                    <p class="text-slate-400 leading-relaxed">
                        "Adicione " <span class="text-emerald-400 font-medium">"5g (1 colher medidora)"</span> " em 200ml de água ou na bebida de sua preferência para facilitar a ingestão."
                    </p>
                </div>
                <div class="hidden lg:block h-px w-12 bg-emerald-500/30 mt-12"></div>
                <div class="flex flex-col items-center text-center w-full md:w-72 group">
                    <div class="relative mb-6">
                        <div class="bg-emerald-500/10 border border-emerald-400/30 p-6 rounded-2xl backdrop-blur-sm group-hover:border-emerald-400 transition-colors">
                            <span class="absolute -top-3 -right-3 bg-emerald-500 text-slate-950 font-bold text-xs px-2.5 py-1 rounded-lg">"02"</span>
                            <svg class="w-10 h-10 text-emerald-400" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M12 6v6h4.5m4.5 0a9 9 0 11-18 0 9 9 0 0118 0z" />
                            </svg>
                        </div>
                    </div>
                    <h3 class="text-xl font-bold text-white mb-3">"Tome diariamente"</h3>
                    <p class="text-slate-400 leading-relaxed">
                        "Consuma todos os dias, preferencialmente " <span class="text-emerald-400 font-medium">"após o treino"</span> " ou com uma refeição rica em carboidratos."
                    </p>
                </div>
                <div class="hidden lg:block h-px w-12 bg-emerald-500/30 mt-12"></div>
                <div class="flex flex-col items-center text-center w-full md:w-72 group">
                    <div class="relative mb-6">
                        <div class="bg-emerald-500/10 border border-emerald-400/30 p-6 rounded-2xl backdrop-blur-sm group-hover:border-emerald-400 transition-colors">
                            <span class="absolute -top-3 -right-3 bg-emerald-500 text-slate-950 font-bold text-xs px-2.5 py-1 rounded-lg">"03"</span>
                            <svg class="w-10 h-10 text-emerald-400" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99" />
                            </svg>
                        </div>
                    </div>
                    <h3 class="text-xl font-bold text-white mb-3">"Seja consistente"</h3>
                    <p class="text-slate-400 leading-relaxed">
                        "A saturação completa ocorre em " <span class="text-emerald-400 font-medium">"3 a 4 semanas"</span> ". O uso contínuo é a chave para o ganho de força."
                    </p>
                </div>


                <div class="flex justify-center mt-12 w-full px-4">
                    <div class="group transition-all duration-300 hover:border-emerald-400/50 hover:shadow-[0_0_20px_-10px_rgba(16,185,129,0.3)] bg-emerald-500/5 max-w-3xl w-full p-6 rounded-2xl border border-emerald-500/20 backdrop-blur-sm">
                        <p class="text-center text-lg leading-relaxed text-slate-300">
                            <span class="text-emerald-400 font-bold mr-2 transition-colors group-hover:text-emerald-300">
                                "Dica:"
                            </span>
                            "Não é necessário fazer \"fase de carga\". "
                            <span class="font-semibold text-white">"5g diários"</span>
                            " são suficientes para saturação completa."
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}