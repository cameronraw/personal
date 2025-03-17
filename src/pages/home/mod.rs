mod about_section;
mod articles_section;
mod contact_section;
mod hero_section;
mod projects_section;

use about_section::AboutSection;
use articles_section::ArticlesSection;
use contact_section::ContactSection;
use hero_section::HeroSection;
use leptos::prelude::*;
use projects_section::ProjectsSection;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <HeroSection />
        <AboutSection />
        <ProjectsSection />
        <ArticlesSection />
        <ContactSection />
    }
}
