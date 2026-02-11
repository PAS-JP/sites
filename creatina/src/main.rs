use leptos::prelude::*;
mod components;
use components::prelude::*;

fn main() {
    #[cfg(feature = "ssr")]
    {
        //let js = r#""#;
        let html = view! {
            <html lang="pt-br" class="scroll-smooth">
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                    <meta name="google-site-verification" content="WJ6YR2mJz0iy7Mo2iRK1R9iAtl27zNpswBpmK7a_HD8" />
                    // --- SEO BÁSICO ---
                    <title>"Creatina Monohidratada Growth | 250g Original e Nota Fiscal"</title>
                    <meta name="description" content="Melhore seu desempenho com a Creatina Monohidratada Growth Supplements. 250g de alta pureza, força e ganho muscular. Compre original com garantia e nota fiscal." />
                    <meta name="keywords" content="creatina growth, creatina monohidratada, suplemento musculação, força muscular, growth supplements" />
                    <meta name="robots" content="index, follow" />
                    <link rel="canonical" href="https://g-creatina.pasjp.com.br" />

                    // --- OPEN GRAPH (Facebook/WhatsApp/Instagram) ---
                    <meta property="og:type" content="website" />
                    <meta property="og:title" content="Creatina Monohidratada Growth | Performance Máxima" />
                    <meta property="og:description" content="Garanta sua Creatina Growth original. O suplemento mais estudado para ganho de força e massa muscular." />
                    <meta property="og:image" content="https://g-creatina.pasjp.com.br/public/assets/Creatina Monohidratada 250g Growth Supplements - Sem sabor em Pó.webp" />
                    <meta property="og:url" content="https://g-creatina.pasjp.com.br" />
                    <meta property="og:site_name" content="Creatina Growth" />

                    // --- TWITTER CARDS ---
                    <meta name="twitter:card" content="summary_large_image" />
                    <meta name="twitter:title" content="Creatina Monohidratada Growth | 250g" />
                    <meta name="twitter:description" content="Aumento de força e volume muscular com garantia de originalidade." />
                    <meta name="twitter:image" content="https://g-creatina.pasjp.com.br/public/assets/Creatina Monohidratada 250g Growth Supplements - Sem sabor em Pó.webp" />

                    <link rel="stylesheet" href="public/tailwind.css" />
                    <link href="public/favicon.ico" rel="icon" type="image/x-icon" />
                    <link data-trunk href="public/" rel="copy-dir" />
                    <link data-trunk href="public/robots.txt" rel="copy-file" data-target-path="." />
                </head>
                <body class="bg-radial from-emerald-950 to-slate-950  text-white">
                    <main id="root">
                        <AppSSR />
                    </main>
                    /*<script type="module">
                        {js}
                    </script>*/
                </body>
            </html>
        }
        .to_html();

        let html = format!("<!DOCTYPE html>{html}");
        std::fs::write("index.html", html).expect("fail write html");
        println!("✅ index.html");
    }

    #[cfg(feature = "csr")]
    mount_to_body(AppCSR);
}
