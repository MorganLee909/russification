use crate::change::Change;

pub fn change<'a>() -> Vec<Change<'a>> {
    let folder_location: &str = "../InventoryManagement/";

    let home_js = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/strands/home.js"),
        changes: vec![
            "% vs last month", "% по сравнению с прошлым месяцем",
            "REVENUE", "ДОХОД",
            "\"DATE\"", "\"ДАТА\"",
            "MOST POPULAR INGREDIENTS", "САМЫЕ ПОПУЛЯРНЫЕ ИНГРЕДИЕНТЫ",
            "QUANTITY", "КОЛИЧЕСТВО"
        ]
    };

    let analytics_ejs = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/strands/analytics.js"),
        changes: vec![
            "QUANTITY", "КОЛИЧЕСТВО",
            "\"DATE\"", "\"ДАТА\"",
        ]
    };

    let new_recipe_js = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/sidebars/newRecipe.js"),
        changes: vec![
            "`INGREDIENT", "`ИНГРЕДИЕНТ"
        ]
    };

    let transaction_details_js = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/sidebars/transactionDetails.js"),
        changes: vec![
            "January", "январь",
            "February", "февраль",
            "March", "март",
            "April", "апрель",
            "May", "май",
            "June", "июнь",
            "July", "июль",
            "August", "август",
            "September", "сентябрь",
            "October", "октябрь",
            "November", "ноябрь",
            "December", "декабрь",
            "Sunday", "воскресенье",
            "Monday", "понедельник",
            "Tuesday", "вторник",
            "Wednesday", "среда",
            "Thursday", "четверг",
            "Friday", "пятница",
            "Saturday", "суббота",
            "recipes`", "рецептов`"
        ]
    };

    vec![
        home_js,
        analytics_ejs,
        new_recipe_js,
        transaction_details_js
    ]
}