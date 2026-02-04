use dioxus::prelude::*;
use std::sync::LazyLock;

use crate::flag_lib::{FlagData, FlagDataJSON};

// IMG_ASSETS: LazyLock<HashMap<&'static str, Asset>>
include!(concat!(env!("OUT_DIR"), "/assets.rs"));

static FLAGS_JSON: LazyLock<Vec<FlagDataJSON>> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../assets/flags.json")).expect("Invalid flags.json")
});
static FLAGS: LazyLock<Vec<FlagData>> =
    LazyLock::new(|| FLAGS_JSON.iter().map(FlagData::from_json).collect());

#[macro_export]
macro_rules! get_asset {
    ($name:expr) => {
        *$crate::IMG_ASSETS.get($name).unwrap()
    };
}

mod app;
mod flag;
mod flag_lib;
fn main() {
    dioxus::launch(app::App);
}
