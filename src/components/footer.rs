use chrono::Local;
use leptos::prelude::*;

/// Footer component
#[component]
pub fn Footer() -> impl IntoView {
    let current_year = Local::now().format("%Y").to_string();
    view! {
        <footer class="bg-neutralDark text-white py-8">
            <div class="container mx-auto px-4">
                <div class="flex flex-col md:flex-row justify-between">
                    <div class="mb-4 md:mb-0">
                        <h3 class="text-xl font-bold mb-2">Cameron Raw</h3>
                        <p class="text-gray-300">Software Engineer specializing in modern web technologies</p>
                    </div>
                    <div>
                        <h4 class="text-lg font-semibold mb-2">Connect</h4>
                        <div class="flex space-x-4">
                            <a href="#" class="text-gray-300 hover:text-white transition-colors">GitHub</a>
                            <a href="#" class="text-gray-300 hover:text-white transition-colors">LinkedIn</a>
                            <a href="#" class="text-gray-300 hover:text-white transition-colors">Twitter</a>
                        </div>
                    </div>
                </div>
                <div class="mt-8 pt-4 border-t border-gray-700 text-center text-gray-400">
                    <p>"© " {current_year} " Cameron Raw. All rights reserved."</p>
                </div>
            </div>
        </footer>
    }
}
