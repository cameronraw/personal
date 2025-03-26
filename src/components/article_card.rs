use leptos::prelude::*;
use leptos::{component, view, IntoView};

use crate::models::article::Article;

#[component]
pub fn ArticleCard(article: Article) -> impl IntoView {
    let article_url = format!("/articles/{}", article.document_id);

    view! {
        <div class="bg-white rounded-xl overflow-hidden shadow-sm border border-gray-100 transition-all hover:shadow-md hover:border-primary/20 group">
            <div class="p-6">
                <div class="flex justify-between">
                    <div>
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
                    </div>
                    {
                        if let Some(ref image) = article.image {
                            let image_src = image.formats.get("thumbnail").unwrap().url.clone();
                            let image_url = format!("http://localhost:1337{}", image_src);

                            view! {
                                <div class="w-24 h-24">
                                    <img src={image_url} class="w-full object-cover rounded-lg mb-4"/>
                                </div>
                            }.into_any()
                        } else {
                            view! {<div class="hidden">""</div>}.into_any()
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
