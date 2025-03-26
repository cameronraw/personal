use leptos::html::InnerHtmlAttribute;
use leptos::prelude::CollectView;
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::prelude::{IntoAny, Read, Suspend, Suspense};
use leptos::server::OnceResource;
use leptos::view;
use leptos::{component, IntoView};
use leptos_meta::Title;
use leptos_router::hooks::use_params_map;
use markdown::to_html;
use reqwest::get;

use crate::error::FetchError;
use crate::models::article::ArticleResponse;

async fn fetch_article(document_id: String) -> String {
    let post_url = format!("http://localhost:1337/api/articles/{}", document_id);
    let response = get(post_url).await;
    match response {
        Ok(response) => {
            leptos::logging::log!("response: {:?}", response);
            let text = response.text().await;
            match text {
                Ok(text) => text,
                Err(err) => {
                    leptos::logging::log!("err at parse: {:?}", err);
                    FetchError {
                        message: "Failed to parse article".to_string(),
                    }
                    .to_string()
                }
            }
        }
        Err(err) => {
            leptos::logging::log!("err at fetch: {:?}", err);
            FetchError {
                message: "Failed to fetch article".to_string(),
            }
            .to_string()
        }
    }
}

#[component]
pub fn ArticleDetailPage() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50 py-16">
            <ArticleLoader/>
        </div>
    }
}

#[component]
fn ArticleLoader() -> impl IntoView {
    let params = use_params_map();
    let document_id = params.read().get("documentId").unwrap_or_default();

    let resource = OnceResource::new_blocking(fetch_article(document_id));

    view! {
        <Suspense fallback=move || view! {
            <div class="container mx-auto px-4">
                <div class="text-center py-12">
                    <div class="inline-block animate-spin rounded-full h-8 w-8 border-t-2 border-b-2 border-primary"></div>
                    <p class="mt-4 text-gray-600">"Loading article..."</p>
                </div>
            </div>
        }>
        {
            move || Suspend::new(async move {
                let data = resource.await;
                let article = serde_json::from_str::<ArticleResponse>(&data).map_err(|e| FetchError {
                    message: format!("Failed to parse article: {}", e),
                });
                view! {
                    {
                        match article {
                            Ok(article_response) => {
                                let article = &article_response.data;

                                view! {
                                    <Title text={format!("{} | Cameron Raw", article.title)}/>
                                    <div class="container mx-auto px-4">
                                        <div class="max-w-4xl mx-auto bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden p-4">
                                            <div class="p-8 border-b border-gray-100">
                                                <div class="mb-4">
                                                    <a href="/articles" class="text-primary hover:text-primary/80 flex items-center mb-6 font-medium">
                                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 mr-1" viewBox="0 0 20 20" fill="currentColor">
                                                            <path fill-rule="evenodd" d="M9.707 14.707a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 1.414L7.414 9H15a1 1 0 110 2H7.414l2.293 2.293a1 1 0 010 1.414z" clip-rule="evenodd" />
                                                        </svg>
                                                        "Back to Articles"
                                                    </a>
                                                    <h1 class="text-3xl md:text-4xl font-bold text-neutralDark mb-4">{article.title.clone()}</h1>
                                                    {
                                                        if let Some(ref subtitle) = article.subtitle {
                                                            view! {<p class="text-xl text-gray-600 mb-4">{subtitle.clone()}</p>}.into_any()
                                                        } else {
                                                            view! {<span class="hidden"></span>}.into_any()
                                                        }
                                                    }
                                                </div>

                                                <div class="flex flex-wrap items-center justify-between">
                                                    <div class="flex flex-wrap gap-2 mb-4">
                                                        {
                                                            if let Some(tags) = article.get_tags() {
                                                                tags.into_iter().map(|tag| {
                                                                    view! {
                                                                        <span class="text-xs font-medium text-primary bg-primary/10 px-3 py-1 rounded-full">{tag.clone()}</span>
                                                                    }
                                                                }).collect_view().into_any()
                                                            } else {
                                                                view! {<span class="hidden"></span>}.into_any()
                                                            }
                                                        }
                                                    </div>
                                                    <div class="text-sm text-gray-500">
                                                        {
                                                            if let Some(date) = article.get_published_date() {
                                                                view! {
                                                                    <span>"Published: " {date}</span>
                                                                }.into_any()
                                                            } else {
                                                                view! {<span class="hidden"></span>}.into_any()
                                                            }
                                                        }
                                                    </div>
                                                </div>
                                            </div>

                                            <div class="p-8">
                                                <div class="prose prose-lg max-w-none">
                                                    <div class="whitespace-pre-wrap" inner_html={to_html(&article.content)}></div>
                                                </div>
                                            </div>

                                            <div class="p-8 border-t border-gray-100">
                                                <div class="flex justify-between items-center">
                                                    <div class="text-sm text-gray-500">
                                                        {
                                                            if let Some(date) = article.get_updated_date() {
                                                                view! {
                                                                    <span>"Last updated: " {date}</span>
                                                                }.into_any()
                                                            } else {
                                                                view! {<span class="hidden"></span>}.into_any()
                                                            }
                                                        }
                                                    </div>
                                                    <a href="/articles" class="text-primary hover:text-primary/80 flex items-center font-medium">
                                                        "Back to Articles"
                                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 ml-1" viewBox="0 0 20 20" fill="currentColor">
                                                            <path fill-rule="evenodd" d="M10.293 5.293a1 1 0 011.414 0l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414-1.414L12.586 11H5a1 1 0 110-2h7.586l-2.293-2.293a1 1 0 010-1.414z" clip-rule="evenodd" />
                                                        </svg>
                                                    </a>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                }.into_any()
                            },
                            Err(_) => {
                                view! {
                                    <Title text="Article Not Found | Cameron Raw"/>
                                    <div class="container mx-auto px-4">
                                        <div class="max-w-4xl mx-auto bg-white rounded-xl shadow-sm border border-gray-100 overflow-hidden p-8 text-center">
                                            <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 text-red-500 mx-auto mb-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
                                            </svg>
                                            <h2 class="text-2xl font-bold text-neutralDark mb-2">"Article Not Found"</h2>
                                            <p class="text-gray-600 mb-6">"We couldn't find the article you're looking for. It may have been removed or the URL might be incorrect."</p>
                                            <a href="/articles" class="inline-flex items-center text-white bg-primary hover:bg-primary/90 px-6 py-3 rounded-lg font-semibold transition-colors">
                                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
                                                    <path fill-rule="evenodd" d="M9.707 14.707a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 1.414L7.414 9H15a1 1 0 110 2H7.414l2.293 2.293a1 1 0 010 1.414z" clip-rule="evenodd" />
                                                </svg>
                                                "Back to Articles"
                                            </a>
                                        </div>
                                    </div>
                                }.into_any()
                            },
                        }
                    }
                }
            })
        }
        </Suspense>
    }
}

