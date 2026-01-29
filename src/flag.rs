use crate::IMG_ASSETS;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
struct Transform {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct SymbolData {
    pub src: String,
    pub single: Transform,
    pub merged_left: Transform,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct FlagData {
    pub full_name: String,
    pub name: String,
    pub lines: Vec<String>,
    pub symbol: Option<SymbolData>,
    pub mirror_symbol: Option<bool>,
    pub categories: Vec<String>,
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
pub enum PrideFlag {
    Single(FlagData),
    Merged(FlagData, FlagData),
}

#[derive(Props, PartialEq, Clone)]
pub struct FlagProps {
    pub flag: PrideFlag,
    pub id: String,
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
                        if let Some(symbol) = &data.symbol && let Some(a) = IMG_ASSETS.get(symbol.src.as_str()) {
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
                         if let Some(symbol) = &data1.symbol && let Some(a) = IMG_ASSETS.get(symbol.src.as_str()) {
                            image {
                                x: symbol.merged_left.x,
                                y: symbol.merged_left.y,
                                width: symbol.merged_left.width,
                                height: symbol.merged_left.height,
                                href: "{a}",
                            }
                        }
                         if let Some(symbol) = &data2.symbol && let Some(a) = IMG_ASSETS.get(symbol.src.as_str()) {
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
