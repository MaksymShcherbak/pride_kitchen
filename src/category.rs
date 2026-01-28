use crate::flag::Flag;
use crate::{FLAGS, FlagData, PrideFlag};
use dioxus::prelude::*;
use itertools::Itertools;

#[derive(Props, PartialEq, Clone)]
pub struct CategoryProps {
    full_name: &'static str,
    name: &'static str,
    query: String,
    reduce_eye_strain: bool,
    onselect: Callback<FlagData, ()>,
}

#[component]
pub fn Category(props: CategoryProps) -> Element {
    let mut hidden = use_signal::<bool>(|| false);

    rsx! {
        div { class: "category",
            div {
                class: "category-header",
                onclick: move |_| {
                    let h = !*hidden.read();
                    hidden.set(h);
                },
                h1 { {props.full_name} }
                img {
                    src: asset!("assets/arrow.svg"),
                    transform: format!("scale(1,{})", if !*hidden.read() { -1 } else { 1 }),
                }
            }
            if !*hidden.read() {
                div { class: "flag-list",
                    for flag in FLAGS
                        .clone()
                        .into_iter()
                        .filter(|flag| flag.categories.contains(&props.name.to_string()))
                        .filter(|flag| flag.full_name.to_lowercase().contains(&props.query))
                        .sorted()
                    {
                        div {
                            class: "flag-chooser-slot",
                            onclick: move |_| { (props.onselect)(flag.clone()) },
                            Flag {
                                flag: PrideFlag::Single(flag.clone()),
                                id: "id".to_string(),
                                reduce_eye_strain: props.reduce_eye_strain,
                            }
                            p { {flag.full_name.to_string()} }
                        }
                    }
                }
            }
        }
    }
}
