use std::env;
use std::path::PathBuf;
use std::io::{stdout, BufWriter, Write};

fn base_path() -> PathBuf {
    match env::var("PACIFICA_PATH") {
        Ok(path) => PathBuf::from(path),
        Err(_) => PathBuf::from(env::var("HOME").expect("failed to read $HOME. You have to set PACIFICA_PATH.") + "/src"),
    }
}

fn walk_dir(path: PathBuf, depth: i8) {
    // buffering
    let out = stdout();
    let mut out = BufWriter::new(out.lock());

    let paths = path.read_dir().expect("failed to read dir");

    for p in paths {
        let pb = p.unwrap().path();
        let path_string = pb.display().to_string();

        if !pb.is_dir() {
            continue;
        }

        if depth - 1 >= 0 {
            walk_dir(pb, depth - 1)
        } else if !path_string.split("/").last().expect("failed to path_string.split(\"/\").last()").starts_with(".") {
            // prevent something like a .git coming in.
            write!(out, "{}\n", path_string).expect("failed to write stdout");
        }
    }
}

fn main() {
    let base_path = base_path();

    walk_dir(base_path, 2);
}
