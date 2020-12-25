use std::fs::File;
use std::io::prelude::*;

fn main() {
    let folder_location: &str = "../../javascript/InventoryManagement";

    let mut file_location: &str = format!("{}{}", folder_location, "/views/dashboardPage/dashboard.ejs");
    dashboard_ejs(file_location);

    file_location = format!("{}{}", folder_location, "/views/dashboardPage/sidebars/editIngredient.ejs");
    edit_ingredient_ejs(file_location);

    file_location = format!("{}{}", folder_location, "/views/dashboardPage/sidebars/editRecipe.ejs");
    edit_recipe_ejs(file_location);
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

fn dashboard_ejs(file: &str) {
    let mut contents = match read_file(file) {
        Ok(contents) => contents,
        Err(e) => panic!(e)
    };

    // TODO: get translations for the the cards on the analytics page
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
    contents = contents.replace("FILTER", "ФИЛЬТРОВАТЬ");
    
    match write_file(file, contents) {
        Err(e) => panic!(e),
        _ => ()
    };
}

fn edit_ingredient_ejs(file: &str) {
    let mut contents = match read_file(file) {
        Ok(contents) => contents,
        Err(e) => panic!(e)
    };

    contents = contents.replace("NAME", "НАЗВАНИЕ");
    contents = contents.replace("CATEGORY", "КАТЕГОРИЯ");
    contents = contents.replace("MEASUREMENT UNIT", "ЕДИНИЦА ИЗМЕРЕНИЯ");
    contents = contents.replace("SUBMIT", "РАЗМЕСТИТЬ");

    match write_file(file, contents) {
        Err(e) => panic!(e),
        _ => ()
    };
}

fn edit_recipe_ejs(file: &str) {
    let mut contents = match read_file(file) {
        Ok(contents) => contents,
        Err(e) => panic!(e)
    };

    contents = contents.replace("NAME", "НАЗВАНИЕ");
    contents = contents.replace("INGREDIENTS", "ИНГРЕДИЕНТЫ");
    contents = contents.replace("PRICE", "ЦЕНА");
    contents = contents.replace("UPDATE", "ОБНОВИТЬ");
    contents = contents.replace("CANCEL", "ОТМЕНИТЬ");

    match write_file(file, contents) {
        Err(e) => panic!(e),
        _ => ()
    };
}