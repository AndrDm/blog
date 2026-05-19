use std::fs;
use std::path::PathBuf;

fn main() {
    let base = PathBuf::from(r"C:\Program Files\National Instruments");

    // Collect all LabVIEW versions that contain cintools/labview.lib
    let mut candidates: Vec<(u32, PathBuf)> = Vec::new();

    if let Ok(entries) = fs::read_dir(&base) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            let name = match path.file_name().and_then(|n| n.to_str()) {
                Some(n) => n,
                None => continue,
            };

            // Folder must start with "LabVIEW "
            if let Some(rest) = name.strip_prefix("LabVIEW ") {
                // Parse version (e.g., 2026, 2025, 2024)
                if let Ok(version) = rest.parse::<u32>() {
                    let cintools = path.join("cintools").join("labview.lib");
                    if cintools.exists() {
                        candidates.push((version, cintools));
                    }
                }
            }
        }
    }

    // Sort by version descending (newest first)
    candidates.sort_by(|a, b| b.0.cmp(&a.0));

    // Pick the newest version
    let Some((version, lib_path)) = candidates.into_iter().next() else {
        panic!("Could not find any LabVIEW installation with cintools/labview.lib");
    };

    let lib_dir = lib_path.parent().unwrap();

    println!(
        "cargo:warning=Using LabVIEW version {} at {:?}",
        version, lib_dir
    );
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=labview");
    println!("cargo:rustc-link-lib=dylib=user32");
}
