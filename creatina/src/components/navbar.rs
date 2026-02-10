use super::prelude::*;
use leptos::prelude::*;

#[component]
pub fn NavbarItem(children: Children, id: &'static str) -> impl IntoView {
    view! {
        <div id=id class="cursor-pointer text-white px-4 py-2.5 hover:opacity-70 hover:underline">
            {children()}
        </div>
    }
}

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="fixed top-0 left-0 w-full z-50 bg-slate-950/90 backdrop-blur-md border-b border-white/5">
            <div class="max-w-7xl mx-auto flex flex-col md:flex-row items-center justify-between px-4">
                
                <div class="py-3 shrink-0">
                    <span class="text-white font-bold text-lg bg-slate-900 px-4 py-1.5 rounded-lg border border-white/5 whitespace-nowrap">
                        "Creatina " <span class="text-green-500">"Growth"</span>
                    </span>
                </div>

                <div class="w-full md:w-auto overflow-hidden">
                    <div class="
                        flex items-center gap-2 md:gap-6 
                        py-2 md:py-4 
                        overflow-x-auto flex-nowrap 
                        [scrollbar-width:none] [-ms-overflow-style:none] [&::-webkit-scrollbar]:hidden
                    ">
                        <NavbarItem id="to-benefits">"Benefícios"</NavbarItem>
                        <NavbarItem id="to-how-use">"Uso"</NavbarItem>
                        <NavbarItem id="to-product">"Produto"</NavbarItem>
                        <NavbarItem id="to-FAQ">"FAQ"</NavbarItem>

                        <div class="shrink-0 ml-2">
                            <Link>
                                "Comprar"
                            </Link>
                        </div>
                    </div>
                </div>

            </div>
        </nav>
    }
}