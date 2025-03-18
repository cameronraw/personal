use leptos::prelude::*;

use crate::components::admin::AdminLayout;

#[component]
pub fn Admin() -> impl IntoView {
    view! {
        <AdminLayout>
            <div class="bg-white p-6 rounded-lg shadow-md">
                <h2 class="text-2xl font-semibold mb-4">Welcome to the Admin Dashboard</h2>
                <p class="text-gray-600">
                    This is your central hub for managing your website content.
                </p>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 mt-6">
                    <div class="bg-primary bg-opacity-10 p-4 rounded-lg border border-primary border-opacity-20 hover:shadow-md transition-shadow">
                        <h3 class="font-medium text-primary">Content</h3>
                        <p class="text-sm text-gray-600 mt-1">Manage your articles and pages</p>
                    </div>
                    <div class="bg-secondary bg-opacity-10 p-4 rounded-lg border border-secondary border-opacity-20 hover:shadow-md transition-shadow">
                        <h3 class="font-medium text-secondary">Media</h3>
                        <p class="text-sm text-gray-600 mt-1">Upload and organize your media files</p>
                    </div>
                    <div class="bg-neutralDark bg-opacity-10 p-4 rounded-lg border border-neutralDark border-opacity-20 hover:shadow-md transition-shadow">
                        <h3 class="font-medium text-neutralDark">Settings</h3>
                        <p class="text-sm text-gray-600 mt-1">Configure your website settings</p>
                    </div>
                </div>
            </div>
        </AdminLayout>
    }
}
