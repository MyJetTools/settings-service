use crate::{states::*, AppRoute};
use dioxus::prelude::*;

const ACTIVE_CLASS: &str = "menu-active";

#[component]
pub fn LeftPanel() -> Element {
    let mut main_state = consume_context::<Signal<MainState>>();

    let main_state_read_access = main_state.read();

    let current_location = main_state_read_access.location.clone();

    let mut secrets_active = "";
    let mut templates_active = "";
    let mut products_active = "";

    match current_location {
        LocationState::None => {}
        LocationState::Templates => {
            templates_active = ACTIVE_CLASS;
        }
        LocationState::Secrets => {
            secrets_active = ACTIVE_CLASS;
        }
        LocationState::Products => {
            products_active = ACTIVE_CLASS;
        }
    }

    let env_badge = if main_state_read_access.user.as_str().len() > 0 {
        rsx! {
            div { style: "position: absolute;bottom: 0;width: var(--left-panel-width);",
                {main_state_read_access.user.as_str()}
            }
        }
    } else {
        rsx! {}
    };

    rsx! {
        h1 { "Settings" }

        {env_badge}

        div { id: "menu",
            div { class: "menu-item {secrets_active}",
                Link {
                    to: AppRoute::Secrets,
                    onclick: move |_| {
                        if !current_location.is_secrets() {
                            main_state.write().set_location(LocationState::Secrets);
                        }
                    },
                    "Secrets"
                }
            }
            div { class: "menu-item {templates_active}",
                Link {
                    to: AppRoute::Templates,
                    onclick: move |_| {
                        if !current_location.is_templates() {
                            main_state.write().set_location(LocationState::Templates);
                        }
                    },
                    "Templates"
                }
            }
            div { class: "menu-item {products_active}",
                Link {
                    to: AppRoute::Products,
                    onclick: move |_| {
                        if !current_location.is_products() {
                            main_state.write().set_location(LocationState::Products);
                        }
                    },
                    "Products"
                }
            }
        }
    }
}
