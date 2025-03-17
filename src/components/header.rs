use leptos::prelude::*;

/// Header component with navigation
#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="bg-primary text-white shadow-md">
            <div class="container mx-auto px-4 py-4 flex justify-between items-center">
                <div class="text-xl font-bold">Cameron Raw</div>
                <nav>
                    <ul class="flex space-x-6">
                        <li><a href="#" class="hover:text-gray-200 transition-colors">Home</a></li>
                        <li><a href="#projects" class="hover:text-gray-200 transition-colors">Projects</a></li>
                        <li><a href="#articles" class="hover:text-gray-200 transition-colors">Articles</a></li>
                        <li><a href="#contact" class="hover:text-gray-200 transition-colors">Contact</a></li>
                    </ul>
                </nav>
            </div>
        </header>
    }
}
