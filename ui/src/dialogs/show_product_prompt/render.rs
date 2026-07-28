use std::rc::Rc;

use dioxus::prelude::*;

use dioxus_utils::*;

use crate::models::*;

use super::super::*;

#[component]
pub fn ShowProductPrompt(env_id: Rc<String>, product_id: Rc<String>) -> Element {
    let mut state = use_signal(|| DataState::<ProductHttpModel>::new());

    let read_access = state.read();

    let body = match read_access.as_ref() {
        RenderState::None => {
            let env_id_str = env_id.to_string();
            let product_id_str = product_id.to_string();
            spawn(async move {
                state.write().set_loading();
                match crate::api::products::load_product(env_id_str, product_id_str).await {
                    Ok(value) => {
                        state.write().set_loaded(value);
                    }
                    Err(err) => {
                        state.write().set_error(err.to_string());
                    }
                }
            });
            crate::icons::loading_icon()
        }
        RenderState::Loading => crate::icons::loading_icon(),
        RenderState::Error(err) => crate::icons::render_error(err),
        RenderState::Loaded(value) => {
            if !value.has_metadata {
                rsx! {
                    div { style: "padding: 20px; color: #555;",
                        "No description or prompt has been recorded for this product yet."
                    }
                }
            } else {
                let description = value.description.clone().unwrap_or_default();
                let prompt = value.prompt.clone().unwrap_or_default();

                let description_block = if description.trim().is_empty() {
                    rsx! {
                        div { style: "color: #888;", "(no description)" }
                    }
                } else {
                    rsx! {
                        div { style: "white-space: pre-wrap;", "{description}" }
                    }
                };

                let prompt_block = if prompt.trim().is_empty() {
                    rsx! {
                        div { style: "color: #888;", "(no prompt)" }
                    }
                } else {
                    rsx! {
                        pre { style: "white-space: pre-wrap; word-break: break-word; background: #f6f8fa; padding: 12px; border-radius: 4px;",
                            "{prompt}"
                        }
                    }
                };

                rsx! {
                    div { style: "margin-bottom: 16px;",
                        h5 { "Description" }
                        {description_block}
                    }
                    div {
                        h5 { "Prompt" }
                        {prompt_block}
                    }
                }
            }
        }
    };

    rsx! {
        DialogTemplate {
            header: format!("Product '{}'", product_id.as_str()),
            content: body,
            width: "720px".to_string(),
        }
    }
}
