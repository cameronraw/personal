use leptos::prelude::*;

#[component]
pub fn HeroSection() -> impl IntoView {
    view! {
        <section class="bg-primary text-white py-20">
            <div class="container mx-auto px-4">
                <div class="max-w-3xl mx-auto text-center">
                    <h1 class="text-5xl font-bold mb-6">"Hello, I'm Cameron Raw"</h1>
                    <p class="text-xl mb-8">"A passionate Software Engineer specializing in building high-quality web applications with modern technologies."</p>
                    <div class="flex justify-center space-x-4">
                        <a href="#projects" class="bg-white text-primary px-6 py-3 rounded-lg font-semibold hover:bg-gray-100 transition-colors">"View My Work"</a>
                        <a href="#contact" class="bg-secondary text-white px-6 py-3 rounded-lg font-semibold hover:bg-opacity-90 transition-colors">"Get In Touch"</a>
                    </div>
                </div>
            </div>
        </section>
    }
}
