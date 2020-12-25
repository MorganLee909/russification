use std::fs::File;
use std::io::prelude::*;

fn main() {
    let folder_location: &str = "../../javascript/InventoryManagement";

    let mut file_location: String = format!("{}{}", folder_location, "/views/dashboardPage/dashboard.ejs");
    dashboard_ejs(&file_location);

    file_location = format!("{}{}", folder_location, "/views/dashboardPage/sidebars/editIngredient.ejs");
    edit_ingredient_ejs(&file_location);

    file_location = format!("{}{}", folder_location, "/views/dashboardPage/sidebars/editRecipe.ejs");
    edit_recipe_ejs(&file_location);

    file_location = format!("{}{}", folder_location, "/views/dashboardPage/sidebars/ingredientDetails.ejs");
    ingredient_details_ejs(&file_location);

    file_location = format!("{}{}", folder_location, "/views/dashboardPage/sidebars/newIngredient.ejs");
    new_ingredient_ejs(&file_location);

    file_location = format!("{}{}", folder_location, "/views/dashboardPage/sidebars/newOrder.ejs");
    new_order_ejs(&file_location);

    file_location = format!("{}{}", folder_location, "/views/dashboardPage/sidebars/newRecipe.ejs");
    new_recipe_ejs(&file_location);

    file_location = format!("{}{}", folder_location, "/views/dashboardPage/sidebars/newTransaction.ejs");
    new_transaction_ejs(&file_location);
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
    let changes = [
        "DASHBOARD", "ГЛАВНАЯ",
        "INGREDIENTS", "ИНГРЕДИЕНТЫ",
        "RECIPE BOOK", "КНИГА РЕЦЕПТОВ",
        "ANALYTICS", "АНАЛИТИКА",
        "ORDERS", "ЗАКАЗЫ",
        "TRANSACTIONS", "ТРАНЗАКЦИИ",
        "LOGOUT", "ВЫХОД",
        "INGREDIENT INVENTORY", "ИНГРЕДИЕНТЫ",
        "Total Revenue (month)", "Общий доход (месяц)",
        "Inventory Check", "Проверить Ингредиеты",
        ">Update", ">ОБНОВИТЬ",
        "UPDATE", "ОБНОВИТЬ",
        "NEW", "НОВЫЙ",
        "RECIPES", "РЕЦЕПТЫ",
        "Date Range:", "Диапазон Дат:",
        "SUBMIT", "РАЗМЕСТИТЬ",
        "FILTER", "ФИЛЬТРОВАТЬ"
    ];

    let mut i = 0;
    while i < changes.len() {
        contents = contents.replace(changes[i], changes[i+1]);
        i += 2;
    }
    
    match write_file(file, contents) {
        Err(e) => panic!(e),
        _ => ()
    };
}

fn edit_ingredient_ejs(file: &String) {
    let mut contents = match read_file(file) {
        Ok(contents) => contents,
        Err(e) => panic!(e)
    };

    let changes = [
        "NAME", "НАЗВАНИЕ",
        "CATEGORY", "КАТЕГОРИЯ",
        "MEASUREMENT UNIT", "ЕДИНИЦА ИЗМЕРЕНИЯ",
        "SUBMIT", "РАЗМЕСТИТЬ"
    ];

    let mut i = 0;
    while i < changes.len() {
        contents = contents.replace(changes[i], changes[i+1]);
        i += 2;
    }

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

    let changes = [
        "NAME", "НАЗВАНИЕ",
        "INGREDIENTS", "ИНГРЕДИЕНТЫ",
        "PRICE", "ЦЕНА",
        "UPDATE", "ОБНОВИТЬ",
        "CANCEL", "ОТМЕНИТЬ"
    ];

    let mut i = 0;
    while i < changes.len() {
        contents = contents.replace(changes[i], changes[i+1]);
        i += 2;
    }

    match write_file(file, contents) {
        Err(e) => panic!(e),
        _ => ()
    };
}

fn ingredient_details_ejs(file: &str) {
    let mut contents = match read_file(file) {
        Ok(contents) => contents,
        Err(e) => panic!(e)
    };

    // TODO: translate "Average daily use"
    let changes = [
        "CURRENT STOCK", "ТЕКУЩИЙ ЗАПАС",
        "RECIPES", "РЕЦЕПТЫ"
    ];

    let mut i = 0;
    while i < changes.len() {
        contents = contents.replace(changes[i], changes[i+1]);
        i += 2;
    }

    match write_file(file, contents) {
        Err(e) => panic!(e),
        _ => ()
    };
}

fn new_ingredient_ejs(file: &str) {
    let mut contents = match read_file(file) {
        Ok(contents) => contents,
        Err(e) => panic!(e)
    };

    // TODO: need good translation for "EACH"
    // TODO: need transaction for spreadhseet upload link
    let changes = [
        "CREATE INGREDIENT", "СОЗДАТЬ ИНГРИДИЕНТ",
        "NAME", "НАЗВАНИЕ",
        "CATEGORY", "КАТЕГОРИЯ",
        "QUANTITY", "КОЛИЧЕСТВО",
        "UNIT", "ЕДИНИЦА",
        ">G<", ">Г<",
        ">KG<", ">КГ<",
        "<option type=\"mass\" value=\"oz\">OZ</option>", "",
        "<option type=\"mass\" value=\"lb\">LB</option>", "",
        ">ML<", ">МЛ<",
        ">L<", ">Л<",
        "<option type=\"volume\" value=\"tsp\">TSP</option>", "",
        "<option type=\"volume\" value=\"tbsp\">TBSP</option>", "",
        "<option type=\"volume\" value=\"ozfl\">OZ. FL</option>", "",
        "<option type=\"volume\" value=\"cup\">CUP</option>", "",
        "<option type=\"volume\" value=\"pt\">PT</option>", "",
        "<option type=\"volume\" value=\"qt\">QT</option>", "",
        "<option type=\"volume\" value=\"gal\">GAL</option>", "",
        ">MM<", ">ММ<",
        ">CM<", ">СМ<",
        ">M<", ">М<",
        "<option type=\"length\" value=\"in\">IN</option>", "",
        "<option type=\"length\" value=\"ft\">FT</option>", "",
        "BOTTLE SIZE", "РАЗМЕР БУТЫЛКИ",
        "CREATE", "СОЗДАТЬ",
        "<option value=\"tsp\">TSP</option>", "",
        "<option value=\"tbsp\">TBSP</option>", "",
        "<option value=\"ozfl\">OZ. FL</option>", "",
        "<option value=\"cup\">CUP</option>", "",
        "<option value=\"pt\">PT</option>", "",
        "<option value=\"qt\">QT</option>", "",
        "<option value=\"gal\">GAL</option>", ""
    ];
    
    let mut i = 0;
    while i < changes.len() {
        contents = contents.replace(changes[i], changes[i+1]);
        i += 2;
    }

    match write_file(file, contents) {
        Err(e) => panic!(e),
        _ => ()
    };
}

fn new_order_ejs(file: &str) {
    let mut contents = match read_file(file) {
        Ok(contents) => contents,
        Err(e) => panic!(e)
    };

    // TODO: need translation for name/id
    //       need translation for spreadsheet option
    let changes = [
        "INGREDIENTS", "ИНГРЕДИЕНТЫ",
        "NEW ORDER", "НОВЫЙ ЗАКАЗ",
        "TAXES", "НАЛОГИ",
        "OTHER FEES", "ТАРИФЫ",
        "CREATE", "СОЗДАТЬ",
        "REMOVE", "УДАЛИТЬ"
    ];

    let mut i = 0;
    while i < changes.len() {
        contents = contents.replace(changes[i], changes[i+1]);
        i += 2;
    }

    match write_file(file, contents) {
        Err(e) => panic!(e),
        _ => ()
    };
}

fn new_recipe_ejs(file: &str) {
    let mut contents = match read_file(file) {
        Ok(contents) => contents,
        Err(e) => panic!(e)
    };

    // TODO: need translation for spreadsheet option
    let changes = [
        "NEW RECIPE", "НОВЫЙ РЕЦЕПТ",
        "NAME", "НАЗВАНИЕ",
        "# OF INGREDIENTS", "№ ИНГРЕДИЕНТОВ",
        "INGREDIENTS", "ИНГРЕДИЕНТОВ",
        "INGREDIENT", "ИНГРЕДИЕНТ",
        "QUANTITY", "КОЛИЧЕСТВО",
        "CREATE", "СОЗДАТЬ"
    ];

    let mut i = 0;
    while i < changes.len() {
        contents = contents.replace(changes[i], changes[i+1]);
        i += 2;
    }

    match write_file(file, contents) {
        Err(e) => panic!(e),
        _ => ()
    };
}

fn new_transaction_ejs(file: &str) {
    let mut contents = match read_file(file) {
        Ok(contents) => contents,
        Err(e) => panic!(e)
    };

    // TODO: need translation for spreadhseet option
    let changes = [
        "NEW TRANSACTION", "НОВАЯ ТРАНЗАКЦИЯ",
        "CREATE", "СОЗДАТЬ"
    ];

    let mut i = 0;
    while i < changes.len() {
        contents = contents.replace(changes[i], changes[i+1]);
        i += 2;
    }

    match write_file(file, contents) {
        Err(e) => panic!(e),
        _ => ()
    };
}