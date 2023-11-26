extern crate winres;

fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("dbf2csv.ico");
    res.compile().unwrap();
}
