use leptos::prelude::*;

#[component]
pub fn ArticlesSection() -> impl IntoView {
    view! {
        <section id="articles" class="py-16 bg-white">
            <div class="container mx-auto px-4">
                <h2 class="text-3xl font-bold mb-2 text-primary text-center">"Recent Articles"</h2>
                <p class="text-lg text-neutralDark text-center mb-12 max-w-2xl mx-auto">"I write about software development, best practices, and emerging technologies."</p>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-8 max-w-4xl mx-auto">
                    <ArticleCard
                        category="Rust"
                        category_bg="bg-primary"
                        title="Building Web Applications with Leptos"
                        description="An in-depth guide to building reactive web applications using Leptos, a Rust framework for front-end development."
                        date="March 15, 2024"
                    />
                    <ArticleCard
                        category="JavaScript"
                        category_bg="bg-secondary"
                        title="Modern JavaScript Patterns for Clean Code"
                        description="Exploring design patterns and best practices for writing maintainable and efficient JavaScript code."
                        date="February 28, 2024"
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn ArticleCard(
    category: &'static str,
    category_bg: &'static str,
    title: &'static str,
    description: &'static str,
    date: &'static str,
) -> impl IntoView {
    view! {
        <div class="bg-white rounded-lg overflow-hidden shadow border border-gray-100 transition-all hover:shadow-md">
            <div class="p-6">
                <span class={"inline-block text-white text-xs px-2 py-1 rounded mb-3 ".to_string() + category_bg}>{category}</span>
                <h3 class="text-xl font-bold mb-3 text-neutralDark">{title}</h3>
                <p class="text-gray-600 mb-4">{description}</p>
                <div class="flex items-center justify-between">
                    <span class="text-sm text-gray-500">{date}</span>
                    <a href="#" class="text-primary font-medium hover:text-primaryDark transition-colors">"Read More →"</a>
                </div>
            </div>
        </div>
    }
}
