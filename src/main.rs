use std::path::Path;
use std::env;
use std::fmt;

#[derive(PartialEq, Eq, Debug)]
enum FileType {
    Gif,
    Html,
    JavaScript,
    Markdown,
    Png,
    Rust,
    Toml,
    Unknown,
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let symbol = match *self {
            FileType::Gif | FileType::Png => "",
            FileType::Html => "",
            FileType::JavaScript => "",
            FileType::Markdown => "",
            FileType::Rust => "",
            FileType::Toml => "",
            FileType::Unknown => ""
        };

        write!(f, "{}", symbol)
    }
}

fn classify(path: &Path) -> FileType {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("js")                 => FileType::JavaScript,
        Some("html") | Some("htm") => FileType::Html,
        Some("rs")                 => FileType::Rust,
        Some("toml")               => FileType::Toml,
        Some("md")                 => FileType::Markdown,
        Some("png")                => FileType::Png,
        Some("gif")                => FileType::Gif,
        Some(_)                    => FileType::Unknown,
        None                       => FileType::Unknown,
    }
}

fn main() {
    for path in env::args().skip(1) {
        let file_type = classify(&Path::new(&path));
        println!("{}  {}", file_type, path);
    }
}

#[test]
fn test_file_type_rust() {
    assert_eq!(classify(Path::new("src/main.rs")), FileType::Rust);
}

#[test]
fn test_file_type_toml() {
    assert_eq!(classify(Path::new("Cargo.toml")), FileType::Toml);
}

#[test]
fn test_file_type_html() {
    assert_eq!(classify(Path::new("index.html")), FileType::Html);
    assert_eq!(classify(Path::new("index.htm")), FileType::Html);
}

#[test]
fn test_file_type_unknown() {
    assert_eq!(classify(Path::new("README.xxx")), FileType::Unknown);
}

#[test]
fn test_file_type_missing() {
    assert_eq!(classify(Path::new("LICENSE")), FileType::Unknown);
}
