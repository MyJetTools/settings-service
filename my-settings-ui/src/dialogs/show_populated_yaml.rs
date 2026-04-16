use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_utils::{DataState, RenderState};

use crate::dialogs::*;

#[component]
pub fn ShowPopulatedYaml(
    env_id: Rc<String>,
    product_id: Rc<String>,
    template_id: Rc<String>,
) -> Element {
    let cs = use_signal(ShowPopulatedYamlState::new);

    let cs_ra = cs.read();

    let loaded = match get_data(cs, &cs_ra, env_id.clone(), product_id, template_id) {
        Ok(loaded) => loaded,
        Err(err) => return err,
    };

    let local = highlight_yaml_errors(&loaded.local);

    let prefixes_label = render_prefixes_label(&loaded.local_env_prefixes);

    let content = match loaded.remote.as_deref() {
        Some(remote) => {
            let remote = highlight_yaml_errors(remote);
            rsx! {
                div { style: "display:flex; flex-direction:column; height:100%; gap:8px;",
                    {prefixes_label}
                    div {
                        style: "flex:1 1 50%; min-height:0; display:flex; flex-direction:column;",
                        div { style: "font-weight:bold; margin-bottom:4px;", "Local" }
                        div {
                            class: "form-control",
                            style: "flex:1 1 auto; min-height:0; overflow:auto;",
                            {local.into_iter()}
                        }
                    }
                    div {
                        style: "flex:1 1 50%; min-height:0; display:flex; flex-direction:column;",
                        div { style: "font-weight:bold; margin-bottom:4px;", "Remote" }
                        div {
                            class: "form-control",
                            style: "flex:1 1 auto; min-height:0; overflow:auto;",
                            {remote.into_iter()}
                        }
                    }
                }
            }
        }
        None => rsx! {
            div { style: "display:flex; flex-direction:column; height:100%; gap:8px;",
                {prefixes_label}
                div {
                    class: "form-control",
                    style: "flex:1 1 auto; min-height:0; overflow:auto;",
                    {local.into_iter()}
                }
            }
        },
    };

    rsx! {
        DialogTemplate { header: "Populated yaml", allocate_max_space: true, content }
    }
}

fn get_data<'s>(
    mut cs: Signal<ShowPopulatedYamlState>,
    cs_ra: &'s ShowPopulatedYamlState,
    env_id: Rc<String>,
    product_id: Rc<String>,
    template_id: Rc<String>,
) -> Result<&'s LoadedYaml, Element> {
    match cs_ra.yaml.as_ref() {
        RenderState::None => {
            let env_id = env_id.to_string();
            let product_id = product_id.to_string();
            let template_id = template_id.to_string();
            spawn(async move {
                match crate::api::templates::load_yaml(env_id, product_id, template_id).await {
                    Ok(result) => {
                        cs.write().yaml.set_loaded(LoadedYaml {
                            local: result.yaml,
                            remote: result.remote_yaml,
                            local_env_prefixes: result.local_env_prefixes,
                        });
                    }
                    Err(err) => {
                        cs.write().yaml.set_error(err.to_string());
                    }
                }
            });
            return Err(crate::icons::loading_icon());
        }
        RenderState::Loading => Err(crate::icons::loading_icon()),
        RenderState::Loaded(loaded) => Ok(loaded),
        RenderState::Error(err) => Err(crate::icons::render_error(err)),
    }
}

#[derive(Debug, Clone)]
pub struct LoadedYaml {
    pub local: String,
    pub remote: Option<String>,
    pub local_env_prefixes: Vec<String>,
}

fn render_prefixes_label(prefixes: &[String]) -> Element {
    if prefixes.is_empty() {
        return rsx! {
            div { style: "font-size:12px; color:#888;",
                "No local env prefixes configured — every request is treated as "
                b { "Local" }
                ". Header "
                code { "env-info" }
                " is ignored."
            }
        };
    }

    let items = prefixes
        .iter()
        .map(|p| rsx! { code { style: "margin-right:6px; background:#eef; padding:1px 4px;", "{p}*" } })
        .collect::<Vec<_>>();

    rsx! {
        div { style: "font-size:12px; color:#555;",
            "Requests whose "
            code { "env-info" }
            " header starts with any of these prefixes are treated as "
            b { "Local" }
            ", others as "
            b { "Remote" }
            ": "
            {items.into_iter()}
        }
    }
}

pub struct ShowPopulatedYamlState {
    pub yaml: DataState<LoadedYaml>,
}

impl ShowPopulatedYamlState {
    pub fn new() -> Self {
        Self {
            yaml: DataState::new(),
        }
    }
}

fn highlight_yaml_errors(yaml: &str) -> Vec<Element> {
    let mut result = Vec::new();

    for line in yaml.split('\n') {
        let value = highlight(line);

        let line = rsx! {
            div { {value.into_iter()} }
        };

        result.push(line);
    }

    result
}

fn highlight(yaml: &str) -> Vec<Element> {
    let mut yaml = yaml;
    let mut result = Vec::new();

    loop {
        let start_index = yaml.find("/*");

        let Some(start_index) = start_index else {
            result.push(rsx! {
                {yaml}
            });
            return result;
        };

        result.push(rsx! {
            {&yaml[..start_index]}
        });

        yaml = &yaml[start_index..];

        let end_index = yaml.find("*/");

        let Some(end_index) = end_index else {
            result.push(rsx! {
                span { style: "color:red", {yaml} }

            });
            return result;
        };

        result.push(rsx! {
            span { style: "color:red", {&yaml[..end_index + 2]} }

        });

        yaml = &yaml[end_index + 2..];
    }
}
