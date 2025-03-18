use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};

use crate::components::{Footer, Header};
use crate::pages::{Admin, HomePage, NotFound};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/personal.css"/>

        <Title text="Cameron Raw | Software Engineer"/>

        <Router>
            <Routes fallback=move || "Not found.">
                <Route path=StaticSegment("admin") view=Admin/>
            <Route path=StaticSegment("") view=|| view! {
                <div class="min-h-screen flex flex-col bg-gray-50">
                    <Header/>
                    <main class="flex-grow">
                        <Routes fallback=move || "Not found.">
                            <Route path=StaticSegment("") view=HomePage/>
                            <Route path=WildcardSegment("any") view=NotFound/>
                        </Routes>
                    </main>
                    <Footer/>
                </div>
            } />
            </Routes>

        </Router>
    }
}
