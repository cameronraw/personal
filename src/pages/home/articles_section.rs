use leptos::prelude::*;
use serde_json::Value;
use crate::models::article::{Article, ArticlesResponse};

#[server]
pub async fn get_articles() -> Result<Value, ServerFnError> {
    let response = reqwest::get("http://localhost:1337/api/articles")
        .await?
        .json()
        .await?;

    Ok(response)
}

fn handle_article_resource(
    article_resource: Resource<Result<Value, ServerFnError>>,
) -> impl IntoView {
    view! {
            <Suspense fallback=move || view! {<div class="text-center">{"Loading...".to_string()}</div>}.into_any()>
                {move || match article_resource.get() {
                    Some(Ok(article_json)) => {
                        leptos::logging::log!("Articles: {}", article_json);
                        match serde_json::from_value::<ArticlesResponse>(article_json) {
                            Ok(articles) => articles.data.into_iter().map(|article| {
                                view! {
                                    <ArticleCard article />
                                }
                            }).collect_view().into_any(),
                            Err(parse_error) => {
                                leptos::logging::log!("Error parsing articles: {:?}", parse_error);
                                view! {<div class="text-center">{"Error parsing articles".to_string()}</div>}.into_any()} 
                            }
                    },
                    Some(Err(_err)) => view! {<div class="text-center">{"Error loading articles".to_string()}</div>}.into_any(),
                    None => view! {<div class="text-center">{"Loading...".to_string()}</div>}.into_any()

                }}
            </Suspense>
        }.into_any()
}

#[component]
pub fn ArticlesSection() -> impl IntoView {
    let article_resource = Resource::new(|| (), |_| get_articles());

    let async_result = move || {
        let articles_view = handle_article_resource(article_resource);
        view! {
            <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                {articles_view}
            </div>
        }
    };

    view! {
            <section id="articles" class="py-16 bg-white">
                <div class="container mx-auto px-4">
                    <div class="flex justify-between items-center mb-12">
                        <div>
                            <h2 class="text-3xl font-bold mb-2 text-primary">"Recent Articles"</h2>
                            <p class="text-lg text-neutralDark max-w-2xl">"I write about software development, best practices, and emerging technologies."</p>
                        </div>
                        <a href="/articles" class="inline-flex items-center text-primary font-medium hover:underline">
                            "View All Articles"
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 ml-1" viewBox="0 0 20 20" fill="currentColor">
                                <path fill-rule="evenodd" d="M10.293 5.293a1 1 0 011.414 0l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414-1.414L12.586 11H5a1 1 0 110-2h7.586l-2.293-2.293a1 1 0 010-1.414z" clip-rule="evenodd" />
                            </svg>
                        </a>
                    </div>
                    {async_result}
                </div>
            </section>
    }.into_any()
}

#[component]
fn ArticleCard(article: Article) -> impl IntoView {
    let article_url = format!("/articles/{}", article.document_id);
    
    view! {
        <div class="bg-white rounded-xl overflow-hidden shadow-sm border border-gray-100 transition-all hover:shadow-md hover:border-primary/20 group">
            <div class="p-6">
                <h3 class="text-xl font-bold mb-2 text-neutralDark group-hover:text-primary transition-colors">{article.title.clone()}</h3>
                {
                    if let Some(ref subtitle) = article.subtitle {
                        view! {<p class="text-gray-600 mb-4 line-clamp-2">{subtitle.clone()}</p>}.into_any()
                    } else {
                        view! {<p class="text-gray-600 mb-4">""</p>}.into_any()
                    }
                }
                <div class="flex flex-wrap gap-2 mb-4">
                    {
                        if let Some(tags) = article.get_tags() {
                            tags.into_iter().map(|tag| {
                                view! {
                                    <span class="text-xs font-medium text-primary bg-primary/10 px-3 py-1 rounded-full">{tag.clone()}</span>
                                }
                            }).collect_view().into_any()
                        } else {
                            view! {<span class="hidden">""</span>}.into_any()
                        }
                    }
                </div>
                <div class="w-full flex items-center justify-between pt-2 border-t border-gray-100">
                    <span class="text-sm text-gray-500">
                        {
                            if let Some(date) = article.get_published_date() {
                                date
                            } else {
                                "".to_string()
                            }
                        }
                    </span>
                    <a href={article_url} class="text-primary font-medium group-hover:translate-x-1 transition-transform flex items-center">
                        "Read More"
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4 ml-1" viewBox="0 0 20 20" fill="currentColor">
                            <path fill-rule="evenodd" d="M10.293 5.293a1 1 0 011.414 0l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414-1.414L12.586 11H5a1 1 0 110-2h7.586l-2.293-2.293a1 1 0 010-1.414z" clip-rule="evenodd" />
                        </svg>
                    </a>
                </div>
            </div>
        </div>
    }
}
