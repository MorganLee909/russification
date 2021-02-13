use std::fs::File;
use std::io::prelude::*;

mod ejs;
mod js;
mod classes;
mod controllers;
mod css;
mod change;

fn main() {
    let change_files: Vec<Vec<change::Change>> = vec![
        ejs::change(),
        js::change(),
        classes::change(),
        controllers::change(),
        css::change()
    ];

    for i in 0..change_files.len() {
        for j in 0..change_files[i].len() {
            let mut contents = match read_file(&change_files[i][j].location) {
                Ok(contents) => contents,
                Err(e) => panic!(e)
            };

            let mut k = 0;
            while k < change_files[i][j].changes.len() {
                contents = contents.replace(change_files[i][j].changes[k], change_files[i][j].changes[k+1]);
                k += 2;
            }

            match write_file(&change_files[i][j].location, contents) {
                Err(e) => panic!(e),
                _ => ()
            };
        }
    }

    //Full pages to overwrite
    std::fs::copy("./landing.ejs", "../InventoryManagement/views/otherPages/landing.ejs").unwrap();
    std::fs::copy("./login.ejs", "../InventoryManagement/views/otherPages/login.ejs").unwrap();
    std::fs::copy("./register.ejs", "../InventoryManagement/views/otherPages/register.ejs").unwrap();
    std::fs::copy("./help.ejs", "../InventoryManagement/views/informationPages/help.ejs").unwrap();
    std::fs::copy("./email.ejs", "../InventoryManagement/views/passwordResetPages/email.ejs").unwrap();
    std::fs::copy("./password.ejs", "../InventoryManagement/views/passwordResetPages/password.ejs").unwrap();
    std::fs::copy("./footer.ejs", "../InventoryManagement/views/shared/footer.ejs").unwrap();
    std::fs::copy("./verify.ejs", "../InventoryManagement/views/verifyPage/verify.ejs").unwrap();
    std::fs::copy("./passwordReset.js", "../InventoryManagement/emails/passwordReset.js").unwrap();
    std::fs::copy("./verifyEmail.js", "../InventoryManagement/emails/verifyEmail.js").unwrap();
    std::fs::copy("./loader.ejs", "../InventoryManagement/views/shared/loader.ejs").unwrap();

    // Images
    std::fs::copy("./images/oneLineLogo.png", "../InventoryManagement/views/shared/images/oneLineLogo.png").unwrap();
    std::fs::copy("./images/twoLineLogo.png", "../InventoryManagement/views/shared/images/twoLineLogo.png").unwrap();
    std::fs::copy("./images/favicon.png", "../InventoryManagement/views/shared/images/favicon.png").unwrap();
    std::fs::copy("./images/vectorLogo.png", "../InventoryManagement/views/shared/images/vectorLogo.png").unwrap();
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