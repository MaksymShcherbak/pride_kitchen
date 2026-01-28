use crate::{PrideFlag, SYMBOL_ASSETS};
use dioxus::prelude::*;
fn reduce_eye_strain(hex: &str) -> String {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap() as f32 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap() as f32 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap() as f32 / 255.0;

    // Detect near-grayscale
    if (r - g).abs() < 0.02 && (r - b).abs() < 0.02 && (g - b).abs() < 0.02 {
        let mut l = (r + g + b) / 3.0;
        l = l.clamp(0.2, 0.85); // wider range for grays
        let val = (l * 255.0).round() as u8;
        return format!("#{val:02X}{val:02X}{val:02X}");
    }

    // Convert to HSL
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;

    let delta = max - min;
    let s = if delta == 0.0 {
        0.0
    } else {
        delta / (1.0 - (2.0 * l - 1.0).abs())
    };

    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        ((g - b) / delta).rem_euclid(6.0)
    } else if max == g {
        ((b - r) / delta) + 2.0
    } else {
        ((r - g) / delta) + 4.0
    } * 60.0;

    // Softening adjustments
    let l = l.clamp(0.3, 0.8); // allow more contrast
    let s = s.clamp(0.5, 0.8); // allow more saturation

    // Convert back to RGB
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r1, g1, b1) = match h {
        h if h < 60.0 => (c, x, 0.0),
        h if h < 120.0 => (x, c, 0.0),
        h if h < 180.0 => (0.0, c, x),
        h if h < 240.0 => (0.0, x, c),
        h if h < 300.0 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    let r = ((r1 + m) * 255.0).round() as u8;
    let g = ((g1 + m) * 255.0).round() as u8;
    let b = ((b1 + m) * 255.0).round() as u8;

    format!("#{r:02X}{g:02X}{b:02X}")
}

#[derive(Props, PartialEq, Clone)]
pub struct FlagProps {
    flag: PrideFlag,
    id: String,
    reduce_eye_strain: bool,
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
                for (i , (idx1 , idx2)) in grads.iter().enumerate() {
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
                        rsx! {
                            linearGradient {
                                id: format!("grad{i}"),
                                x1: "10%",
                                x2: "90%",
                                y1: "0%",
                                y2: "0%",
                                stop {
                                    offset: "0%",
                                    stop_color: if props.reduce_eye_strain { reduce_eye_strain(&color1) } else { color1 },
                                }
                                stop {
                                    offset: "100%",
                                    stop_color: if props.reduce_eye_strain { reduce_eye_strain(&color2) } else { color2 },
                                }
                            }
                        }
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
                        PrideFlag::Single(data) => {
                            if props.reduce_eye_strain {
                                reduce_eye_strain(&data.lines[i].to_string())
                            } else {
                                data.lines[i].to_string()
                            }
                        }
                        PrideFlag::Merged(_, _) => format!("url(#grad{i})"),
                    },
                    shape_rendering: "crispEdges",
                }
            }

            {
                match &flag {
                    PrideFlag::Single(data) => rsx! {
                        if let Some(symbol) = &data.symbol
                            && let Some(a) = SYMBOL_ASSETS.get(symbol.src.as_str())
                        {
                            image {
                                x: symbol.single.x,
                                y: symbol.single.y,
                                width: symbol.single.width,
                                height: symbol.single.height,
                                href: "{a}",
                            }
                        }
                    },
                    PrideFlag::Merged(data1, data2) => rsx! {
                        if let Some(symbol) = &data1.symbol
                            && let Some(a) = SYMBOL_ASSETS.get(symbol.src.as_str())
                        {
                            image {
                                x: symbol.merged_left.x,
                                y: symbol.merged_left.y,
                                width: symbol.merged_left.width,
                                height: symbol.merged_left.height,
                                href: "{a}",
                            }
                        }
                        if let Some(symbol) = &data2.symbol
                            && let Some(a) = SYMBOL_ASSETS.get(symbol.src.as_str())
                        {
                            image {
                                x: width - symbol.merged_left.x - symbol.merged_left.width,
                                y: symbol.merged_left.y,
                                width: symbol.merged_left.width,
                                height: symbol.merged_left.height,
                                href: "{a}",
                                transform: if let Some(true) = data2.mirror_symbol { format!("translate({} 0) scale(-1 1)", width - symbol.merged_left.x) } else { "" },
                            }
                        }
                    },
                }
            }
        }
    }
}
