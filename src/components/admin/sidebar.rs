use leptos::prelude::*;

#[component]
pub fn AdminSidebar() -> impl IntoView {
    view! {
        <div class="bg-neutralDark text-white w-64 min-h-screen p-4">
            <div class="text-xl font-bold mb-6">Admin Dashboard</div>
            <nav>
                <ul class="space-y-2">
                    <li>
                        <a href="/admin" class="block py-2 px-4 rounded hover:bg-primaryDark transition-colors">
                            Dashboard
                        </a>
                    </li>
                    <li>
                        <a href="/admin/articles" class="block py-2 px-4 rounded hover:bg-primaryDark transition-colors">
                            Articles
                        </a>
                    </li>
                    <li>
                        <a href="/admin/settings" class="block py-2 px-4 rounded hover:bg-primaryDark transition-colors">
                            Settings
                        </a>
                    </li>
                </ul>
            </nav>
            <div class="mt-8 pt-4 border-t border-gray-700">
                <a href="/" class="text-gray-300 hover:text-white transition-colors flex items-center">
                     Back to Site
                </a>
            </div>
        </div>
    }
}
