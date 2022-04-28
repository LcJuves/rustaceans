use std::io::{Result, Write};
use std::path::Path;

use clap::Parser;
use qrcode::render::svg;
use qrcode::{EcLevel, QrCode, Version};

/// For generating QR code vector graphics
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// What the QR code contains
    #[clap(parse(from_str))]
    text: String,

    /// Vector generated path
    #[clap(short, long, default_value_t = String::from(""))]
    out_path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let code = QrCode::with_version(args.text.as_bytes(), Version::Normal(10), EcLevel::L).unwrap();
    let image = code
        .render()
        .min_dimensions(400, 400)
        .dark_color(svg::Color("#800000"))
        .light_color(svg::Color("#ffff80"))
        .build();
    let svg_out_path = Path::new(args.out_path.as_str());
    if !args.out_path.is_empty() {
        let svg_out_path_parent = svg_out_path
            .parent()
            .expect("The folder where the build path is located does not exist!");
        if !svg_out_path_parent.exists() {
            std::fs::create_dir_all(svg_out_path_parent)?;
        }
        let mut svg_file = std::fs::OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(true)
            .open(svg_out_path)?;
        svg_file.write_all(image.as_bytes())?;
        svg_file.flush()?;
    } else {
        println!("{}", image);
    }
    Ok(())
}
