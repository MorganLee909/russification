use std::fs::File;
use std::io::prelude::*;

fn main() {
    let folder_location: String = "../../javascript/InventoryManagement".to_string();
    dashboard_ejs(folder_location + "/views/dashboardPage/dashboard.ejs");
    let folder_location: String = "../../javascript/InventoryManagement".to_string();
    shared_css(folder_location + "/views/shared/shared.css");
    let folder_location: String = "../../javascript/InventoryManagement".to_string();
    dashboard_css(folder_location + "/views/dashboardPage/dashboard.css");
    let folder_location: String = "../../javascript/InventoryManagement".to_string();
    sidebars_css(folder_location + "/views/dashboardPage/sidebars.css");
}

fn read_file(file: &String) -> std::io::Result<String> {
    let mut file = File::open(file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn write_file(file: &String, contents: String) -> std::io::Result<()> {
    let mut file = File::create(file)?;
    file.write(contents.as_bytes())?;
    Ok(())
}

fn dashboard_ejs(file: String) {
    let mut contents = match read_file(&file) {
        Ok(contents) => contents,
        Err(e) => panic!(e)
    };

    contents = contents.replace("DASHBOARD", "ГЛАВНАЯ");
    contents = contents.replace("INGREDIENTS", "ИНГРЕДИЕНТЫ");
    contents = contents.replace("RECIPE BOOK", "КНИГА РЕЦЕПТОВ");
    contents = contents.replace("ANALYTICS", "АНАЛИТИКА");
    contents = contents.replace("ORDERS", "ЗАКАЗЫ");
    contents = contents.replace("TRANSACTIONS", "ТРАНЗАКЦИИ");
    contents = contents.replace("LOGOUT", "ВЫХОД");
    contents = contents.replace("INGREDIENT INVENTORY", "ИНГРЕДИЕНТЫ");
    contents = contents.replace("Total Revenue (month)", "Общий доход (месяц)");
    contents = contents.replace("Inventory Check", "Проверить Ингредиеты");
    contents = contents.replace(">Update", ">ОБНОВИТЬ");
    contents = contents.replace("UPDATE", "ОБНОВИТЬ");
    contents = contents.replace("NEW", "НОВЫЙ");
    contents = contents.replace("RECIPES", "РЕЦЕПТЫ");
    contents = contents.replace("Date Range:", "Диапазон Дат:");
    contents = contents.replace("SUBMIT", "РАЗМЕСТИТЬ");

    match write_file(&file, contents) {
        Err(e) => panic!(e),
        _ => ()
    };
}

fn shared_css(file: String) {
    let mut contents = match read_file(&file) {
        Ok(contents) => contents,
        Err(e) => panic!(e)
    };

    contents = contents.replace("green", "rgb(0, 27, 45)");

    match write_file(&file, contents) {
        Err(e) => panic!(e),
        _ => ()
    };
}

fn dashboard_css(file: String) {
    let mut contents = match read_file(&file) {
        Ok(contents) => contents,
        Err(e) => panic!(e)
    };

    contents = contents.replace("green", "rgb(0, 27, 45)");

    match write_file(&file, contents) {
        Err(e) => panic!(e),
        _ => ()
    };
}

fn sidebars_css(file: String) {
    let mut contents = match read_file(&file) {
        Ok(contents) => contents,
        Err(e) => panic!(e)
    };

    contents = contents.replace("green", "rgb(0, 27, 45)");

    match write_file(&file, contents) {
        Err(e) => panic!(e),
        _ => ()
    };
}
