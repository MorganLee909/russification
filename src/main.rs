use std::fs::File;
use std::io::prelude::*;
// use walkdir::{DirEntry, WalkDir};

mod ejs;
mod js;
mod classes;
mod controllers;
mod css;
mod change;

fn main() {
    let ejs_files: Vec<change::Change> = ejs::change();
    let js_files: Vec<change::Change> = js::change();
    let class_files: Vec<change::Change> = classes::change();
    let controller_files: Vec<change::Change> = controllers::change();
    let css_files: Vec<change::Change> = css::change();

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
        };

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
        };

        match write_file(&controller_files[i].location, contents) {
            Err(e) => panic!(e),
            _ => ()
        };
    }

    for i in 0..css_files.len() {
        let mut contents = match read_file(&css_files[i].location) {
            Ok(contents) => contents,
            Err(e) => panic!(e)
        };

        let mut j = 0;
        while j < css_files[i].changes.len() {
            contents = contents.replace(css_files[i].changes[j], css_files[i].changes[j+1]);
            j += 2;
        };

        match write_file(&css_files[i].location, contents) {
            Err(e) => panic!(e),
            _ => ()
        };
    }

    //Full pages to overwrite
    let contents = match read_file("./landing.ejs") {
        Ok(contents) => contents,
        Err(e) => panic!(e)
    };
    match write_file("../InventoryManagement/views/otherPages/landing.ejs", contents) {
        Err(e) => panic!(e),
        _ => ()
    };

    let contents = match read_file("./login.ejs") {
        Ok(contents) => contents,
        Err(e) => panic!(e)
    };
    match write_file("../InventoryManagement/views/otherPages/login.ejs", contents) {
        Err(e) => panic!(e),
        _ => ()
    };
    
    let contents = match read_file("./register.ejs") {
        Ok(contents) => contents,
        Err(e) => panic!(e)
    };
    match write_file("../InventoryManagement/views/otherPages/register.ejs", contents) {
        Err(e) => panic!(e),
        _ => ()
    };

    let contents = match read_file("./help.ejs") {
        Ok(contents) => contents,
        Err(e) => panic!(e)
    };
    match write_file("../InventoryManagement/views/informationPages/help.ejs", contents) {
        Err(e) => panic!(e),
        _ => ()
    };

    let contents = match read_file("./email.ejs") {
        Ok(contents) => contents,
        Err(e) => panic!(e)
    };
    match write_file("../InventoryManagement/views/passwordResetPages/email.ejs", contents) {
        Err(e) => panic!(e),
        _ => ()
    };

    let contents = match read_file("./password.ejs") {
        Ok(contents) => contents,
        Err(e) => panic!(e)
    };
    match write_file("../InventoryManagement/views/passwordResetPages/password.ejs", contents) {
        Err(e) => panic!(e),
        _ => ()
    };

    let contents = match read_file("./footer.ejs") {
        Ok(contents) => contents,
        Err(e) => panic!(e)
    };
    match write_file("../InventoryManagement/views/shared/footer.ejs", contents) {
        Err(e) => panic!(e),
        _ => ()
    };

    let contents = match read_file("./verify.ejs") {
        Ok(contents) => contents,
        Err(e) => panic!(e)
    };
    match write_file("../InventoryManagement/views/verifyPage/verify.ejs", contents) {
        Err(e) => panic!(e),
        _ => ()
    };
    std::fs::copy("./passwordReset.js", "../InventoryManagement/emails/passwordReset.js");
    std::fs::copy("./verifyEmail.js", "../InventoryManagement/emails/verifyEmail.js");

    // Images
    std::fs::copy("./images/oneLineLogo.png", "../InventoryManagement/views/shared/images/oneLineLogo.png").unwrap();
    std::fs::copy("./images/twoLineLogo.png", "../InventoryManagement/views/shared/images/twoLineLogo.png").unwrap();
    std::fs::copy("./images/favicon.png", "../InventoryManagement/views/shared/images/favicon.png").unwrap();
    std::fs::copy("./images/vectorLogo.png", "../InventoryManagement/views/shared/images/vectorLogo.png").unwrap();

    //Replace on all pages
    // fn is_hidden(entry: &DirEntry) -> bool {
    //     entry.file_name()
    //         .to_str()
    //         .map(|s| s.starts_with(".")) 
    //         .unwrap_or(false)
    // }

    // let walker = WalkDir::new("../InventoryManagement/").into_iter();
    // for entry in walker.filter_entry(|e| !is_hidden(e)) {
    //     let entry = entry.unwrap();
    //     let path_string: String = entry.path().display().to_string();
        
    //     if !path_string.contains("node_modules") && (path_string.contains(".js") || path_string.contains(".ejs")) {
    //         let mut contents = match read_file(&path_string[..]) {
    //             Ok(contents) => contents,
    //             Err(e) => panic!(e)
    //         };

    //         contents = contents.replace("$$", "₽$");
    //         contents = contents.replace("parseFloat(", "parseInt(");
    //         contents = contents.replace(".toFixed(2)", ".toFixed()");

    //         match write_file(&path_string, contents) {
    //             Err(e) => panic!(e),
    //             _ => ()
    //         };
    //     }
    // }
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