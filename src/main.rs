use std::fs::File;
use std::io::prelude::*;

mod ejs;
mod js;
mod classes;
mod controllers;
mod change;

fn main() {
    let ejs_files: Vec<change::Change> = ejs::change();
    let js_files: Vec<change::Change> = js::change();
    let class_files: Vec<change::Change> = classes::change();
    let controller_files: Vec<change::Change> = controllers::change();

    for i in 0..ejs_files.len() {
        let mut contents = match read_file(&ejs_files[i].location) {
            Ok(contents) => contents,
            Err(e) => panic!(e)
        };

        let mut j = 0;
        while j < ejs_files[i].changes.len() {
            contents = contents.replace(ejs_files[i].changes[j], ejs_files[i].changes[j+1]);
            j += 2;
        }

        match write_file(&ejs_files[i].location, contents) {
            Err(e) => panic!(e),
            _ => ()
        };
    }

    for i in 0..js_files.len() {
        let mut contents = match read_file(&js_files[i].location) {
            Ok(contents) => contents,
            Err(e) => panic!(e)
        };

        let mut j = 0;
        while j < js_files[i].changes.len() {
            contents = contents.replace(js_files[i].changes[j], js_files[i].changes[j+1]);
            j += 2;
        }

        match write_file(&js_files[i].location, contents) {
            Err(e) => panic!(e),
            _ => ()
        };
    }

    for i in 0..class_files.len() {
        let mut contents = match read_file(&class_files[i].location) {
            Ok(contents) => contents,
            Err(e) => panic!(e)
        };

        let mut j = 0;
        while j < class_files[i].changes.len() {
            contents = contents.replace(class_files[i].changes[j], class_files[i].changes[j+1]);
            j +=2;
        }

        match write_file(&class_files[i].location, contents) {
            Err(e) => panic!(e),
            _ => ()
        };
    }

    for i in 0..controller_files.len() {
        let mut contents = match read_file(&controller_files[i].location) {
            Ok(contents) => contents,
            Err(e) => panic!(e)
        };

        let mut j = 0;
        while j < controller_files[i].changes.len() {
            contents = contents.replace(controller_files[i].changes[j], controller_files[i].changes[j+1]);
            j +=2;
        }

        match write_file(&controller_files[i].location, contents) {
            Err(e) => panic!(e),
            _ => ()
        };
    }

    //Full pages
    let contents = match read_file("./landing.ejs") {
        Ok(contents) => contents,
        Err(e) => panic!(e)
    };
    match write_file("../../javascript/InventoryManagement/views/otherPages/landing.ejs", contents) {
        Err(e) => panic!(e),
        _ => ()
    };

    let contents = match read_file("./login.ejs") {
        Ok(contents) => contents,
        Err(e) => panic!(e)
    };
    match write_file("../../javascript/InventoryManagement/views/otherPages/login.ejs", contents) {
        Err(e) => panic!(e),
        _ => ()
    };
    
    let contents = match read_file("./register.ejs") {
        Ok(contents) => contents,
        Err(e) => panic!(e)
    };
    match write_file("../../javascript/InventoryManagement/views/otherPages/register.ejs", contents) {
        Err(e) => panic!(e),
        _ => ()
    };

    let contents = match read_file("./help.ejs") {
        Ok(contents) => contents,
        Err(e) => panic!(e)
    };
    match write_file("../../javascript/InventoryManagement/views/informationPages/help.ejs", contents) {
        Err(e) => panic!(e),
        _ => ()
    };
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