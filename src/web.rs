use dioxus::prelude::document;

pub fn download_result_png() {
    document::eval(include_str!("js/download_png.js"));
}

pub fn download_result_svg() {
    document::eval(include_str!("js/download_svg.js"));
}

pub fn scroll_to_top() {
    document::eval(include_str!("js/scroll_top.js"));
}