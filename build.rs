use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Get the path to the project root
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let source_marker_path = Path::new(&manifest_dir).join(".html-source");

    // Check if .html-source file exists
    if source_marker_path.exists() {
        // Read the path from .html-source
        let source_path = fs::read_to_string(&source_marker_path)
            .expect("Failed to read .html-source file")
            .trim()
            .to_string();

        let source_path_abs = Path::new(&manifest_dir).join(&source_path);

        // Check if the source file actually exists
        if !source_path_abs.exists() {
            panic!(
                "Source HTML file '{}' does not exist!",
                source_path_abs.display()
            );
        }

        let dest_path = Path::new(&manifest_dir).join("src").join("index.html");

        let mut html = fs::read_to_string(&source_path_abs).expect("Failed to read HTML file");
        let pos = html.rfind("</body>").unwrap();
        html.insert_str(
            pos,
            "  <div id=\"port-marker-he9RYeXH5Psd7vcKOzWs\" style=\"display: none;\"></div>\n  ",
        );

        // Copy the file
        fs::write(&dest_path, &html).expect("Failed to copy HTML file to src/index.html");

        println!("cargo:rerun-if-changed=.html-source");
        println!("cargo:rerun-if-changed={}", source_path_abs.display());
    } else {
        println!("cargo:rerun-if-changed=.html-source");
    }
}
