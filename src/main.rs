use clap::Parser;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Target{
    file: String,
    #[arg(short, long,default_value = "0.5")]
    percentage: f32,

}
fn delete_percent(file : &str , percentage: f32) {
    let path = Path::new(&file);
    let v = fs::read(path).unwrap();
    fs::write(path, v[0..(v.len() as f32 * (1.0-percentage)) as usize].to_vec()).unwrap();
}
fn main() {
    let target = Target::parse();
    assert!(target.percentage >= 0.0 && target.percentage <= 1.0);
    if fs::metadata(&target.file).is_ok() {
        let meta = fs::metadata( &target.file).expect("msg");
        println!("File size is {} bytes", meta.len());
        delete_percent(&target.file, target.percentage);

        println!("File size is {} bytes", fs::metadata( &target.file).expect("msg").len());
    } else {
        println!("File does not exist");
    }
}
