use dioxus::prelude::*;
use itertools::Itertools;
use rand::rng;
use rand::seq::IndexedRandom;
use std::ops::Deref;

use crate::FLAGS;
use crate::flag::Flag;
use crate::flag_lib::FlagData;

#[component]
pub fn App() -> Element {
    let mut flag1 = use_signal::<Option<FlagData>>(|| None);
    let mut flag2 = use_signal::<Option<FlagData>>(|| None);
    let flag_mix = if let (Some(f1), Some(f2)) = (&*flag1.read(), &*flag2.read()) {
        Some(FlagData::mix(f1, f2))
    } else {
        None
    };

    let mut selected_slot = use_signal::<usize>(|| 1);
    let get_unselected_flag = move || {
        if *selected_slot.read() == 2 {
            flag1.read().clone()
        } else {
            flag2.read().clone()
        }
    };

    let swap_flags = move |_| {
        let (f1, f2) = (flag1.read().clone(), flag2.read().clone());
        flag1.set(f2);
        flag2.set(f1);
    };

    let mut option_icons = use_signal::<bool>(|| true);
    let mut option_reduce_strain = use_signal::<bool>(|| false);
    let mut option_softness = use_signal::<f32>(|| 40.0);
    let mut option_blur = use_signal::<f32>(|| 0.0);

    let mut select_flag = move |flag: FlagData| {
        let slot = *selected_slot.read();
        if slot == 1 {
            flag1.set(Some(flag));
            if (*flag2.read()).is_none() {
                selected_slot.set(2)
            }
        } else if slot == 2 {
            flag2.set(Some(flag));
            if (*flag1.read()).is_none() {
                selected_slot.set(1)
            }
        }
    };

    let select_random_flags = move |_| {
        let mut rng = rng();
        let flags = FLAGS.deref().clone();
        let flag_a = flags.choose(&mut rng).unwrap().clone();
        let mut flag_b = flags.choose(&mut rng).unwrap().clone();

        while flag_a.name == flag_b.name || !FlagData::is_compatible(&flag_a, &flag_b) {
            flag_b = flags.choose(&mut rng).unwrap().clone();
        }

        flag1.set(Some(flag_a));
        flag2.set(Some(flag_b));
    };

    let mut query = use_signal::<String>(String::new);

    // Categories and whether they are open
    let mut categories = use_signal::<Vec<(&'static str, bool)>>(|| {
        vec![
            ("sexual orientation", true),
            ("gender identity", true),
            ("romantic orientation", true),
            ("other", true),
        ]
    });
    let mut toggle_category = move |i: usize| {
        let mut cs = categories.read().clone();
        cs[i].1 = !cs[i].1;
        categories.set(cs);
    };

    let category_names = [
        "Sexual Orientation",
        "Gender Identity / Sex",
        "Romantic Orientation",
        "Other Identities",
    ];

    rsx! {
        document::Stylesheet { href: asset!("assets/main.css") }
        document::Link { rel: "icon", href: asset!("assets/favicon.ico") }
        document::Script { src: asset!("assets/client.js") }
        header {
            class: "header",
            background_image: format!("url('{}')", get_asset!("header-bg.svg")),
            h1 {
                class: "title",
                "Pride Kitchen 🍴"
            }
            h2 {
                class: "subtitle",
                span { "Made by "}
                a {
                    class: "author-link",
                    href: "https://github.com/MaksymShcherbak",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "Maksym Shcherbak"
                }
            }
            div {
                class: "slot-row",
                div {
                    class: if *selected_slot.read() == 1 { "flag-slot selected" } else {"flag-slot" },
                    onclick: move |_| selected_slot.set(1),
                    if let Some(flag) = &*flag1.read() {
                        Flag {
                            key: "f1-{option_icons}",
                            flag: flag.clone(),
                            id: "f1".to_string(),
                            option_icons: *option_icons.read(),
                            option_reduce_strain: *option_reduce_strain.read(),
                            option_blur: *option_blur.read(),
                            option_softness: *option_softness.read()
                        }
                        p { class: "flag-slot-name", {flag.name.to_string()} }
                    }
                }
                p { class: "flag-slot-op", "+" }
                div {
                    class: if *selected_slot.read() == 2 { "flag-slot selected" } else {"flag-slot" },
                    onclick: move |_| selected_slot.set(2),
                    if let Some(flag) = &*flag2.read() {
                        Flag {
                            key: "f2-{option_icons}",
                            flag: flag.clone(),
                            id: "f2".to_string(),
                            option_icons: *option_icons.read(),
                            option_reduce_strain: *option_reduce_strain.read(),
                            option_blur: *option_blur.read(),
                            option_softness: *option_softness.read()
                        }
                        p { class: "flag-slot-name", {flag.name.to_string()} }
                    }
                }
                p { class: "flag-slot-op", "=" }
                div {
                    class: "flag-slot",
                    if let Some(flag) = &flag_mix {
                        Flag {
                            key: "k-{option_icons.read()}",
                            flag: flag.clone(),
                            id: "result".to_string(),
                            option_icons: *option_icons.read(),
                            option_reduce_strain: *option_reduce_strain.read(),
                            option_blur: *option_blur.read(),
                            option_softness: *option_softness.read()
                        },
                        p { {flag.name.to_string()} }
                    }
                }
            }
            div {
                class: "input-row",
                button {
                    onclick: select_random_flags,
                    img { src: get_asset!("dice.svg") },
                    "I'm Feeling Lucky",
                }
                button {
                    class: if flag_mix.is_none() { "disabled" } else { "" },
                    onclick: swap_flags,
                    img { src: get_asset!("swap.svg") },
                    "Swap"
                },
                button {
                    class: if flag_mix.is_none() { "disabled" } else { "" },
                    id: "download-png-btn".to_string(),
                    img { src: get_asset!("download.svg") },
                    "Download PNG"
                },
                button {
                    class: if flag_mix.is_none() {"disabled" } else { "" },
                    id: "download-svg-btn".to_string(),
                    img { src: get_asset!("download.svg") },
                    "Download SVG"
                },
            },
        },
        div {
            class: "input-row2",
            input {
                type: "text",
                class: "search",
                placeholder: "🔎 Search Flags...",
                value: "{query}",
                oninput: move |event| { query.set(event.value()) },
            },
            div {
                onclick: move |_| {
                    let current = *option_icons.read();
                    option_icons.set(!current);
                },
                input {
                    type: "checkbox",
                    id: "icon-toggle",
                    checked: *option_icons.read(),
                },
                label { for: "icon-toggle", "Show Symbols" }
            },
            div {
                onclick: move |_| {
                    let current = *option_reduce_strain.read();
                    option_reduce_strain.set(!current);
                },
                input {
                    type: "checkbox",
                    id: "strain-toggle",
                    checked: *option_reduce_strain.read(),
                },
                label { for: "strain-toggle", "Reduce Eye Strain" }
            },
            div {
                input {
                    type: "range",
                    min: 0.0,
                    max: 49.,
                    id: "softness-toggle",
                    value: *option_softness.read(),
                    onchange: move |e| { option_softness.set(e.value().parse().expect("Can't parse the blur value")) },
                },
                label { for: "softness-toggle", "Softness" }
            }
            div {
                input {
                    type: "range",
                    min: 0.0,
                    max: 25.0,
                    id: "blur-toggle",
                    value: *option_blur.read(),
                    onchange: move |e| { option_blur.set(e.value().parse().expect("Can't parse the blur value")) },
                },
                label { for: "blur-toggle", "Blur" }
            }
        },
        div {
            class: "flag-chooser",
            for (i, c) in categories.iter().enumerate() {
                div {
                    class: "category",
                    div {
                        class: "category-header",
                        onclick: move |_| toggle_category(i),
                        h1 { {category_names[i]} }
                        img {
                            src: get_asset!("arrow.svg"),
                            transform: if c.1 {
                                "scale(-1)"
                            } else { "" }
                        }
                    }
                    div {
                        class: "flag-list",
                        if c.1 {
                            for (index, flag) in FLAGS.deref()
                                .clone().into_iter()
                                .filter(|flag| flag.categories.contains(c.0))
                                .filter(|flag| flag.full_name.to_lowercase().contains(&query.read().to_lowercase()))
                                .sorted()
                                .enumerate() {
                                    div {
                                        class: "flag-chooser-slot",
                                        onclick: move |_| {select_flag(flag.clone());},
                                        Flag {
                                            key: "{i}-{index}-{option_icons}",
                                            flag: flag.clone(),
                                            id: format!("{i}-{index}"),
                                            option_icons: *option_icons.read(),
                                            option_reduce_strain: *option_reduce_strain.read(),
                                            option_blur: *option_blur.read(),
                                            option_softness: *option_softness.read(),
                                            other_flag: get_unselected_flag()
                                        },
                                        p { {flag.full_name.to_string()} }
                                    }
                            }
                        }
                    }
                }
            }
        }
        div {
            class: "footer",
            span { "Thanks for trying out my website! If you enjoyed it, check out the " },
            a {
                href: "https://github.com/MaksymShcherbak/pride_kitchen",
                target: "_blank",
                rel: "noopener noreferrer",
                "source code"
            },
            span { " or find me at:" }
            div {
                a {
                    class: "author-link",
                    href: "https://github.com/MaksymShcherbak",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    img { src: get_asset!("github.svg") },
                    "GitHub"
                }
            },
            div {
                a {
                    class: "author-link",
                    href: "https://www.linkedin.com/in/maksym-shcherbak-11159b3a7/",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    img { src: get_asset!("linkedin.svg") },
                    "LinkedIn"
                }
            }
        }
    }
}
