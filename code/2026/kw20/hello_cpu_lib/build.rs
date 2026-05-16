use std::fs;
use std::path::PathBuf;

fn main() {
    // Base directory where LabVIEW installs cintools
    let base = PathBuf::from(r"C:\Program Files\National Instruments");

    // Find all directories starting with "LabVIEW "
    let mut candidates: Vec<(u32, PathBuf)> = Vec::new();

    if let Ok(entries) = fs::read_dir(&base) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }

            // Folder name must start with "LabVIEW "
            let name = match path.file_name().and_then(|n| n.to_str()) {
                Some(n) => n,
                None => continue,
            };

            if let Some(rest) = name.strip_prefix("LabVIEW ") {
                // Try to parse the version number (2026, 2025, 2024...)
                if let Ok(version) = rest.parse::<u32>() {
                    let cintools = path.join("cintools").join("labview.lib");
                    if cintools.exists() {
                        candidates.push((version, cintools));
                    }
                }
            }
        }
    }

    // Pick the highest LabVIEW version found
    let Some((version, lib_path)) = candidates.into_iter().max_by_key(|c| c.0) else {
        panic!("Could not find any LabVIEW installation with cintools/labview.lib");
    };

    let lib_dir = lib_path.parent().unwrap();

    println!(
        "cargo:warning=Using LabVIEW version {} at {:?}",
        version, lib_dir
    );
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=labview");
    // Always link user32.lib (Windows system library)
    println!("cargo:rustc-link-lib=dylib=user32");
}
