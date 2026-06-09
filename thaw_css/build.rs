use std::{env, fs, path::Path};
use walkdir::WalkDir;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_root = Path::new(&manifest_dir).parent().unwrap();

    // Collect CSS from both thaw/src and thaw_components/src
    let css_dirs = [
        workspace_root.join("thaw/src"),
        workspace_root.join("thaw_components/src"),
    ];

    for dir in &css_dirs {
        if !dir.exists() {
            panic!(
                "thaw_css: Could not find directory at {:?}. \
                 This crate must be built from within the thaw workspace.",
                dir
            );
        }
    }

    let mut css_files: Vec<_> = css_dirs
        .iter()
        .flat_map(|dir| WalkDir::new(dir).into_iter())
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "css"))
        .collect();

    css_files.sort_by(|a, b| a.path().cmp(b.path()));

    let mut bundle = String::new();
    for entry in &css_files {
        let content = fs::read_to_string(entry.path()).unwrap();
        bundle.push_str(&content);
        bundle.push('\n');
        println!("cargo:rerun-if-changed={}", entry.path().display());
    }

    let css_path = Path::new(&out_dir).join("thaw.css");
    fs::write(&css_path, &bundle).unwrap();

    let module = format!(
        r#"/// All Thaw component styles concatenated into a single CSS bundle.
///
/// This can be:
/// - Inlined in your HTML: `<style>{{THAW_CSS}}</style>`
/// - Written to a file in your build.rs for serving as a static asset
///
/// # Example
///
/// ```ignore
/// // In your app's build.rs:
/// use std::fs;
/// fn main() {{
///     fs::write("public/thaw.css", thaw_css::THAW_CSS).unwrap();
/// }}
/// ```
pub const THAW_CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/thaw.css"));

/// Length of the CSS bundle in bytes.
pub const THAW_CSS_LEN: usize = {};
"#,
        bundle.len()
    );
    fs::write(Path::new(&out_dir).join("generated_css.rs"), module).unwrap();
}
