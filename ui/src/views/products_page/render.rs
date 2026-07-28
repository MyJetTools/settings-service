use std::rc::Rc;

use dioxus::prelude::*;

use dioxus_utils::*;

use crate::{dialogs::*, icons::*, models::*, states::*, ui_utils::ToastType};

#[component]
pub fn ProductsPage() -> Element {
    let ms = consume_context::<Signal<MainState>>();

    let ms_ra = ms.read();

    let selected_env_id = Rc::new(crate::storage::selected_env::get());

    let products = match get_data(ms, &ms_ra, selected_env_id.as_str()) {
        Ok(products) => products,
        Err(err) => {
            return err;
        }
    };

    let rows = products.iter().cloned().map(|itm| {
        let product_id = Rc::new(itm.id.clone());
        let env_id_view = selected_env_id.clone();
        let env_id_edit = selected_env_id.clone();
        let env_id_delete = selected_env_id.clone();

        let view_btn = if itm.has_metadata {
            let product_id = product_id.clone();
            rsx! {
                button {
                    class: "btn btn-sm btn-success",
                    title: "View prompt (read-only)",
                    onclick: move |_| {
                        let env_id = env_id_view.clone();
                        let product_id = product_id.clone();
                        consume_context::<Signal<DialogState>>()
                            .set(DialogState::ShowProductPrompt {
                                env_id,
                                product_id,
                            });
                    },
                    {view_template_icon()}
                }
            }
        } else {
            rsx! {}
        };

        let edit_btn = {
            let product_id = product_id.clone();
            rsx! {
                button {
                    class: "btn btn-sm btn-primary",
                    onclick: move |_| {
                        let env_id = env_id_edit.clone();
                        let product_id = product_id.clone();
                        consume_context::<Signal<DialogState>>()
                            .set(DialogState::EditProduct {
                                env_id: env_id.clone(),
                                product_id,
                                on_ok: EventHandler::new(move |value| {
                                    exec_save_product(env_id.to_string(), value);
                                }),
                            });
                    },
                    EditIcon {}
                }
            }
        };

        let delete_btn = if itm.has_metadata {
            let product_id_for_delete = product_id.clone();
            rsx! {
                button {
                    class: "btn btn-sm btn-danger",
                    onclick: move |_| {
                        let product_id = product_id_for_delete.clone();
                        let env_id = env_id_delete.clone();
                        consume_context::<Signal<DialogState>>()
                            .set(DialogState::Confirmation {
                                content: format!("Delete product description '{}'?", product_id.as_str()),
                                on_ok: EventHandler::new(move |_| {
                                    exec_delete_product(env_id.to_string(), product_id.to_string());
                                }),
                            });
                    },
                    DeleteIcon {}
                }
            }
        } else {
            rsx! {}
        };

        let metadata_badge = if itm.has_metadata {
            rsx! {
                span { class: "badge text-bg-success", "explicit" }
            }
        } else {
            rsx! {
                span { class: "badge text-bg-warning", "implicit" }
            }
        };

        let description = itm.description.clone().unwrap_or_default();

        rsx! {
            tr { style: "border-top: 1px solid lightgray;",
                td { style: "padding: 10px",
                    span { style: "font-weight: 600", "{itm.id}" }
                }
                td { {metadata_badge} }
                td { style: "padding: 10px; color: #555;", "{description}" }
                td { style: "text-align:center", "{itm.templates_amount}" }
                td { style: "text-align:center", "{itm.secrets_amount}" }
                td {
                    div { class: "btn-group",
                        {view_btn}
                        {edit_btn}
                        {delete_btn}
                    }
                }
            }
        }
    });

    let add_env_id = selected_env_id.clone();

    rsx! {
        table { class: "table table-striped", style: "text-align: left;",
            thead {
                tr {
                    th { style: "padding: 10px; width:25%", "Product id" }
                    th { "Type" }
                    th { style: "width:40%", "Description" }
                    th { style: "text-align:center", "Templates" }
                    th { style: "text-align:center", "Secrets" }
                    th {
                        div {
                            button {
                                class: "btn btn-sm btn-primary",
                                onclick: move |_| {
                                    let env_id = add_env_id.clone();
                                    consume_context::<Signal<DialogState>>()
                                        .set(DialogState::EditProduct {
                                            env_id: env_id.clone(),
                                            product_id: Rc::new(String::new()),
                                            on_ok: EventHandler::new(move |value| {
                                                exec_save_product(env_id.to_string(), value);
                                            }),
                                        });
                                },
                                AddIcon {}
                            }
                        }
                    }
                }
            }
            tbody { {rows} }
        }
    }
}

fn get_data<'s>(
    mut ms: Signal<MainState>,
    ms_ra: &'s MainState,
    env_id: &str,
) -> Result<&'s [ProductHttpModel], Element> {
    match ms_ra.products_list.as_ref() {
        RenderState::None => {
            let env_id = env_id.to_string();
            spawn(async move {
                ms.write().products_list.set_loading();
                match crate::api::products::load_all_products(env_id).await {
                    Ok(value) => {
                        ms.write().products_list.set_value(value);
                    }
                    Err(err) => {
                        ms.write().products_list.set_error(err.to_string());
                    }
                }
            });
            return Err(crate::icons::loading_icon());
        }
        RenderState::Loading => {
            return Err(crate::icons::loading_icon());
        }
        RenderState::Loaded(value) => Ok(value.as_slice()),
        RenderState::Error(err) => {
            return Err(crate::icons::render_error(err));
        }
    }
}

fn exec_save_product(env_id: String, value: UpdateProductHttpModel) {
    let mut main_state = consume_context::<Signal<MainState>>();
    spawn(async move {
        match crate::api::products::save_product(env_id, value).await {
            Ok(_) => {
                main_state.write().products_list.reset();
                crate::ui_utils::show_toast("Product is saved", ToastType::Info);
            }
            Err(_) => {
                crate::ui_utils::show_toast("Error saving product", ToastType::Error);
            }
        }
    });
}

fn exec_delete_product(env_id: String, product_id: String) {
    let mut main_state = consume_context::<Signal<MainState>>();
    spawn(async move {
        match crate::api::products::delete_product(env_id, product_id).await {
            Ok(_) => {
                main_state.write().products_list.reset();
                crate::ui_utils::show_toast("Product is deleted", ToastType::Info);
            }
            Err(_) => {
                crate::ui_utils::show_toast("Error deleting product", ToastType::Error);
            }
        }
    });
}
