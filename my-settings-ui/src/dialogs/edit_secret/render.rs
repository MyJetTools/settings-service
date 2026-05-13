use std::rc::Rc;

use dioxus::prelude::*;

use dioxus_utils::*;

use crate::icons::*;
use crate::models::*;
use crate::states::MainState;

use super::super::*;

use super::state::*;

#[component]
pub fn EditSecret(
    env_id: Rc<String>,
    product_id: Option<Rc<String>>,
    secret_id: Rc<String>,
    clone_from: Option<Rc<String>>,
    on_ok: EventHandler<UpdateSecretValueHttpModel>,
) -> Element {
    let is_clone = clone_from.is_some();
    let mut cs = use_signal(|| EditSecretState::new(secret_id.to_string(), &product_id, is_clone));
    let cs_ra = cs.read();

    let load_secret_id: Rc<String> = if let Some(src) = clone_from.as_ref() {
        src.clone()
    } else {
        secret_id.clone()
    };

    match get_data(cs, &cs_ra, &env_id, &product_id, &load_secret_id) {
        Ok(_) => {}
        Err(err) => return err,
    };

    let ms = consume_context::<Signal<MainState>>();
    let ms_ra = ms.read();

    let product_select = crate::components::select_product(
        &ms_ra,
        Some("Shared"),
        cs_ra.product_id.as_deref(),
        !cs_ra.new_secret,
        EventHandler::new(move |value| {
            cs.write().product_id = value;
        }),
    );

    let box_shadow = if cs_ra.product_id.is_none() {
        "box-shadow: 0 0 7px yellow;"
    } else {
        "box-shadow: 0 0 0px lightgray"
    };

    let content = rsx! {

        div { class: "form-floating mb-3", style: box_shadow,
            {product_select}
            label { "Product scope" }
        }

        div { class: "form-floating mb-3",
            input {
                class: "form-control",
                disabled: !cs_ra.new_secret,
                oninput: move |cx| {
                    cs.write().secret_id = cx.value();
                },
                value: cs_ra.secret_id.as_str(),
            }
            label { "Secret name" }
        }

        div { class: "form-floating mb-3",
            input {
                class: "form-control",
                oninput: move |cx| {
                    cs.write().value.value = cx.value();
                },
                value: cs_ra.value.value.as_str(),
            }
            label { "Secret value" }
        }

        div { class: "form-floating mb-3",
            input {
                class: "form-control",
                oninput: move |cx| {
                    cs.write().value.remote_value = cx.value();
                },
                value: cs_ra.value.remote_value.as_str(),
            }
            label { "Remote value (optional)" }
        }

        div { class: "form-floating mb-3",
            input {
                class: "form-control",
                r#type: "number",
                oninput: move |cx| {
                    cs.write().value.level = cx.value();
                },
                value: cs_ra.value.level.as_str(),
            }
            label { "Secret level" }
        }

        div { class: "form-floating mb-3",
            textarea {
                class: "form-control",
                style: "height: 100px;",
                oninput: move |cx| {
                    cs.write().value.description = cx.value();
                },
                value: cs_ra.value.description.as_str(),
            }
            label { "Description (optional)" }
        }

        div {
            class: "form-check mb-3",
            style: "padding-top: 4px;",
            input {
                class: "form-check-input",
                id: "visible-for-mcp-checkbox",
                r#type: "checkbox",
                checked: cs_ra.value.visible_for_mcp,
                onchange: move |cx| {
                    cs.write().value.visible_for_mcp = cx.value() == "true";
                },
            }
            label {
                class: "form-check-label",
                r#for: "visible-for-mcp-checkbox",
                style: "margin-left: 6px;",
                "Visible for MCP / AI agents"
            }
            div {
                style: "font-size: 0.85em; color: #777; margin-top: 4px;",
                "When on, AI can read this secret's value through the MCP tool `get_secret_value`. Keep off for credentials; turn on only for non-sensitive config."
            }
        }
    };

    let header = if is_clone {
        "Clone secret"
    } else if cs_ra.new_secret {
        "New secret"
    } else {
        "Edit secret"
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
    mut cs: Signal<EditSecretState>,
    cs_ra: &EditSecretState,
    env_id: &str,
    product_id: &Option<Rc<String>>,
    secret_id: &str,
) -> Result<(), Element> {
    match cs_ra.value_on_init.as_ref() {
        RenderState::None => {
            let env_id = env_id.to_string();
            let product_id = match product_id {
                Some(product_id) => Some(product_id.to_string()),
                None => None,
            };
            let secret_id = secret_id.to_string();
            spawn(async move {
                cs.write().value_on_init.set_loading();
                match crate::api::secrets::load_secret_value(env_id, product_id, secret_id).await {
                    Ok(value) => {
                        cs.write().init_value(SecretValue {
                            value: value.value,
                            level: value.level.to_string(),
                            remote_value: value.remote_value.unwrap_or_default(),
                            description: value.description.unwrap_or_default(),
                            visible_for_mcp: value.visible_for_mcp,
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
