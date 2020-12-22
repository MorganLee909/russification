use std::fs::File;
use std::io::prelude::*;

fn main() {
    let thing = match read_file() {
        Ok(contents) => edit_string(&contents),
        Err(e) => panic!(e)
    };

    match create_file(&thing){
        Err(e) => panic!(e),
        _ => ()
    }
}

fn read_file() -> std::io::Result<String> {
    let mut file = File::open("../../javascript/InventoryManagement/views/dashboardPage/dashboard.ejs")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn create_file(contents: &String) -> std::io::Result<()> {
    let mut file = File::create("something.html")?;
    file.write(contents.as_bytes())?;
    Ok(())
}

fn edit_string(contents: &String) -> String{
    contents.replace("INGREDIENTS", "ИНГЕДИЕТЫ")
}
