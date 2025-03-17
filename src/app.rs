use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/personal.css"/>

        // sets the document title
        <Title text="Cameron Raw | Software Engineer"/>

        // content for this welcome page
        <Router>
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
        </Router>
    }
}

/// Header component with navigation
#[component]
fn Header() -> impl IntoView {
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

/// Footer component
#[component]
fn Footer() -> impl IntoView {
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
                    <p>"© " {2024} " Cameron Raw. All rights reserved."</p>
                </div>
            </div>
        </footer>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        // Hero Section
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

        // About Section
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

        // Projects Section
        <section id="projects" class="py-16 bg-gray-50">
            <div class="container mx-auto px-4">
                <h2 class="text-3xl font-bold mb-2 text-primary text-center">"Featured Projects"</h2>
                <p class="text-lg text-neutralDark text-center mb-12 max-w-2xl mx-auto">"Here are some of the projects I've worked on that showcase my skills and expertise."</p>

                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
                    // Project 1
                    <div class="bg-white rounded-lg overflow-hidden shadow-lg transition-transform hover:transform hover:scale-105">
                        <div class="h-48 bg-primaryDark flex items-center justify-center">
                            <span class="text-white text-xl">"Project Screenshot"</span>
                        </div>
                        <div class="p-6">
                            <h3 class="text-xl font-bold mb-2 text-primary">"E-Commerce Platform"</h3>
                            <p class="text-neutralDark mb-4">"A full-featured e-commerce solution built with Rust and React, featuring real-time inventory management and payment processing."</p>
                            <div class="flex flex-wrap gap-2 mb-4">
                                <span class="bg-gray-100 text-xs text-neutralDark px-2 py-1 rounded">"Rust"</span>
                                <span class="bg-gray-100 text-xs text-neutralDark px-2 py-1 rounded">"React"</span>
                                <span class="bg-gray-100 text-xs text-neutralDark px-2 py-1 rounded">"PostgreSQL"</span>
                            </div>
                            <div class="flex space-x-4">
                                <a href="#" class="text-primary hover:text-primaryDark transition-colors">"View Project"</a>
                                <a href="#" class="text-primary hover:text-primaryDark transition-colors">"GitHub"</a>
                            </div>
                        </div>
                    </div>

                    // Project 2
                    <div class="bg-white rounded-lg overflow-hidden shadow-lg transition-transform hover:transform hover:scale-105">
                        <div class="h-48 bg-secondary flex items-center justify-center">
                            <span class="text-white text-xl">"Project Screenshot"</span>
                        </div>
                        <div class="p-6">
                            <h3 class="text-xl font-bold mb-2 text-primary">"Content Management System"</h3>
                            <p class="text-neutralDark mb-4">"A modern CMS built with TypeScript and Node.js, featuring a headless architecture and GraphQL API."</p>
                            <div class="flex flex-wrap gap-2 mb-4">
                                <span class="bg-gray-100 text-xs text-neutralDark px-2 py-1 rounded">"TypeScript"</span>
                                <span class="bg-gray-100 text-xs text-neutralDark px-2 py-1 rounded">"Node.js"</span>
                                <span class="bg-gray-100 text-xs text-neutralDark px-2 py-1 rounded">"GraphQL"</span>
                            </div>
                            <div class="flex space-x-4">
                                <a href="#" class="text-primary hover:text-primaryDark transition-colors">"View Project"</a>
                                <a href="#" class="text-primary hover:text-primaryDark transition-colors">"GitHub"</a>
                            </div>
                        </div>
                    </div>

                    // Project 3
                    <div class="bg-white rounded-lg overflow-hidden shadow-lg transition-transform hover:transform hover:scale-105">
                        <div class="h-48 bg-primary flex items-center justify-center">
                            <span class="text-white text-xl">"Project Screenshot"</span>
                        </div>
                        <div class="p-6">
                            <h3 class="text-xl font-bold mb-2 text-primary">"Analytics Dashboard"</h3>
                            <p class="text-neutralDark mb-4">"A real-time analytics dashboard built with Leptos and WebSockets, featuring interactive data visualizations."</p>
                            <div class="flex flex-wrap gap-2 mb-4">
                                <span class="bg-gray-100 text-xs text-neutralDark px-2 py-1 rounded">"Rust"</span>
                                <span class="bg-gray-100 text-xs text-neutralDark px-2 py-1 rounded">"Leptos"</span>
                                <span class="bg-gray-100 text-xs text-neutralDark px-2 py-1 rounded">"WebSockets"</span>
                            </div>
                            <div class="flex space-x-4">
                                <a href="#" class="text-primary hover:text-primaryDark transition-colors">"View Project"</a>
                                <a href="#" class="text-primary hover:text-primaryDark transition-colors">"GitHub"</a>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        // Articles Section
        <section id="articles" class="py-16 bg-white">
            <div class="container mx-auto px-4">
                <h2 class="text-3xl font-bold mb-2 text-primary text-center">"Recent Articles"</h2>
                <p class="text-lg text-neutralDark text-center mb-12 max-w-2xl mx-auto">"I write about software development, best practices, and emerging technologies."</p>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-8 max-w-4xl mx-auto">
                    // Article 1
                    <div class="bg-white rounded-lg overflow-hidden shadow border border-gray-100 transition-all hover:shadow-md">
                        <div class="p-6">
                            <span class="inline-block bg-primary text-white text-xs px-2 py-1 rounded mb-3">"Rust"</span>
                            <h3 class="text-xl font-bold mb-3 text-neutralDark">"Building Web Applications with Leptos"</h3>
                            <p class="text-gray-600 mb-4">"An in-depth guide to building reactive web applications using Leptos, a Rust framework for front-end development."</p>
                            <div class="flex items-center justify-between">
                                <span class="text-sm text-gray-500">"March 15, 2024"</span>
                                <a href="#" class="text-primary font-medium hover:text-primaryDark transition-colors">"Read More →"</a>
                            </div>
                        </div>
                    </div>

                    // Article 2
                    <div class="bg-white rounded-lg overflow-hidden shadow border border-gray-100 transition-all hover:shadow-md">
                        <div class="p-6">
                            <span class="inline-block bg-secondary text-white text-xs px-2 py-1 rounded mb-3">"JavaScript"</span>
                            <h3 class="text-xl font-bold mb-3 text-neutralDark">"Modern JavaScript Patterns for Clean Code"</h3>
                            <p class="text-gray-600 mb-4">"Exploring design patterns and best practices for writing maintainable and efficient JavaScript code."</p>
                            <div class="flex items-center justify-between">
                                <span class="text-sm text-gray-500">"February 28, 2024"</span>
                                <a href="#" class="text-primary font-medium hover:text-primaryDark transition-colors">"Read More →"</a>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        // Contact Section
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

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
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
