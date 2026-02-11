use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
  // `tauri::generate_context!` validates `distDir` at compile time.
  // Ensure the path exists so `cargo check` works before web assets are built.
  let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
  let dist_dir = manifest_dir.join("../cinny/dist");
  let _ = fs::create_dir_all(dist_dir);

  tauri_build::build()
}
