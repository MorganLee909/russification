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
            "QUANTITY", "КОЛИЧЕСТВО",
            "$$", "₽$",
            "title: \"$\"", "title: \"₽\""
        ]
    };

    let recipe_book_js = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/strands/recipeBook.js"),
        changes: vec![
            "$$", "₽$"
        ]
    };

    let analytics_js = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/strands/analytics.js"),
        changes: vec![
            "QUANTITY", "КОЛИЧЕСТВО",
            "\"DATE\"", "\"ДАТА\"",
            "$$", "₽$"
        ]
    };

    let orders_js = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/strands/orders.js"),
        changes: vec![
            "$$", "₽$"
        ]
    };

    let transactions_js = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/strands/transactions.js"),
        changes: vec![
            "$$", "₽$"
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
            "recipes`", "рецептов`",
            "$$", "₽$"
        ]
    };

    let order_details_js = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/sidebars/orderDetails.js"),
        changes: vec![
            "$$", "₽$",
            "order.taxes.toFixed(2)", "order.taxes",
            "order.fees.toFixed(2)", "order.fees",
            "order.ingredients[i].cost().toFixed(2)", "order.ingredients[i].cost()",
            "order.ingredients[i].pricePerUnit.toFixed(2)", "order.ingredients[i].pricePerUnit",
            "order.ingredients[i].pricePerUnit.toFixed(2)", "order.ingredients[i].pricePerUnit",
            "order.getTotalCost().toFixed(2)", "order.getTotalCost()"
        ]
    };

    let recipe_details_js = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/sidebars/recipeDetails.js"),
        changes: vec![
            "$$", "₽$"
        ]
    };

    vec![
        home_js,
        recipe_book_js,
        analytics_js,
        orders_js,
        transactions_js,
        new_recipe_js,
        transaction_details_js,
        order_details_js,
        recipe_details_js
    ]
}