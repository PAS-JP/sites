use leptos::prelude::*;

#[component]
pub fn FAQSection() -> impl IntoView {
    view! {
        <section id="faq" class="pt-48 pb-32 flex flex-col items-center px-4">
            <div class="text-center mb-16">
                <h2 class="text-6xl font-black text-white tracking-tighter mb-4">
                    "Perguntas " <span class="text-emerald-400">"Frequentes"</span>
                </h2>
                <p class="text-slate-400 text-lg">"Tudo que você precisa saber sobre creatina monohidratada."</p>
            </div>

            <div class="w-full max-w-4xl flex flex-col gap-6">
                <FAQItem 
                    question="O que é creatina monohidratada?" 
                    answer="A creatina monohidratada é a forma mais estudada e eficaz de creatina. É um aminoácido naturalmente produzido pelo corpo e encontrado em alimentos como carne vermelha. Como suplemento, ajuda a aumentar os estoques de fosfocreatina nos músculos, melhorando a produção de energia durante exercícios de alta intensidade."
                />
                
                <FAQItem 
                    question="Quem pode tomar creatina?" 
                    answer="A creatina é indicada para praticantes de musculação, atletas de esportes de força e potência, e pessoas que realizam exercícios de alta intensidade. É segura para adultos saudáveis. Consulte um médico antes de usar se tiver condições renais ou outras condições de saúde."
                />

                <FAQItem 
                    question="Preciso fazer fase de carga?" 
                    answer="Não é necessário. A fase de carga (20g/dia por 5-7 dias) acelera a saturação, mas não é obrigatória. Tomando 5g diários, você atinge a saturação completa em 3-4 semanas com os mesmos resultados finais."
                />

                <FAQItem 
                    question="Qual o melhor horário para tomar creatina?" 
                    answer="A creatina funciona por saturação, então o horário específico não é crucial. No entanto, muitos preferem tomar após o treino junto com carboidratos, pois a captação muscular pode ser ligeiramente melhor. Em dias de descanso, tome com qualquer refeição."
                />

                <FAQItem 
                    question="A creatina Growth é original?" 
                    answer="Sim! Comercializamos apenas produtos 100% originais da Growth Supplements, com nota fiscal e garantia de procedência. A Growth é uma das marcas mais respeitadas do Brasil em suplementação esportiva."
                />

                <FAQItem 
                    question="Creatina causa retenção de líquido?" 
                    answer="A creatina pode causar uma leve retenção hídrica INTRAMUSCULAR (dentro do músculo), o que é diferente de inchaço. Essa água no músculo é benéfica para a síntese proteica e contribui para o aumento de volume muscular. Não causa inchaço subcutâneo."
                />

                <FAQItem 
                    question="Por quanto tempo posso usar creatina?" 
                    answer="A creatina pode ser usada continuamente. Não há evidências de que o uso prolongado seja prejudicial para pessoas saudáveis. Não é necessário ciclar ou fazer pausas no uso do suplemento."
                />
            </div>
        </section>
    }
}

#[component]
fn FAQItem(question: &'static str, answer: &'static str) -> impl IntoView {
    view! {
        <div class="group p-8 rounded-2xl bg-slate-900/40 border border-slate-800 transition-all duration-300 hover:border-emerald-500/40 hover:bg-slate-900/60">
            <h3 class="text-xl font-bold text-white mb-4 flex items-center gap-3">
                <span class="w-1.5 h-6 bg-emerald-500 rounded-full"></span>
                {question}
            </h3>
            <p class="text-slate-400 leading-relaxed pl-4 border-l border-slate-800 group-hover:border-emerald-500/20 transition-colors">
                {answer}
            </p>
        </div>
    }
}