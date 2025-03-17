use leptos::prelude::*;

#[component]
pub fn AboutSection() -> impl IntoView {
    view! {
        <section class="py-16 bg-white">
            <div class="container mx-auto px-4">
                <div class="max-w-4xl mx-auto">
                    <h2 class="text-3xl font-bold mb-8 text-primary text-center">"About Me"</h2>
                    <div class="flex flex-col md:flex-row items-center gap-8">
                        <div class="w-full md:w-1/3 flex justify-center">
                            <div class="w-64 h-64 bg-gray-200 rounded-full overflow-hidden">
                                // Placeholder for profile image
                                <div class="w-full h-full bg-primaryDark flex items-center justify-center text-white text-4xl font-bold">"CR"</div>
                            </div>
                        </div>
                        <div class="w-full md:w-2/3">
                            <p class="text-lg text-neutralDark mb-4">
                                "I'm a software engineer with a passion for creating elegant solutions to complex problems.
                                With expertise in modern web technologies, I build applications that are not only functional 
                                but also provide exceptional user experiences."
                            </p>
                            <p class="text-lg text-neutralDark mb-6">
                                "My technical toolkit includes Rust, JavaScript, TypeScript, React, and various backend technologies.
                                I'm particularly interested in performance optimization, accessibility, and clean code practices."
                            </p>
                            <div class="flex flex-wrap gap-2">
                                <span class="bg-gray-100 text-neutralDark px-3 py-1 rounded-full">"Rust"</span>
                                <span class="bg-gray-100 text-neutralDark px-3 py-1 rounded-full">"JavaScript"</span>
                                <span class="bg-gray-100 text-neutralDark px-3 py-1 rounded-full">"TypeScript"</span>
                                <span class="bg-gray-100 text-neutralDark px-3 py-1 rounded-full">"React"</span>
                                <span class="bg-gray-100 text-neutralDark px-3 py-1 rounded-full">"Node.js"</span>
                                <span class="bg-gray-100 text-neutralDark px-3 py-1 rounded-full">"Leptos"</span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
