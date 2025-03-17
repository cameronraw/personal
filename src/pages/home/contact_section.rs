use leptos::prelude::*;

#[component]
pub fn ContactSection() -> impl IntoView {
    view! {
        <section id="contact" class="py-16 bg-gray-50">
            <div class="container mx-auto px-4">
                <div class="max-w-2xl mx-auto text-center">
                    <h2 class="text-3xl font-bold mb-2 text-primary">"Get In Touch"</h2>
                    <p class="text-lg text-neutralDark mb-8">"Interested in working together? Feel free to reach out to me."</p>
                    <a href="mailto:contact@example.com" class="inline-block bg-primary text-white px-8 py-4 rounded-lg font-semibold hover:bg-primaryDark transition-colors">"Contact Me"</a>
                </div>
            </div>
        </section>
    }
}
