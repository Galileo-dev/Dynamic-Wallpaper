const x65_windows_libs: &str = "D:/Programming/c++/vcpkg/installed/x64-windows/debug/lib";

fn main() {
  println!("cargo:rustc-link-search=native={}", x65_windows_libs);
  println!("cargo:rustc-link-lib=heif");
  println!("cargo:rustc-link-lib=libde265");
  println!("cargo:rustc-link-lib=x265-static");
  tauri_build::build()
}
