use crate::compile::to_html;
use crate::error::Error;
use crate::file_walker::{dirs_walker, file_name, files_walker, path_to_str};
use std::collections::HashMap;
use std::fs::{create_dir_all, read_to_string, write};
use std::path::PathBuf;

fn compute_out(dir: &PathBuf, src: &PathBuf) -> Result<PathBuf, Error> {
    let mut res = match dir.strip_prefix(src) {
        Ok(res) => res.into(),
        Err(_) => unreachable!(),
    };
    if file_name(&res).to_lowercase() == "readme.md" {
        res.set_file_name("index.html")
    } else if res.extension().is_some_and(|ext| ext == "md") {
        res.set_extension("html");
    }
    Ok(res)
}

fn compute_base_path(dir: &PathBuf, src: &PathBuf) -> Result<String, Error> {
    match dir.strip_prefix(src) {
        Ok(res) => Ok(res.iter().fold("./".into(), |acc, _| acc + "../")),
        Err(_) => unreachable!(),
    }
}

fn make_link(dir: &(PathBuf, PathBuf)) -> Result<String, Error> {
    let content = read_to_string(dir.0.clone())?;
    let title = match content.lines().find(|s| s.starts_with("title: ")) {
        Some(title) => title[7..].trim().to_string(),
        None => file_name(&dir.1),
    };
    Ok(format!(
        "<li><a href=\"{}\">{}</a></li>\n",
        path_to_str(&dir.1),
        title
    ))
}

/// Generates the list of links of a collection formatted as an HTML list
fn make_links(links: &Vec<(PathBuf, PathBuf)>) -> String {
    links
        .iter()
        .filter(|(_, to)| !(file_name(to) == "index.html"))
        .filter_map(|dir| make_link(dir).ok())
        .fold(String::new(), |acc, link| acc + &link)
}

pub fn make_page(dirs: (&PathBuf, PathBuf), vars: &HashMap<&str, String>) -> Result<(), Error> {
    let mut content = read_to_string(&dirs.0)?;
    content = match to_html(&content, &vars) {
        Ok(v) => v,
        Err(Error::MissingLayoutGeneric) => return Err(Error::MissingLayout(path_to_str(dirs.0))),
        Err(Error::SettingsNotFoundGeneric) => {
            return Err(Error::SettingsNotFound(path_to_str(dirs.0)))
        }
        Err(Error::ContentNotFoundGeneric) => {
            return Err(Error::ContentNotFound(path_to_str(dirs.0)))
        }
        Err(e) => return Err(e),
    };
    Ok(write(&dirs.1, content)?)
}

pub fn make_collection(dir: &PathBuf, src: &PathBuf, out: &PathBuf) -> Result<(), Error> {
    let output_dir = compute_out(dir, src)?;
    if !output_dir.exists() {
        create_dir_all(&output_dir)?;
    }

    let mut dirs = files_walker(dir)?
        .iter_mut()
        .filter(|file| file.extension().is_some_and(|ext| ext == "md"))
        .filter_map(|file| compute_out(file, src).ok().map(|o| (file.clone(), o)))
        .collect::<Vec<_>>();

    dirs.sort_by(|(a, _), (b, _)| {
        file_name(b)
            .to_lowercase()
            .cmp(&file_name(a).to_lowercase())
    });

    dirs.push(("".into(), output_dir));

    let mut vars = HashMap::new();
    vars.insert("base-path", compute_base_path(dir, src)?);
    vars.insert("folder", file_name(dir));
    vars.insert("links", make_links(&dirs));

    for i in 1..dirs.len() - 1 {
        vars.insert("next-page", path_to_str(&dirs[i - 1].1));
        vars.insert("previous-page", path_to_str(&dirs[i + 1].1));
        make_page((&dirs[i].0, out.join(&dirs[i].1)), &vars)?;
    }

    if dirs.len() > 2 {
        vars.insert("next-page", path_to_str(&dirs[dirs.len() - 1].1));
        vars.insert("previous-page", path_to_str(&dirs[1].1));
    }
    if dirs.len() > 1 {
        make_page((&dirs[0].0, out.join(&dirs[0].1)), &vars)?
    }
    Ok(())
}

pub fn make_site(src: &str, out: &str) -> Result<(), Error> {
    if !PathBuf::from(out).exists() {
        create_dir_all(out)?;
    }

    let src = &PathBuf::from(src).canonicalize()?;
    let out = &PathBuf::from(out).canonicalize()?;

    let mut dirs = dirs_walker(src)?;
    dirs.pop();

    for dir in dirs {
        make_collection(&dir, src, out)?;
    }

    Ok(())
}
