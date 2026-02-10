use leptos::prelude::*;

#[component]
pub fn BenefitsRow() -> impl IntoView {
    view! {
        <div class="
            flex flex-wrap items-center justify-center lg:justify-start gap-x-6 gap-y-3
            text-sm font-medium mb-7
            text-emerald-400
        ">
            <span class="inline-flex items-center gap-1.5">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="size-5 mb-1">
                    <path fill-rule="evenodd" d="M14.615 1.595a.75.75 0 0 1 .359.852L12.982 9.75h7.268a.75.75 0 0 1 .548 1.262l-10.5 11.25a.75.75 0 0 1-1.272-.71l1.992-7.302H3.75a.75.75 0 0 1-.548-1.262l10.5-11.25a.75.75 0 0 1 .913-.143Z" clip-rule="evenodd" />
                </svg>
                "Mais força"
            </span>

            <span class="inline-flex items-center gap-1.5">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="size-5 mb-1">
                    <path fill-rule="evenodd" d="M12.516 2.17a.75.75 0 0 0-1.032 0 11.209 11.209 0 0 1-7.877 3.08.75.75 0 0 0-.722.515A12.74 12.74 0 0 0 2.25 9.75c0 5.942 4.064 10.933 9.563 12.348a.749.749 0 0 0 .374 0c5.499-1.415 9.563-6.406 9.563-12.348 0-1.39-.223-2.73-.635-3.985a.75.75 0 0 0-.722-.516l-.143.001c-2.996 0-5.717-1.17-7.734-3.08Zm3.094 8.016a.75.75 0 1 0-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 0 0-1.06 1.06l2.25 2.25a.75.75 0 0 0 1.14-.094l3.75-5.25Z" clip-rule="evenodd" />
                </svg>
                "100% pura"
            </span>

            <span class="inline-flex items-center gap-1.5">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"  class="size-5 mb-1">
                    <path fill-rule="evenodd" d="M8.603 3.799A4.49 4.49 0 0 1 12 2.25c1.357 0 2.573.6 3.397 1.549a4.49 4.49 0 0 1 3.498 1.307 4.491 4.491 0 0 1 1.307 3.497A4.49 4.49 0 0 1 21.75 12a4.49 4.49 0 0 1-1.549 3.397 4.491 4.491 0 0 1-1.307 3.497 4.491 4.491 0 0 1-3.497 1.307A4.49 4.49 0 0 1 12 21.75a4.49 4.49 0 0 1-3.397-1.549 4.49 4.49 0 0 1-3.498-1.306 4.491 4.491 0 0 1-1.307-3.498A4.49 4.49 0 0 1 2.25 12c0-1.357.6-2.573 1.549-3.397a4.49 4.49 0 0 1 1.307-3.497 4.49 4.49 0 0 1 3.497-1.307Zm7.007 6.387a.75.75 0 1 0-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 0 0-1.06 1.06l2.25 2.25a.75.75 0 0 0 1.14-.094l3.75-5.25Z" clip-rule="evenodd" />
                </svg>
                "Original Growth"
            </span>
        </div>
    }
}

#[component]
pub fn OriginalBadge() -> impl IntoView {
    view! {
        <div
            class="
                inline-flex items-center gap-2
                px-4 py-2 mb-4
                rounded-full

                bg-emerald-500/10
                text-emerald-400
                text-sm font-semibold

                shadow-[0_0_20px_rgba(16,185,129,0.25)]
                border border-emerald-400/30

                backdrop-blur-sm
            "
        >
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"  class="size-5 shrink-0">
                <path fill-rule="evenodd" d="M8.603 3.799A4.49 4.49 0 0 1 12 2.25c1.357 0 2.573.6 3.397 1.549a4.49 4.49 0 0 1 3.498 1.307 4.491 4.491 0 0 1 1.307 3.497A4.49 4.49 0 0 1 21.75 12a4.49 4.49 0 0 1-1.549 3.397 4.491 4.491 0 0 1-1.307 3.497 4.491 4.491 0 0 1-3.497 1.307A4.49 4.49 0 0 1 12 21.75a4.49 4.49 0 0 1-3.397-1.549 4.49 4.49 0 0 1-3.498-1.306 4.491 4.491 0 0 1-1.307-3.498A4.49 4.49 0 0 1 2.25 12c0-1.357.6-2.573 1.549-3.397a4.49 4.49 0 0 1 1.307-3.497 4.49 4.49 0 0 1 3.497-1.307Zm7.007 6.387a.75.75 0 1 0-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 0 0-1.06 1.06l2.25 2.25a.75.75 0 0 0 1.14-.094l3.75-5.25Z" clip-rule="evenodd" />
            </svg>

            <span class="relative top-[0.5px] whitespace-nowrap">
                "100% Original Growth Supplements"
            </span>
        </div>
    }
}

#[component]
pub fn CardLink(children: Children) -> impl IntoView {
    view! {
        <a
            href="https://mercadolivre.com/sec/2z7u4or"
            target="_blank"
            class="
                inline-flex items-center justify-center gap-2
                text-base md:text-lg
                bg-linear-to-r from-emerald-500 to-teal-400
                px-6 md:px-10 py-4 md:py-5
                font-bold text-slate-950
                rounded-2xl
                w-full sm:w-auto

                shadow-[0_0_40px_8px_rgba(16,185,129,0.45)]
                hover:shadow-[0_0_70px_16px_rgba(16,185,129,0.6)]

                transition-all duration-300
                hover:opacity-80
                hover:scale-[1.02]

                overflow-hidden
                text-center
                animate-bounce
            "
        >
            <span class="relative whitespace-nowrap">
                {children()}
            </span>

            <svg
                xmlns="http://www.w3.org/2000/svg"
                viewBox="0 0 24 24"
                fill="currentColor"
                class="size-6 shrink-0 mb-1"
            >
                <path
                    fill-rule="evenodd"
                    d="M15.75 2.25H21a.75.75 0 0 1 .75.75v5.25a.75.75 0 0 1-1.5 0V4.81L8.03 17.03a.75.75 0 0 1-1.06-1.06L19.19 3.75h-3.44a.75.75 0 0 1 0-1.5Zm-10.5 4.5a1.5 1.5 0 0 0-1.5 1.5v10.5a1.5 1.5 0 0 0 1.5 1.5h10.5a1.5 1.5 0 0 0 1.5-1.5V10.5a.75.75 0 0 1 1.5 0v8.25a3 3 0 0 1-3 3H5.25a3 3 0 0 1-3-3V8.25a3 3 0 0 1 3-3h8.25a.75.75 0 0 1 0 1.5H5.25Z"
                    clip-rule="evenodd"
                />
            </svg>
        </a>
    }
}

#[component]
pub fn HeroText() -> impl IntoView {
    view! {
        <p class="max-w-xl text-lg md:text-xl mb-4 leading-relaxed text-slate-300">
            "Aumente sua "
            <span class="font-semibold text-slate-100">
                "força e potência muscular"
            </span>
            " com "
            <span class="font-medium text-emerald-400">
                "creatina 100% pura"
            </span>
            ", sem sabor e sem aditivos. "
            <span class="text-slate-400">
                "O suplemento mais estudado do mundo."
            </span>
        </p>
    }
}

#[component]
pub fn Img() -> impl IntoView {
    view! {
        <div class="w-full max-w-[300px] md:max-w-md">
            <div class="relative rounded-2xl overflow-hidden bg-black shadow-[0_0_40px_8px_rgba(16,185,129,0.45)]
            hover:shadow-[0_0_70px_16px_rgba(16,185,129,0.6)]
            transition-all duration-300
            hover:opacity-80
            hover:scale-[1.02]">
              <img src="public/assets/Creatina Monohidratada 250g Growth Supplements - Sem sabor em Pó.webp" 
                alt="Creatina Monohidratada 250g Growth Supplements - Sem sabor em Pó"
                title="Creatina Monohidratada 250g Growth Supplements - Sem sabor em Pó"
                class="w-full h-auto"
              />
            </div>
        </div>
    }
}

#[component]
pub fn Card() -> impl IntoView {
    view! {
        <div class="flex justify-center pt-40 md:pt-32 lg:pt-48 px-4 pb-20">
            <section class="flex flex-col lg:flex-row justify-between items-center w-full lg:w-9/12 gap-12">
                <div class="flex flex-col items-center lg:items-start text-center lg:text-left order-2 lg:order-1">
                    <OriginalBadge/>
                    <h1 class="text-4xl md:text-6xl lg:text-7xl mb-5 font-extrabold leading-tight">
                        <span class="block">"Creatina"</span>
                        <span class="block text-green-400">"Monohidratada"</span>
                        <span class="block">"Growth 250g"</span>
                    </h1>
                    <HeroText/>
                    <BenefitsRow/>
                    <CardLink>
                        "Comprar Agora no Mercado Livre"
                    </CardLink>
                    <div class="text-slate-400 mt-6 text-sm md:text-base">
                        "✓ Entrega rápida  •  ✓ Compra segura  •  ✓ Produto original"
                    </div>
                </div>
                <div class="order-1 lg:order-2">
                    <Img/>
                </div>
            </section>
        </div>
    }
}