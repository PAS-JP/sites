use leptos::prelude::*;
mod components;
use components::prelude::*;

fn main() {
    #[cfg(feature = "ssr")]
    {
        let html = view! {
        <html lang="pt-br" class="scroll-smooth">
            <head>
                <meta charset="UTF-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />

                <title>"iPhone 16 Pro Max | Melhor Preço no Brasil com Garantia"</title>
                <meta
                    name="description"
                    content="Compre o iPhone 16 Pro Max original com nota fiscal e garantia. Melhor preço do Brasil, entrega rápida e pagamento seguro."
                />
                <meta name="robots" content="index, follow, max-image-preview:large" />
                <link rel="canonical" href="https://i.pasjp.com.br" />

                <meta property="og:type" content="website" />
                <meta property="og:title" content="iPhone 16 Pro Max | Melhor Preço no Brasil" />
                <meta property="og:image" content="https://i.pasjp.com.br/public/iphone.webp" />

                <script type="application/ld+json">
                    {r#"
                    {
                        "@context": "https://schema.org",
                        "@type": "Product",
                        "name": "iPhone 16 Pro Max",
                        "brand": { "@type": "Brand", "name": "Apple" },
                        "offers": {
                            "@type": "Offer",
                            "priceCurrency": "BRL",
                            "availability": "https://schema.org/InStock"
                        }
                    }
                    "#}
                </script>
                    <link rel="stylesheet" href="public/tailwind.css" />
                    <link href="public/favicon.ico" rel="icon" type="image/x-icon" />
                    <link data-trunk href="public/" rel="copy-dir" />
                    <link data-trunk href="public/robots.txt" rel="copy-file" data-target-path="." />
            </head>

            <body class="bg-black text-white">
                <main id="root">
                    <AppSSR />
                </main>
            </body>
        </html>
    }
    .to_html();

        let html = format!("<!DOCTYPE html>{html}");
        std::fs::write("index.html", html).expect("fail write html");
    }

    #[cfg(feature = "csr")]
    mount_to_body(AppCSR);
}
