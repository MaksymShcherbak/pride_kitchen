use dioxus::prelude::*;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::HashMap};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

lazy_static::lazy_static! {
    pub static ref SYMBOL_ASSETS: HashMap<&'static str, Asset> = {
        let mut map = HashMap::new();

        // Each asset path must be written out explicitly — asset!() cannot take dynamic input
        map.insert("heart.svg", asset!("/assets/symbols/heart.svg"));
        map.insert("intersex.svg", asset!("/assets/symbols/intersex.svg"));
        map.insert("progress.svg", asset!("/assets/symbols/progress.svg"));
        map.insert("nbmen.svg", asset!("/assets/symbols/nbmen.svg"));
        map.insert("bear.svg", asset!("/assets/symbols/bear.svg"));
        //map.insert("star.svg", asset!("/assets/symbols/star.svg"));
        //map.insert("circle.svg", asset!("/assets/symbols/circle.svg"));

        // Add more entries here manually as needed

        map
    };
}

static FLAGS_JSON: &str = include_str!("../assets/flags.json");

lazy_static::lazy_static! {
    static ref FLAGS: Vec<FlagData> = serde_json::from_str(FLAGS_JSON).expect("Invalid flags.json");
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut flag1 = use_signal::<Option<FlagData>>(|| None);
    let mut flag2 = use_signal::<Option<FlagData>>(|| None);
    let mut selected_slot = use_signal::<Option<usize>>(|| Some(1));
    let mut query = use_signal::<String>(String::new);

    let categories = [
        "sexual orientation",
        "gender identity",
        "romantic orientation",
        "other",
    ];
    let category_names = [
        "Sexual Orientation",
        "Gender Identity / Sex",
        "Romantic Orientation",
        "Other",
    ];

    rsx! {
            document::Link { rel: "icon", href: FAVICON }
            document::Link { rel: "stylesheet", href: MAIN_CSS }
            h1 {
                class: "title",
                "Pride Flag Mixer 🎨"
            }
            h2 {
                class: "subtitle",
                "Made by MaksymShcherbak"
            }
            div {
                    class: "slot-row",
                    div {
                        class: if let Some(1) = *selected_slot.read() { "flag-slot selected" } else {"flag-slot" },
                        onclick: move |_| selected_slot.set(Some(1)),
                        if let Some(flag) = &*flag1.read() {
                            Flag { flag: PrideFlag::Single(flag.clone()), id: "f1".to_string() }
                            p { {flag.name.to_string()} }
                        }
                    }
                    p {
                        "+"
                    }
                    div {
                        class: "flag-slot",
                        class: if let Some(2) = *selected_slot.read() { "flag-slot selected" } else {"flag-slot" },
                        onclick: move |_| selected_slot.set(Some(2)),
                        if let Some(flag) = &*flag2.read() {
                            Flag { flag: PrideFlag::Single(flag.clone()), id: "f2".to_string() }
                            p { {flag.name.to_string()} }
                        }
                    }
                    p {
                        "="
                    } div {
                        class: "flag-slot",
                        if let (Some(flag1), Some(flag2)) = (&*flag1.read(), &*flag2.read()) {
                        Flag { flag:
                            if flag1.full_name != flag2.full_name {
                                PrideFlag::Merged(flag1.clone(), flag2.clone())
                            } else {
                                PrideFlag::Single(flag1.clone())
                            },
                        id: "result".to_string()}
                        p { {if flag1.name != flag2.name {
                            format!("{} {}", flag1.name, flag2.name)
                        } else {
                            flag1.name.to_string()
                        }} }
                    }
                }
            }
            if let Some(slot) = *selected_slot.read() {
                div {
                    class: "input-row",
                    input {
                        type: "text",
                        class: "search",
                        placeholder: "Search Flags...",
                        value: "{query}",
                        oninput: move |event| { query.set(event.value()) },
                    },
                    button {class: if flag1.read().is_none() && flag2.read().is_none() {
                            "disabled"
                        } else { "" },
                        onclick: move |_| {
                            let (f1, f2) = (flag1.read().clone(), flag2.read().clone());
                            flag1.set(f2);
                            flag2.set(f1);
                        },
                        "Swap"
                    },
                    button {
                        class: if flag1.read().is_none() || flag2.read().is_none() {
                            "disabled"
                        } else { "" },
                        onclick: |_| {
        let js = r#"
    const svg = document.getElementById('result');
if (!svg) {
  alert('SVG element not found!');
} else {
  // Inline all <image> hrefs
  const images = svg.querySelectorAll('image');
  const promises = [];

  images.forEach((img) => {
    const href = img.getAttribute('href') || img.getAttribute('xlink:href');
    if (!href || href.startsWith('data:')) return;

    const p = fetch(href)
      .then((res) => res.blob())
      .then((blob) => {
        return new Promise((resolve) => {
          const reader = new FileReader();
          reader.onloadend = () => {
            img.setAttribute('href', reader.result);
            resolve();
          };
          reader.readAsDataURL(blob);
        });
      });

    promises.push(p);
  });

  Promise.all(promises).then(() => {
    const serializer = new XMLSerializer();
    const source = serializer.serializeToString(svg);
    const svgBlob = new Blob([source], { type: 'image/svg+xml;charset=utf-8' });
    const url = URL.createObjectURL(svgBlob);

    const img = new Image();
    img.style.imageRendering = 'pixelated';
    img.onload = function () {
      const scale = 5;
      const canvas = document.createElement('canvas');
      canvas.width = (img.width || 250) * scale;
      canvas.height = (img.height || 150) * scale;
      const ctx = canvas.getContext('2d');

      ctx.imageSmoothingEnabled = false;
      ctx.scale(scale, scale);
      ctx.drawImage(img, 0, 0);
      URL.revokeObjectURL(url);
      canvas.toBlob(function (blob) {
        const a = document.createElement('a');
        a.href = URL.createObjectURL(blob);
        a.download = 'flag.png';
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
      }, 'image/png');
    };
    img.src = url;
  });
}

    "#;
    document::eval(js);
                        },
                        "Download (png)"
                    },
                    button {
                        class: if flag1.read().is_none() || flag2.read().is_none() {
                            "disabled"
                        } else { "" },
                        onclick: |_| {
                            let js = r#"
                            const svg = document.getElementById('result');
if (!svg) {
  alert('SVG element not found!');
} else {
  // Inline all <image> hrefs
  const images = svg.querySelectorAll('image');
  const promises = [];

  images.forEach((img) => {
    const href = img.getAttribute('href') || img.getAttribute('xlink:href');
    if (!href || href.startsWith('data:')) return;

    const p = fetch(href)
      .then((res) => res.blob())
      .then((blob) => {
        return new Promise((resolve) => {
          const reader = new FileReader();
          reader.onloadend = () => {
            img.setAttribute('href', reader.result);
            resolve();
          };
          reader.readAsDataURL(blob);
        });
      });

    promises.push(p);
  });

  Promise.all(promises).then(() => {
    const serializer = new XMLSerializer();
    const source = serializer.serializeToString(svg);
    const blob = new Blob([source], { type: 'image/svg+xml;charset=utf-8' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'flag.svg';
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  });
}

                            "#;
                            document::eval(js);
                        },
                        "Download (svg)"
                    }
                }
                div {
                    class: "flag-chooser",
                    for (i, category) in categories.iter().enumerate() {
                        div {
                            class: "category",
                            h1 { {category_names[i]} }
                            div {
                                class: "flag-list",
                                for flag in FLAGS
                                    .clone().into_iter()
                                    .filter(|flag| flag.categories.contains(&category.to_string()))
                                    .filter(|flag| flag.full_name.to_lowercase().contains(&query.read().to_lowercase()))
                                    .sorted() {
                                    div {
                                        class: "flag-chooser-slot",
                                        onclick: move |_| {
                                            if slot == 1 {
                                                flag1.set(Some(flag.clone()));
                                                if (*flag2.read()).is_none() {
                                                    selected_slot.set(Some(2))
                                                }
                                            }
                                            if slot == 2 {
                                                flag2.set(Some(flag.clone()));
                                                if (*flag1.read()).is_none() {
                                                    selected_slot.set(Some(1))
                                                }
                                            }
                                        },
                                        Flag { flag: PrideFlag::Single(flag.clone()), id: "id".to_string() }
                                        p { {flag.full_name.to_string()} }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
struct Transform {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
struct SymbolData {
    src: String,
    single: Transform,
    merged_left: Transform,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
struct FlagData {
    full_name: String,
    name: String,
    lines: Vec<String>,
    symbol: Option<SymbolData>,
    mirror_symbol: Option<bool>,
    categories: Vec<String>,
}

impl PartialOrd for FlagData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FlagData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

#[derive(PartialEq, Clone)]
enum PrideFlag {
    Single(FlagData),
    Merged(FlagData, FlagData),
}

#[derive(Props, PartialEq, Clone)]
struct FlagProps {
    flag: PrideFlag,
    id: String,
}
#[component]
pub fn Flag(props: FlagProps) -> Element {
    let (width, height) = (250, 150);
    let flag = props.flag;
    let id = props.id;

    // Prepare big and small references
    let (big, small, swapped) = match &flag {
        PrideFlag::Merged(data1, data2) => {
            if data1.lines.len() >= data2.lines.len() {
                (data1.clone(), data2.clone(), false)
            } else {
                (data2.clone(), data1.clone(), true)
            }
        }
        PrideFlag::Single(data) => (data.clone(), data.clone(), false),
    };

    // Clone the lines so we can modify them
    let mut big_lines = big.lines.clone();
    let mut small_lines = small.lines.clone();

    // If the difference in line count is odd, duplicate both
    if (big_lines.len() as isize - small_lines.len() as isize).abs() % 2 == 1 {
        big_lines = big_lines
            .iter()
            .flat_map(|s| [s.clone(), s.clone()])
            .collect();
        small_lines = small_lines
            .iter()
            .flat_map(|s| [s.clone(), s.clone()])
            .collect();
    }

    let count_big = big_lines.len();
    let count_small = small_lines.len();
    let count = count_big;
    let min_count = count_small;
    let pad_top = (count - min_count) / 2;
    let stripe_height = (height as f32 / count as f32).round();
    //height = (stripe_height as usize) * count;

    // Precompute gradient indices for merged flags
    let gradients = if let PrideFlag::Merged(_, _) = &flag {
        Some(
            (0..count)
                .map(|i| {
                    let idx_big = i;
                    let idx_small = if i < pad_top {
                        0
                    } else if i >= pad_top + min_count {
                        min_count - 1
                    } else {
                        i - pad_top
                    };

                    if swapped {
                        (
                            idx_small.min(small_lines.len().saturating_sub(1)),
                            idx_big.min(big_lines.len().saturating_sub(1)),
                        )
                    } else {
                        (
                            idx_big.min(big_lines.len().saturating_sub(1)),
                            idx_small.min(small_lines.len().saturating_sub(1)),
                        )
                    }
                })
                .collect::<Vec<_>>(),
        )
    } else {
        None
    };

    // Precompute gradient elements for RSX
    let gradient_defs = gradients.as_ref().map(|grads| {
        rsx!(
            defs {
                for (i, (idx1, idx2)) in grads.iter().enumerate() {
                    {
                        let color1 = if swapped {
                            small_lines.get(*idx1).cloned().unwrap_or_else(|| "#000000".to_string())
                        } else {
                            big_lines.get(*idx1).cloned().unwrap_or_else(|| "#000000".to_string())
                        };
                        let color2 = if swapped {
                            big_lines.get(*idx2).cloned().unwrap_or_else(|| "#000000".to_string())
                        } else {
                            small_lines.get(*idx2).cloned().unwrap_or_else(|| "#000000".to_string())
                        };

                        rsx!(
                            linearGradient {
                                id: format!("grad{i}"),
                                x1: "10%", x2: "90%",
                                y1: "0%", y2: "0%",
                                stop { offset: "0%", stop_color: "{color1}" }
                                stop { offset: "100%", stop_color: "{color2}" }
                            }
                        )
                    }
                }
            }
        )
    });

    rsx! {
        svg {
            id,
            width,
            height,
            view_box: format!("0 0 {width} {height}"),
            shape_rendering: "crispEdges",
            {gradient_defs}

            for i in 0..count {
                rect {
                    width,
                    height: stripe_height,
                    y: stripe_height * (i as f32),
                    fill: match &flag {
                        PrideFlag::Single(data) => data.lines[i].to_string(),
                        PrideFlag::Merged(_, _) => format!("url(#grad{i})"),
                    },
                    shape_rendering: "crispEdges"
                }
            }

            {
                match &flag {
                    PrideFlag::Single(data) => rsx!{
                        if let Some(symbol) = &data.symbol && let Some(a) = SYMBOL_ASSETS.get(symbol.src.as_str()) {
                            image {
                                x: symbol.single.x,
                                y: symbol.single.y,
                                width: symbol.single.width,
                                height: symbol.single.height,
                                href: "{a}",
                            }
                        }
                    },
                    PrideFlag::Merged(data1, data2) => rsx!{
                         if let Some(symbol) = &data1.symbol && let Some(a) = SYMBOL_ASSETS.get(symbol.src.as_str()) {
                            image {
                                x: symbol.merged_left.x,
                                y: symbol.merged_left.y,
                                width: symbol.merged_left.width,
                                height: symbol.merged_left.height,
                                href: "{a}",
                            }
                        }
                         if let Some(symbol) = &data2.symbol && let Some(a) = SYMBOL_ASSETS.get(symbol.src.as_str()) {
                            image {
                                x: width - symbol.merged_left.x - symbol.merged_left.width,
                                y: symbol.merged_left.y,
                                width: symbol.merged_left.width,
                                height: symbol.merged_left.height,
                                href: "{a}",
                                transform: if let Some(true) = data2.mirror_symbol {
                                    format!("translate({} 0) scale(-1 1)", width - symbol.merged_left.x)
                                } else {"" }
                            }
                        }
                    },
                }
            }
        }
    }
}
