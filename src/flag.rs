use crate::flag_lib::{FlagData, Position, reduce_strain};
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct FlagProps {
    pub flag: FlagData,
    pub id: String,
    pub option_icons: bool,
    pub option_reduce_strain: bool,
    pub option_blur: f32,
    pub option_softness: f32,
    // for checking compatibility
    pub other_flag: Option<FlagData>,
}
#[component]
pub fn Flag(props: FlagProps) -> Element {
    let (width, height) = (250, 150);
    let flag = props.flag;
    let id = &props.id;

    let hardness = 49. - props.option_softness;
    let gradients: Vec<Element> = flag
        .lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            let stops: Vec<Element> = line
                .iter()
                .enumerate()
                .map(|(j, color)| {
                    let color = if props.option_reduce_strain {
                        reduce_strain(&color.0)
                    } else {
                        color.0.clone()
                    };

                    let offset = (j as f32) / ((line.len() - 1) as f32) * 100.0;
                    rsx! {
                        stop {
                            offset: "{offset}%",
                            stop_color: "{color}",
                        }
                    }
                })
                .collect();
            rsx! {
                linearGradient {
                    id: "grad{id}-{i}",
                    x1: "{hardness}%",
                    y1: "0%",
                    x2: "{100.-hardness}%",
                    y2: "0%",
                    for s in stops {
                        {s}
                    }
                }
            }
        })
        .collect();

    let stripe_height = height as f32 / (flag.lines.len() as f32);
    let len = flag.lines.len();

    let symbols: Vec<Element> = if props.option_icons {
        flag.symbols
            .iter()
            .map(|(symbol, position)| {
                let asset = get_asset!(symbol.src.as_str());

                let t = match position {
                    Position::Single => &symbol.single,
                    Position::MergedLeft => &symbol.merged_left,
                    Position::MergedRight => &symbol.get_merged_right(width),
                };

                let transform = if *position == Position::MergedRight && symbol.mirror {
                    format!("translate({width} 0) scale(-1 1)")
                } else {
                    String::new()
                };

                rsx! {
                    image {
                        href: "{asset}",
                        x: "{t.x}",
                        y: "{t.y}",
                        width: "{t.width}",
                        height: "{t.height}",
                        transform: "{transform}",
                    }
                }
            })
            .collect()
    } else {
        vec![]
    };

    rsx! {
        svg {
            id: id.to_string(),
            width,
            height,
            view_box: format!("0 0 {width} {height}"),
            shape_rendering: "crispEdges",
            opacity: if let Some(other) = &props.other_flag && !FlagData::is_compatible(&flag, other) { 0.4 } else { 1.0 },
            defs {
                for g in gradients {
                    {g}
                },
                filter {
                    id: "blur{id}",
                    x: "-20%",
                    y: "-20%",
                    width: "140%",
                    height: "140%",

                    // Extract RGB, force alpha to 1
                    feColorMatrix {
                        "in": "SourceGraphic",
                        type: "matrix",
                        values: "
                            1 0 0 0 0
                            0 1 0 0 0
                            0 0 1 0 0
                            0 0 0 0 1
                        ",
                        result: "rgb"
                    }

                    feGaussianBlur {
                        "in": "rgb",
                        std_deviation: props.option_blur,
                        result: "blurred"
                    }

                    // Re-apply original alpha
                    feComposite {
                        "in": "blurred",
                        in2: "SourceGraphic",
                        operator: "in"
                    }
                }
            },

            g {
                filter: "url(#blur{id})",
                for i in 0..len {
                    rect {
                        width,
                        height: stripe_height,
                        y: stripe_height * (i as f32),
                        fill: format!("url(#grad{id}-{i})"),
                        shape_rendering: "crispEdges"
                    },
                },
                for s in symbols {
                    {s}
                }
            }
        }
    }
}
