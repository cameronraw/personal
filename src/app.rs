use crate::components::{Footer, Header};
use crate::pages::{ArticleDetailPage, ArticlesPage, HomePage, NotFound};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::components::{Outlet, ParentRoute};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/personal.css"/>

        <Title text="Cameron Raw | Software Engineer"/>

        <Router>
            <Routes fallback=move || "Not found.">
                <ParentRoute path=path!("") view=MainLayout>
                            <Route path=path!("") view=HomePage/>
                            <Route path=path!("articles") view=ArticlesPage/>
                            <Route path=path!("articles/:documentId") view=ArticleDetailPage/>
                            <Route path=path!("*any") view=NotFound/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}

#[component]
pub fn MainLayout() -> impl IntoView {
    view! {
        <div>
        <div class="min-h-screen flex flex-col bg-gray-50">
            <Header/>
            <main class="flex-grow">
                <Outlet />
            </main>
            <Footer />
            </div>
        </div>
    }
}
