use std::rc::Rc;

use dioxus::prelude::*;

use dioxus_utils::*;

use crate::icons::*;
use crate::models::*;

use super::super::*;

use super::state::*;

#[component]
pub fn EditProduct(
    env_id: Rc<String>,
    product_id: Rc<String>,
    on_ok: EventHandler<UpdateProductHttpModel>,
) -> Element {
    let mut cs = use_signal(|| EditProductState::new(product_id.to_string()));
    let cs_ra = cs.read();

    match get_data(cs, &cs_ra, &env_id, &product_id) {
        Ok(_) => {}
        Err(err) => return err,
    };

    let id_input = if cs_ra.new_product {
        rsx! {
            div { class: "form-floating mb-3",
                input {
                    class: "form-control",
                    oninput: move |cx| {
                        cs.write().product_id = cx.value();
                    },
                    value: cs_ra.product_id.as_str(),
                }
                label { "Product id" }
            }
        }
    } else {
        rsx! {
            div { class: "form-floating mb-3",
                input {
                    class: "form-control",
                    disabled: true,
                    value: cs_ra.product_id.as_str(),
                }
                label { "Product id" }
            }
        }
    };

    let content = rsx! {

        {id_input}

        div { class: "form-floating mb-3",
            input {
                class: "form-control",
                oninput: move |cx| {
                    cs.write().value.description = cx.value();
                },
                value: cs_ra.value.description.as_str(),
            }
            label { "Description" }
        }

        div { class: "form-floating mb-3",
            textarea {
                class: "form-control",
                style: "height: 280px;",
                oninput: move |cx| {
                    cs.write().value.prompt = cx.value();
                },
                value: cs_ra.value.prompt.as_str(),
            }
            label { "Prompt (read by AI agents to understand the product)" }
        }
    };

    let header = if cs_ra.new_product {
        "New product"
    } else {
        "Edit product"
    };

    rsx! {

        DialogTemplate {
            header,
            content,
            ok_button: rsx! {
                button {
                    class: "btn btn-primary",
                    disabled: cs_ra.save_button_is_disabled(),
                    onclick: move |_| {
                        let result = cs.read().get_result();
                        on_ok.call(result);
                        consume_context::<Signal<DialogState>>().set(DialogState::None);
                    },
                    OkButtonIcon {}
                    "Save"
                }
            },
        }
    }
}

fn get_data(
    mut cs: Signal<EditProductState>,
    cs_ra: &EditProductState,
    env_id: &str,
    product_id: &str,
) -> Result<(), Element> {
    match cs_ra.value_on_init.as_ref() {
        RenderState::None => {
            let env_id = env_id.to_string();
            let product_id = product_id.to_string();
            spawn(async move {
                cs.write().value_on_init.set_loading();
                match crate::api::products::load_product(env_id, product_id).await {
                    Ok(value) => {
                        cs.write()
                            .init_value(ProductValue {
                                description: value.description.unwrap_or_default(),
                                prompt: value.prompt.unwrap_or_default(),
                            });
                    }
                    Err(err) => {
                        cs.write().value_on_init.set_error(err.to_string());
                    }
                };
            });

            return Err(crate::icons::loading_icon());
        }
        RenderState::Loading => {
            return Err(crate::icons::loading_icon());
        }

        RenderState::Loaded(_) => {
            return Ok(());
        }

        RenderState::Error(err) => {
            return Err(crate::icons::render_error(err));
        }
    }
}
