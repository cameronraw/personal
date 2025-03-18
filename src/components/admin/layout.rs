use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::{component, prelude::Children, view, IntoView};

use super::sidebar::AdminSidebar;

#[component]
pub fn AdminLayout(
    #[prop(default = None)] class: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="flex min-h-screen bg-gray-50">
            <AdminSidebar/>
            <div class="flex-1">
                <header class="bg-primary text-white shadow-md">
                    <div class="container mx-auto px-4 py-4">
                        <h1 class="text-2xl font-bold">Admin Dashboard</h1>
                    </div>
                </header>
                <main class=move || format!("container mx-auto px-4 py-6 {}", class.unwrap_or(""))>
                    {children()}
                </main>
            </div>
        </div>
    }
}
