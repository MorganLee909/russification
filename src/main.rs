use std::fs::File;
use std::io::prelude::*;

mod ejs;
use ejs::Change;

fn main() {
    let files: Vec<Change> = ejs::change();

    for i in 0..files.len() {
        let mut contents = match read_file(&files[i].location) {
            Ok(contents) => contents,
            Err(e) => panic!(e)
        };

        let mut j = 0;
        while j < files[i].changes.len() {
            contents = contents.replace(files[i].changes[j], files[i].changes[j+1]);
            j += 2;
        }

        match write_file(&files[i].location, contents) {
            Err(e) => panic!(e),
            _ => ()
        };
    }
}

fn read_file(file: &str) -> std::io::Result<String> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn write_file(file: &str, contents: String) -> std::io::Result<()> {
    let mut file = File::create(file)?;
    file.write(contents.as_bytes())?;
    Ok(())
}