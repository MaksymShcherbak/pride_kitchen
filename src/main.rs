use dioxus::{
    logger::tracing::{self, Level},
    prelude::*,
};
use std::collections::HashMap;

// We use the build.rs to generate some static build files
// IMG_ASSETS: HashMap<&'static str, Asset>
// FLAGS: Vec<FlagData>
include!(concat!(env!("OUT_DIR"), "/assets.rs"));

const FAVICON: Asset = asset!("/assets/favicon.ico");
const CSS: Asset = asset!("/assets/main.css");

mod app;
mod flag;
fn main() {
    dioxus::logger::init(Level::DEBUG).expect("failed to init logger");
    dioxus::launch(app::App);
}
