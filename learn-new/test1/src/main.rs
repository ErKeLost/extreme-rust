use serde_json::{json, Value};

fn resolve_import_path(exports: &Value, import_path: &str) -> Option<String> {
    let mut path_segments: Vec<&str> = import_path.split('/').collect();
    let mut current_exports = exports;

    while let Some(segment) = path_segments.pop() {
        if let Some(exports_obj) = current_exports.as_object() {
            if let Some(exports_entry) = exports_obj.get(segment) {
                if let Some(export_path) = exports_entry.get("import").and_then(Value::as_str) {
                    return Some(export_path.to_string());
                } else if let Some(inner_exports) = exports_entry.get("worker") {
                    current_exports = inner_exports;
                } else {
                    // If no "import" or "worker" found, break the loop.
                    break;
                }
            } else {
                // If the segment is not found in exports, break the loop.
                break;
            }
        } else {
            // If current exports is not an object, break the loop.
            break;
        }
    }

    None
}

fn main() {
    let exports: Value = json!({
        "./dist": {
            "worker": {
                "import": "./dist/index.js"
            },
            "lib": {
                "import": "./dist/lib.js"
            }
        },
        "./src": {
            "app": {
                "import": "./src/app.js"
            }
        }
    });

    let import_path = "dist/worker";

    if let Some(exported_path) = resolve_import_path(&exports, import_path) {
        println!("Resolved import path: {}", exported_path);
    } else {
        println!("Import path not found or invalid.");
    }
}
