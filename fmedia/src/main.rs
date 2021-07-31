mod data;

#[path = "lib.rs"]
mod mimpl;

use mimpl::*;

fn main() {
    println!("png: {}", MediaType::from_file_extension("png").unwrap());
    println!("jpeg: {}", MediaType::from_file_extension("jpeg").unwrap());
    println!("jpg: {}", MediaType::from_file_extension("jpg").unwrap());
    println!("svg: {}", MediaType::from_file_extension("svg").unwrap());
    println!("json: {}", MediaType::from_file_extension("json").unwrap());

    assert_eq!("image/png", MediaType::from_file_extension("png").unwrap());
    assert_eq!(
        "image/jpeg",
        MediaType::from_file_extension("jpeg").unwrap()
    );
    assert_eq!(
        "image/svg+xml",
        MediaType::from_file_extension("svg").unwrap()
    );
    assert_eq!(
        "application/json",
        MediaType::from_file_extension("json").unwrap()
    );

    assert_eq!(
        vec!["png"],
        FileExtension::from_media_type("image/png").unwrap()
    );
    assert_eq!(
        vec!["jpe", "jpeg", "jpg"],
        FileExtension::from_media_type("image/jpeg").unwrap()
    );
    assert_eq!(
        vec!["svg", "svgz"],
        FileExtension::from_media_type("image/svg+xml").unwrap()
    );
    assert_eq!(
        vec!["json"],
        FileExtension::from_media_type("application/json").unwrap()
    );

    // println!("png: {}", MediaType::from_file_extension("png"));
}
