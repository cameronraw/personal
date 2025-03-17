use leptos::prelude::*;

/// 404 - Not Found
#[component]
pub fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <div class="container mx-auto px-4 py-16 text-center">
            <h1 class="text-6xl font-bold text-primary mb-4">"404"</h1>
            <h2 class="text-2xl font-semibold text-neutralDark mb-6">"Page Not Found"</h2>
            <p class="text-lg text-gray-600 mb-8">"The page you're looking for doesn't exist or has been moved."</p>
            <a href="/" class="bg-primary text-white px-6 py-3 rounded-lg font-semibold hover:bg-primaryDark transition-colors">"Back to Home"</a>
        </div>
    }
}
