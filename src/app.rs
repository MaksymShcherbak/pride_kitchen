use dioxus::prelude::*;
use itertools::Itertools;
use std::ops::Deref;

use crate::flag::{Flag, FlagData, PrideFlag};
use crate::{CSS, FAVICON, FLAGS};

#[component]
pub fn App() -> Element {
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
            document::Stylesheet { href: CSS }
            document::Link { rel: "icon", href: FAVICON }
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
                                for flag in FLAGS.deref()
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
