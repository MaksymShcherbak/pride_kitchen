use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("assets.rs");

    let base_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let assets_dir = PathBuf::from(&base_dir).join("./assets");

    let img_assets_dir = assets_dir.join("./symbols");
    let flags_path = assets_dir.join("./flags.json");

    let mut code = String::new();
    code.push_str(&format!(
        "static FLAGS_JSON: &str = include_str!(\"{}\");",
        flags_path.to_string_lossy().replace("\\", "/")
    ));

    code.push_str(
        r#"
lazy_static::lazy_static! {
    pub static ref FLAGS: Vec<crate::flag::FlagData> = serde_json::from_str(FLAGS_JSON).expect("Invalid flags.json");

    pub static ref IMG_ASSETS: HashMap<&'static str, Asset> = {
        let mut map = HashMap::new();
"#,
    );

    if let Ok(entries) = fs::read_dir(&img_assets_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let fname = path.file_name().unwrap().to_str().unwrap();
                let rel_path = format!("/assets/symbols/{fname}");
                code.push_str(&format!(
                    "        map.insert(\"{fname}\", asset!(\"{rel_path}\"));\n"
                ));
            }
        }
    }

    code.push_str("        map\n    };\n}\n");

    fs::write(&dest_path, code).unwrap();

    println!("cargo:rerun-if-changed=./assets/symbols/");
    println!("cargo:rerun-if-changed=build.rs");
}
