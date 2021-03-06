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
            "title: \"$\"", "title: \"₽\"",
            "SOMETHING WENT WRONG. PLEASE REFRESH THE PAGE", "ЧТО-ТО ПОШЛО НЕ ТАК. ПОЖАЛУЙСТА, ОБНОВИТЕ СТРАНИЦУ.",
            "INGREDIENTS UPDATED", "ИНГРЕДИЕНТЫ ОБНОВЛЁН"
        ]
    };

    let recipe_book_js = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/strands/recipeBook.js"),
        changes: vec![
            "$$", "₽$",
            "SOMETHING WENT WRONG. PLEASE REFRESH THE PAGE", "ЧТО-ТО ПОШЛО НЕ ТАК. ПОЖАЛУЙСТА, ОБНОВИТЕ СТРАНИЦУ.",
            "RECIPES SUCCESSFULLY UPDATED", "РЕЦЕПТ УСПЕШНО ОБНОВЛЁН"
        ]
    };

    let analytics_js = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/strands/analytics.js"),
        changes: vec![
            "QUANTITY", "КОЛИЧЕСТВО",
            "\"DATE\"", "\"ДАТА\"",
            "$$", "₽$",
            "UNABLE TO UPDATE THE PAGE", "НЕВОЗМОЖНО ОБНОВИТЬ СТРАНИЦУ"
        ]
    };

    let orders_js = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/strands/orders.js"),
        changes: vec![
            "$$", "₽$",
            "SOMETHING WENT WRONG. PLEASE REFRESH THE PAGE", "ЧТО-ТО ПОШЛО НЕ ТАК. ПОЖАЛУЙСТА, ОБНОВИТЕ СТРАНИЦУ.",
            "UNABLE TO DISPLAY ORDERS", "НЕВОЗМОЖНО ОТРАЗИТЬ ЗАКАЗЫ"
        ]
    };

    let transactions_js = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/strands/transactions.js"),
        changes: vec![
            "$$", "₽$"
        ]
    };

    let account_js = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/strands/account.js"),
        changes: vec![
            "SOMETHING WENT WRONG. PLEASE REFRESH THE PAGE", "ЧТО-ТО ПОШЛО НЕ ТАК. ПОЖАЛУЙСТА, ОБНОВИТЕ СТРАНИЦУ.",
            "DATA UPDATED", "ДАННЫЕ ОБНОВИЛИ"
        ]
    };

    let new_recipe_js = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/sidebars/newRecipe.js"),
        changes: vec![
            "`INGREDIENT", "`ИНГРЕДИЕНТ",
            "SOMETHING WENT WRONG. PLEASE REFRESH THE PAGE", "ЧТО-ТО ПОШЛО НЕ ТАК. ПОЖАЛУЙСТА, ОБНОВИТЕ СТРАНИЦУ.",
            "RECIPE CREATED", "РЕЦЕПТ СОЗДАН",
            "ALL INGREDIENTS SUCCESSFULLY CREATED", "ВСЕ ИНГРЕДИЕНТЫ УСПЕШНО СОЗДАНЫ"
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
            "$$", "₽$",
            "SOMETHING WENT WRONG. PLEASE REFRESH THE PAGE", "ЧТО-ТО ПОШЛО НЕ ТАК. ПОЖАЛУЙСТА, ОБНОВИТЕ СТРАНИЦУ.",
            "TRANSACTION REMOVED", "ТРАНЗАКЦИЯ УДАЛЕНА"
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
            "order.getTotalCost().toFixed(2)", "order.getTotalCost()",
            "SOMETHING WENT WRONG. PLEASE REFRESH THE PAGE", "ЧТО-ТО ПОШЛО НЕ ТАК. ПОЖАЛУЙСТА, ОБНОВИТЕ СТРАНИЦУ.",
            "ORDER REMOVED", "ЗАКАЗ УДАЛЁН"
        ]
    };

    let recipe_details_js = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/sidebars/recipeDetails.js"),
        changes: vec![
            "$$", "₽$",
            "SOMETHING WENT WRONG. PLEASE REFRESH THE PAGE", "ЧТО-ТО ПОШЛО НЕ ТАК. ПОЖАЛУЙСТА, ОБНОВИТЕ СТРАНИЦУ.",
            "RECIPE REMOVED", "РЕЦЕПТ УДАЛЁН"
        ]
    };

    let edit_ingredient_js = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/sidebars/editIngredient.js"),
        changes: vec![
            "SOMETHING WENT WRONG. PLEASE REFRESH THE PAGE", "ЧТО-ТО ПОШЛО НЕ ТАК. ПОЖАЛУЙСТА, ОБНОВИТЕ СТРАНИЦУ.",
            "INGREDIENT UPDATED", "ИНГРЕДИЕНТ ОБНОВЛЁН"
        ]
    };

    let edit_recipe_js = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/sidebars/editRecipe.js"),
        changes: vec![
            "SOMETHING WENT WRONG. PLEASE REFRESH THE PAGE", "ЧТО-ТО ПОШЛО НЕ ТАК. ПОЖАЛУЙСТА, ОБНОВИТЕ СТРАНИЦУ.",
            "RECIPE UPDATED", "РЕЦЕПТ ОБНОВЛЁН"
        ]
    };

    let ingredient_details_js = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/sidebars/ingredientDetails.js"),
        changes: vec![
            "SOMETHING WENT WRONG. PLEASE REFRESH THE PAGE", "ЧТО-ТО ПОШЛО НЕ ТАК. ПОЖАЛУЙСТА, ОБНОВИТЕ СТРАНИЦУ.",
            "MUST REMOVE INGREDIENT FROM ALL RECIPES BEFORE REMOVING FROM INVENTORY", "УДАЛИТЕ ИНГРЕДИЕНТ ИЗ ВСЕХ РЕЦЕПТОВ, ПРЕЖДЕ ЧЕМ УДАЛЯТЬ ЕГО ИЗ ЗАПАСОВ",
            "INGREDIENT REMOVED", "ИНГРЕДИЕНТ УДЛЁН"
        ]
    };
    
    let new_ingredient_js = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/sidebars/newIngredient.js"),
        changes: vec![
            "SOMETHING WENT WRONG. PLEASE REFRESH THE PAGE", "ЧТО-ТО ПОШЛО НЕ ТАК. ПОЖАЛУЙСТА, ОБНОВИТЕ СТРАНИЦУ.",
            "INGREDIENT CREATED", "ИНГРЕДИЕНТ СОЗДАН",
            "INGREDIENTS SUCCESSFULLY ADDED", "ИНГРЕДИЕНТ УСПЕШНО ДОБАВЛЕН"
        ]
    };

    let new_order_js = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/sidebars/newOrder.js"),
        changes: vec![
            "SOMETHING WENT WRONG. PLEASE REFRESH THE PAGE", "ЧТО-ТО ПОШЛО НЕ ТАК. ПОЖАЛУЙСТА, ОБНОВИТЕ СТРАНИЦУ.",
            "NEW ORDER CREATED", "НОВЫЙ ЗАКАЗ СОЗДАН",
            "ORDER CREATED AND INGREDIENTS UPDATED SUCCESSFULLY", "ЗАКАЗ СОЗДАН, ИНГРЕДИЕНТЫ УСПЕШНО ОБНОВЛЕНЫ"
        ]
    };

    let new_transaction_js = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/sidebars/newTransaction.js"),
        changes: vec![
            "SOMETHING WENT WRONG. PLEASE REFRESH THE PAGE", "ЧТО-ТО ПОШЛО НЕ ТАК. ПОЖАЛУЙСТА, ОБНОВИТЕ СТРАНИЦУ.",
            "TRANSACTION CREATED", "ТРАНЗАКЦИЯ СОЗДАНА",
            "TRANSACTION SUCCESSFULLY CREATED.  INGREDIENTS UPDATED", "ТРАНЗАКЦИЯ УСПЕШНО СОЗДАНА. ИНГРЕДИЕНТЫ ОБНОВЛЕНЫ"
        ]
    };

    let transaction_filter_js = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/sidebars/transactionFilter.js"),
        changes: vec![
            "SOMETHING WENT WRONG. PLEASE REFRESH THE PAGE", "ЧТО-ТО ПОШЛО НЕ ТАК. ПОЖАЛУЙСТА, ОБНОВИТЕ СТРАНИЦУ.",
            "START DATE CANNOT BE AFTER END DATE", "ДАТА НАЧАЛА НЕ МОЖЕТ БЫТЬ ПОЗЖЕ ДАТЫ ОКОНЧАНИЯ",
            "NO TRANSACTIONS MATCH YOUR SEARCH", "НЕТ ТРАНЗАКЦИЙ СООТВЕТСТВУЮЩИХ ВАШЕМУ ЗАПРОСУ"
        ]
    };

    let order_calculator_js = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/sidebars/orderCalculator.js"),
        changes: vec![
            "ERROR: UNABLE TO MAKE PREDICTION", "ОШИБКА: НЕ УДАЛОСЬ СДЕЛАТЬ ПРОГНОЗ"
        ]
    };

    let order_filter_js = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/sidebars/orderFilter.js"),
        changes: vec![
            "NO ORDERS MATCH YOUR SEARCH", "НЕТ ЗАКАЗОВ, СООТВЕТСТВУЮЩИХ ВАШЕМУ ЗАПРОСУ"
        ]
    };

    let middleware_js = Change {
        location: format!("{}{}", folder_location, "middleware.js"),
        changes: vec![
            "PLEASE LOG IN", "ВХОД",
            "ERROR: UNABLE TO RETRIEVE DATA", "ОШИБКА: НЕ УДАЛОСЬ ПОЛУЧИТЬ ДАННЫЕ"
        ]
    };

    vec![
        home_js,
        recipe_book_js,
        analytics_js,
        orders_js,
        transactions_js,
        account_js,
        new_recipe_js,
        transaction_details_js,
        order_details_js,
        recipe_details_js,
        edit_ingredient_js,
        edit_recipe_js,
        ingredient_details_js,
        new_ingredient_js,
        new_order_js,
        new_transaction_js,
        transaction_filter_js,
        order_calculator_js,
        order_filter_js,
        middleware_js
    ]
}