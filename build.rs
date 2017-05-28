// build.rs

extern crate winres;

fn main() {
  if cfg!(target_os = "windows") {
    let mut res = winres::WindowsResource::new();
    res.set_icon("vim48x48.ico");
    res.compile().unwrap();
  }
}
