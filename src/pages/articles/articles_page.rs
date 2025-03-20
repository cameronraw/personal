use leptos::prelude::*;
use leptos_meta::Title;

use crate::models::article::Article;

#[server]
pub async fn get_all_articles() -> Result<String, ServerFnError> {
    let response = reqwest::get("http://localhost:1337/articles")
        .await?
        .text()
        .await?;

    Ok(response)
}

#[component]
pub fn ArticlesPage() -> impl IntoView {
    let article_resource = Resource::new(|| (), |_| get_all_articles());

    view! {
        <Title text="Articles | Cameron Raw"/>
        <div class="py-16 bg-gray-50">
            <div class="container mx-auto px-4">
                <h1 class="text-4xl font-bold mb-4 text-primary">"Articles"</h1>
                <p class="text-lg text-neutralDark mb-12 max-w-3xl">"Thoughts, tutorials, and insights on software development and technology."</p>

                <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                    <Suspense fallback=move || view! {
                        <div class="col-span-full text-center py-12">
                            <div class="inline-block animate-spin rounded-full h-8 w-8 border-t-2 border-b-2 border-primary"></div>
                            <p class="mt-4 text-gray-600">"Loading articles..."</p>
                        </div>
                    }>
                        {move || match article_resource.get() {
                            Some(Ok(article_string)) => {
                                match serde_json::from_str::<Vec<Article>>(&article_string) {
                                    Ok(articles) => {
                                        if articles.is_empty() {
                                            view! {
                                                <div class="col-span-full text-center py-12">
                                                    <p class="text-xl text-gray-600">"No articles found."</p>
                                                </div>
                                            }.into_any()
                                        } else {
                                            articles.into_iter().map(|article| {
                                                view! {
                                                    <ArticleListItem article=article />
                                                }
                                            }).collect_view().into_any()
                                        }
                                    },
                                    Err(parse_error) => {
                                        leptos::logging::log!("Error parsing articles: {:?}", parse_error);
                                        view! {
                                            <div class="col-span-full text-center py-12">
                                                <p class="text-xl text-red-600">"Error parsing articles"</p>
                                            </div>
                                        }.into_any()
                                    }
                                }
                            },
                            Some(Err(_)) => view! {
                                <div class="col-span-full text-center py-12">
                                    <p class="text-xl text-red-600">"Error loading articles"</p>
                                </div>
                            }.into_any(),
                            None => view! {
                                <div class="col-span-full text-center py-12">
                                    <div class="inline-block animate-spin rounded-full h-8 w-8 border-t-2 border-b-2 border-primary"></div>
                                    <p class="mt-4 text-gray-600">"Loading articles..."</p>
                                </div>
                            }.into_any()
                        }}
                    </Suspense>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ArticleListItem(article: Article) -> impl IntoView {
    let article_slug = article.title.to_lowercase().replace(" ", "-");
    let article_url = format!("/articles/{}", article_slug);

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
