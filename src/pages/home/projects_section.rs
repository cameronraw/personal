use leptos::prelude::*;

#[component]
pub fn ProjectsSection() -> impl IntoView {
    view! {
        <section id="projects" class="py-16 bg-gray-50">
            <div class="container mx-auto px-4">
                <h2 class="text-3xl font-bold mb-2 text-primary text-center">"Featured Projects"</h2>
                <p class="text-lg text-neutralDark text-center mb-12 max-w-2xl mx-auto">"Here are some of the projects I've worked on that showcase my skills and expertise."</p>

                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                    <ProjectCard
                        bg_color="bg-primaryDark"
                        title="E-Commerce Platform"
                        description="A full-featured e-commerce solution built with Rust and React, featuring real-time inventory management and payment processing."
                        technologies={vec!["Rust", "React", "PostgreSQL"]}
                    />
                    <ProjectCard
                        bg_color="bg-secondary"
                        title="Content Management System"
                        description="A modern CMS built with TypeScript and Node.js, featuring a headless architecture and GraphQL API."
                        technologies={vec!["TypeScript", "Node.js", "GraphQL"]}
                    />
                    <ProjectCard
                        bg_color="bg-primary"
                        title="Analytics Dashboard"
                        description="A real-time analytics dashboard built with Leptos and WebSockets, featuring interactive data visualizations."
                        technologies={vec!["Rust", "Leptos", "WebSockets"]}
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn ProjectCard(
    bg_color: &'static str,
    title: &'static str,
    description: &'static str,
    technologies: Vec<&'static str>,
) -> impl IntoView {
    view! {
        <div class="bg-white rounded-lg overflow-hidden shadow-lg transition-transform hover:transform hover:scale-105">
            <div class={"h-48 flex items-center justify-center ".to_string() + bg_color}>
                <span class="text-white text-xl">"Project Screenshot"</span>
            </div>
            <div class="p-6">
                <h3 class="text-xl font-bold mb-2 text-primary">{title}</h3>
                <p class="text-neutralDark mb-4">{description}</p>
                <div class="flex flex-wrap gap-2 mb-4">
                    {technologies.into_iter().map(|tech| view! {
                        <span class="bg-gray-100 text-xs text-neutralDark px-2 py-1 rounded">{tech}</span>
                    }).collect::<Vec<_>>()}
                </div>
                <div class="flex space-x-4">
                    <a href="#" class="text-primary hover:text-primaryDark transition-colors">"View Project"</a>
                    <a href="#" class="text-primary hover:text-primaryDark transition-colors">"GitHub"</a>
                </div>
            </div>
        </div>
    }
}
