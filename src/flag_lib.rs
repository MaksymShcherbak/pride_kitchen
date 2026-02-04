use num_integer::lcm;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::HashSet, iter::repeat_n};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Color(pub String);

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug)]
pub struct Transform {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug)]
pub struct SymbolData {
    pub src: String,
    pub single: Transform,
    pub merged_left: Transform,
    pub mirror: bool,
}

impl SymbolData {
    pub fn get_merged_right(&self, width: i32) -> Transform {
        let transform = &self.merged_left;
        Transform {
            x: width - transform.x - transform.width,
            y: transform.y,
            width: transform.width,
            height: transform.height,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Position {
    Single,
    MergedLeft,
    MergedRight,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct FlagData {
    pub full_name: String,
    pub name: String,
    pub categories: HashSet<String>,
    pub lines: Vec<Vec<Color>>,
    pub symbols: Vec<(SymbolData, Position)>,
}

impl PartialOrd for FlagData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FlagData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.full_name.cmp(&other.full_name)
    }
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct FlagDataJSON {
    pub full_name: String,
    pub name: String,
    pub lines: Vec<String>,
    pub categories: Vec<String>,
    pub symbol: Option<SymbolData>,
}

impl FlagData {
    pub fn from_json(json: &FlagDataJSON) -> FlagData {
        let lines = json
            .lines
            .iter()
            .map(|line| vec![Color(line.clone())])
            .collect();

        let categories = json.categories.iter().cloned().collect();

        let symbols = if let Some(sym) = &json.symbol {
            vec![(sym.clone(), Position::Single)]
        } else {
            vec![]
        };

        FlagData {
            full_name: json.full_name.clone(),
            name: json.name.clone(),
            lines,
            categories,
            symbols,
        }
    }

    pub fn duplicate_middle(flag: &FlagData) -> FlagData {
        let mid = flag.lines.len() / 2;

        let lines = flag
            .lines
            .iter()
            .enumerate()
            .flat_map(|(i, line)| repeat_n(line.clone(), 1 + (i == mid) as usize))
            .collect();

        FlagData {
            lines,
            ..flag.clone()
        }
    }

    pub fn multiply(flag: &FlagData, factor: usize) -> FlagData {
        let lines = flag
            .lines
            .iter()
            .flat_map(|line| std::iter::repeat_n(line.clone(), factor))
            .collect();
        FlagData {
            lines,
            ..flag.clone()
        }
    }

    pub fn mix(flag1: &FlagData, flag2: &FlagData) -> FlagData {
        let mut flag1 = flag1.clone();
        let mut flag2 = flag2.clone();

        if flag1.name == flag2.name {
            return flag1;
        }

        let mut len1 = flag1.lines.len();
        let mut len2 = flag2.lines.len();

        match (len1 % 2, len2 % 2) {
            (1, 0) => {
                flag1 = FlagData::duplicate_middle(&flag1);
                len1 += 1;
            }
            (0, 1) => {
                flag2 = FlagData::duplicate_middle(&flag2);
                len2 += 1;
            }
            _ => {}
        }

        let lcm = lcm(len1, len2);
        let flag1_multiplied = FlagData::multiply(&flag1, lcm / len1);
        let flag2_multiplied = FlagData::multiply(&flag2, lcm / len2);

        let full_name = if flag1.full_name != flag2.full_name {
            format!("{} {}", flag1.full_name, flag2.full_name)
        } else {
            flag1.full_name.clone()
        };

        let name = format!("{} {}", flag1.name, flag2.name);

        let lines = flag1_multiplied
            .lines
            .iter()
            .zip(flag2_multiplied.lines.iter())
            .map(|(line1, line2)| {
                line1
                    .iter()
                    .chain(line2.iter())
                    .cloned()
                    .collect::<Vec<Color>>()
            })
            .collect();

        let categories = flag1.categories.union(&flag2.categories).cloned().collect();

        let symbols = flag1
            .symbols
            .iter()
            .map(|(s, _)| (s.clone(), Position::MergedLeft))
            .chain(
                flag2
                    .symbols
                    .iter()
                    .map(|(s, _)| (s.clone(), Position::MergedRight)),
            )
            .collect();

        FlagData {
            full_name,
            name,
            lines,
            categories,
            symbols,
        }
    }

    pub fn is_compatible(flag1: &FlagData, flag2: &FlagData) -> bool {
        let mut len1 = flag1.lines.len();
        let mut len2 = flag2.lines.len();

        match (len1 % 2, len2 % 2) {
            (1, 0) => {
                len1 += 1;
            }
            (0, 1) => {
                len2 += 1;
            }
            _ => {}
        }

        len2.is_multiple_of(len1) || len1.is_multiple_of(len2)
    }
}

pub fn reduce_strain(hex: &str) -> String {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap() as f32 / 255.0;
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap() as f32 / 255.0;
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap() as f32 / 255.0;

    // Detect near-grayscale
    if (r - g).abs() < 0.02 && (r - b).abs() < 0.02 && (g - b).abs() < 0.02 {
        let mut l = (r + g + b) / 3.0;
        l = l.clamp(0.2, 0.85);
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
    let l = l.clamp(0.3, 0.8);
    let s = s.clamp(0.5, 0.8);

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
