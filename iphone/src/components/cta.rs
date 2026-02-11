use leptos::prelude::*;

#[component]
pub fn CTA(children: Children) -> impl IntoView {
    view! {
        <a
            href="https://mercadolivre.com/sec/2WDgP9N"
            target="_blank"
               class="
                inline-flex items-center justify-center gap-2
                text-base md:text-lg
                bg-linear-to-r from-blue-500 via-indigo-500 to-purple-600
                px-6 md:px-10 py-4 md:py-5
                font-bold text-white
                rounded-2xl
                w-full sm:w-auto

                shadow-[0_0_40px_8px_rgba(99,102,241,0.45)]
                hover:shadow-[0_0_70px_16px_rgba(124,58,237,0.6)]

                transition-all duration-300 ease-out
                hover:opacity-95
                hover:scale-[1.03]

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
