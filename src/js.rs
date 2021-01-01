use crate::change::Change;

pub fn change<'a>() -> Vec<Change<'a>> {
    let folder_location: &str = "../../javascript/InventoryManagement/";

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

    vec![
        home_js
    ]
}