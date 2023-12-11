use std::fs;
use std::io;
use std::path::PathBuf;

// markdown::to_html(&content)
// string.replace("{{ content }}", markdown::to_html(&content))
pub fn html_generator(path: &PathBuf, html: &str) -> io::Result<()> {
    println!("path: {:?}", path);
    let content: String = fs::read_to_string(&path).unwrap();

    let mut html = html.replace("{{ base-path }}", &base_path(&path));
    html = html.replace("{{ content }}", &markdown::to_html(&content));

    fs::remove_file(&path)?;
    let mut path = path.clone();

    if path.file_name().unwrap() == "README.md" {
        path.set_file_name("index.html");
        fs::write(path, html)?;
    } else {
        path.set_extension("html");
        fs::write(path, html)?;
    }

    Ok(())
}

fn base_path(path: &PathBuf) -> String {
    let mut base_path = String::new();
    let mut path = path.clone();
    path.pop();
    for _ in 0..path.iter().count() {
        base_path.push_str("../");
    }
    base_path
}
