use crate::pages::{Home, Not_Found};
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{A, Route, Router, Routes},
    path,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Router>
            <Stylesheet id="leptos" href="/pkg/portfolio.css"/>
            <Title text="Christian Nava" />
            <header>
                <A href="#" class:white-text class:text-lg>"Portfolio"</A>
                <nav id="nav">
                    <a href="#about">"About"</a>
                    <a href="#projects">"Projects"</a>
                    <a href="#skills">"Skills"</a>
                    <a href="#contact">"Contact"</a>
                </nav>
            </header>
            <main>
                <Routes transition=true fallback=Not_Found>
                    <Route path=path!("") view=Home />
                    <Route path=path!("/*any") view=Not_Found />
                </Routes>
            </main>
        </Router>
    }
}

#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::wasm_bindgen;
#[cfg(feature = "hydrate")]
#[wasm_bindgen(module = "/js/foo.js")]
extern "C" {
    pub fn message() -> String;

}

#[cfg(not(feature = "hydrate"))]
#[allow(dead_code)]
pub fn message() -> String {
    "Rust".to_string()
}
