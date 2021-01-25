use crate::change::Change;

pub fn change<'b>() -> Vec<Change<'b>> {
    let folder_location: &str = "../InventoryManagement/";

    let menu_ejs = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/ejs/menu.ejs"),
        changes: vec![
            "THE SUBLINE", "SANO",
            "DASHBOARD", "ГЛАВНАЯ",
            "INGREDIENTS", "ИНГРЕДИЕНТЫ",
            "RECIPE BOOK", "КНИГА РЕЦЕПТОВ",
            "ANALYTICS", "АНАЛИТИКА",
            "ORDERS", "ЗАКАЗЫ",
            "TRANSACTIONS", "ТРАНЗАКЦИИ",
            "LOGOUT", "ВЫХОД",
        ]
    };

    let analytics_ejs = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/ejs/strands/analytics.ejs"),
        changes: vec![
            "SUBMIT", "РАЗМЕСТИТЬ",
            "ANALYTICS", "АНАЛИТИКА",
            "INGREDIENTS", "ИНГРЕДИЕНТЫ",
            "RECIPES", "РЕЦЕПТЫ",
            "Date Range", "ДИАПАЗОН ДАТ",
            "MINIMUM DAILY USE", "МИНИМАЛЬНОЕ ЕЖЕДНЕВНОЕ ИСПОЛЬЗОВАНИЕ",
            "AVERAGE DAILY USE", "СРЕДНЕЕ ЕЖЕДНЕВНОЕ ИСПОЛЬЗОВАНИЕ",
            "MAXIMUM DAILY USE", "МАКСИМАЛЬНОЕ ЕЖЕДНЕВНОЕ ИСПОЛЬЗОВАНИЕ",
            "Sunday Average", "в среднем воскресенья",
            "Monday Average", "в среднем понидельника",
            "Tuesday Average", "в среднем вторника",
            "Wednesday Average", "в среднем среды",
            "Thursday Average", "в среднем четверга",
            "Friday Average", "в среднем пятницы",
            "Saturday Average", "в среднем субботы",
            "Average Daily Sales", "Среднесуточные продажи",
            "Average Daily Revenue", "Средний дневной доход"

        ]
    };

    let home_ejs = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/ejs/strands/home.ejs"),
        changes: vec![
            "DASHBOARD", "ГЛАВНАЯ",
            "Total Revenue (month)", "Общий доход (месяц)",
            "Inventory Check", "Проверить Ингредиеты",
            ">Update", ">ОБНОВИТЬ"
        ]
    };

    let ingredients_ejs = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/ejs/strands/ingredients.ejs"),
        changes: vec![
            "INGREDIENT INVENTORY", "ИНГРЕДИЕНТЫ",
            "NEW", "НОВЫЙ",
            "SEARCH", "ПОИСК"
        ]
    };

    let orders_ejs = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/ejs/strands/orders.ejs"),
        changes: vec![
            "ORDERS", "ЗАКАЗЫ",
            "FILTER", "ФИЛЬТРОВАТЬ",
            "NEW", "НОВЫЙ",
            "CALCULATOR", "КАЛЬКУЛЯТОР",
            "PREDICTOR", "ПРЕДИКТОР"
        ]
    };

    let recipe_book_ejs = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/ejs/strands/recipeBook.ejs"),
        changes: vec![
            "RECIPE BOOK", "КНИГА РЕЦЕПТОВ",
            "NEW", "НОВЫЙ",
            "SEARCH", "ПОИСК"
        ]
    };

    let transactions_ejs = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/ejs/strands/transactions.ejs"),
        changes: vec![
            "TRANSACTIONS", "ТРАНЗАКЦИИ",
            "NEW", "НОВЫЙ",
            "FILTER", "ФИЛЬТРОВАТЬ"
        ]
    };

    let edit_ingredient_ejs = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/ejs/sidebars/editIngredient.ejs"),
        changes: vec![
            "NAME", "НАЗВАНИЕ",
            "CATEGORY", "КАТЕГОРИЯ",
            "MEASUREMENT UNIT", "ЕДИНИЦА ИЗМЕРЕНИЯ",
            "SUBMIT", "РАЗМЕСТИТЬ"
        ]
    };

    let edit_recipe_ejs = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/ejs/sidebars/editRecipe.ejs"),
        changes: vec![
            "NAME", "НАЗВАНИЕ",
            "INGREDIENTS", "ИНГРЕДИЕНТЫ",
            "PRICE", "ЦЕНА",
            "UPDATE", "ОБНОВИТЬ",
            "CANCEL", "ОТМЕНИТЬ"
        ]
    };

    let ingredient_details_ejs = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/ejs/sidebars/ingredientDetails.ejs"),
        changes: vec![
            "CURRENT STOCK", "ТЕКУЩИЙ ЗАПАС",
            "RECIPES", "РЕЦЕПТЫ",
            "AVERAGE DAILY USE (30 DAYS)", "СРЕДНЕЕ ЕЖЕДНЕВНОЕ ИСПОЛЬЗОВАНИЕ (30 ДНЕЙ)"
        ]
    };

    let new_ingredient_ejs = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/ejs/sidebars/newIngredient.ejs"),
        changes: vec![
            "CREATE INGREDIENT", "СОЗДАТЬ ИНГРИДИЕНТ",
            "NAME", "НАЗВАНИЕ",
            "CATEGORY", "КАТЕГОРИЯ",
            "QUANTITY", "КОЛИЧЕСТВО",
            "UNIT", "ЕДИНИЦА",
            "MASS/WEIGHT", "МАССА",
            "<option type=\"mass\" value=\"g\">G</option>", "<option type=\"mass\" value=\"г\">Г</option>",
            "<option type=\"mass\" value=\"kg\">KG</option>", "<option type=\"mass\" value=\"кг\">КГ</option>",
            "<option type=\"mass\" value=\"oz\">OZ</option>", "",
            "<option type=\"mass\" value=\"lb\">LB</option>", "",
            "VOLUME", "ОБЪЕМ",
            "<option type=\"volume\" value=\"ml\">ML</option>", "<option type=\"volume\" value=\"мл\">МЛ</option>",
            "<option type=\"volume\" value=\"l\">L</option>", "<option type=\"volume\" value=\"л\">Л</option>",
            "<option type=\"volume\" value=\"tsp\">TSP</option>", "",
            "<option type=\"volume\" value=\"tbsp\">TBSP</option>", "",
            "<option type=\"volume\" value=\"ozfl\">OZ. FL</option>", "",
            "<option type=\"volume\" value=\"cup\">CUP</option>", "",
            "<option type=\"volume\" value=\"pt\">PT</option>", "",
            "<option type=\"volume\" value=\"qt\">QT</option>", "",
            "<option type=\"volume\" value=\"gal\">GAL</option>", "",
            "LENGTH", "ДЛИНА",
            "<option type=\"length\" value=\"mm\">MM</option>", "<option type=\"length\" value=\"мм\">ММ</option>",
            "<option type=\"length\" value=\"cm\">СМ</option>", "<option type=\"length\" value=\"см\">СМ</option>",
            "<option type=\"length\" value=\"m\">М</option>", "<option type=\"length\" value=\"м\">М</option>",
            "<option type=\"length\" value=\"in\">IN</option>", "",
            "<option type=\"length\" value=\"ft\">FT</option>", "",
            "OTHER", "ДРУГОЙ",
            "<option type=\"other\" value=\"each\">EACH</option>", "<option type=\"other\" value=\"каждый\">КАЖДЫЙ</option>",
            "<option type=\"other\" value=\"bottle\">BOTTLE</option>", "<option type=\"other\" value=\"бутылка\">БУТЫЛКА</option>",
            "BOTTLE SIZE", "РАЗМЕР БУТЫЛКИ",
            "<option value=\"ml\">ML</option>", "<option value=\"мл\">МЛ</option>",
            "<option value=\"l\">L</option>", "<option value=\"л\">Л</option>",
            "CREATE", "СОЗДАТЬ",
            "<option value=\"tsp\">TSP</option>", "",
            "<option value=\"tbsp\">TBSP</option>", "",
            "<option value=\"ozfl\">OZ. FL</option>", "",
            "<option value=\"cup\">CUP</option>", "",
            "<option value=\"pt\">PT</option>", "",
            "<option value=\"qt\">QT</option>", "",
            "<option value=\"gal\">GAL</option>", "",
            "Or, upload a spreadsheet", "Или, загрузить электронную таблицу",
        ]
    };

    let new_order_ejs = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/ejs/sidebars/newOrder.ejs"),
        changes: vec![
            "INGREDIENTS", "ИНГРЕДИЕНТЫ",
            "NEW ORDER", "НОВЫЙ ЗАКАЗ",
            "TAXES", "НАЛОГИ",
            "OTHER FEES", "ТАРИФЫ",
            "CREATE", "СОЗДАТЬ",
            "REMOVE", "УДАЛИТЬ",
            "DATE", "ДАТА",
            "NAME/ID", "НАЗВАНИЕ",
            "Or, upload a spreadsheet", "Или, загрузить электронную таблицу"
        ]
    };

    let new_recipe_ejs = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/ejs/sidebars/newRecipe.ejs"),
        changes: vec![
            "NEW RECIPE", "НОВЫЙ РЕЦЕПТ",
            "NAME", "НАЗВАНИЕ",
            "# OF INGREDIENTS", "№ ИНГРЕДИЕНТОВ",
            "INGREDIENTS", "ИНГРЕДИЕНТОВ",
            "INGREDIENT", "ИНГРЕДИЕНТ",
            "QUANTITY", "КОЛИЧЕСТВО",
            "CREATE", "СОЗДАТЬ",
            "Or, upload a spreadsheet", "Или, загрузить электронную таблицу"
        ]
    };

    let new_transaction_ejs = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/ejs/sidebars/newTransaction.ejs"),
        changes: vec![
            "NEW TRANSACTION", "НОВАЯ ТРАНЗАКЦИЯ",
            "Create", "СОЗДАТЬ",
            "Or, upload a spreadsheet", "Или, загрузить электронную таблицу"
        ]
    };

    let order_details_ejs = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/ejs/sidebars/orderDetails.ejs"),
        changes: vec![
            "GRAND TOTAL", "ОБЩИЙ ИТОГ",
            "TOTAL", "ИТОГ",
            "TAXES", "НАЛОГИ",
            "FEES", "ТАРИФЫ"
        ]
    };

    let order_filter_ejs = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/ejs/sidebars/orderFilter.ejs"),
        changes: vec![
            "FILTER ORDERS", "ФИЛЬТРОВАТЬ ЗАКАЗЫ",
            "Date Range", "ДИАПАЗОН ДАТ",
            ">Ingredients", ">ИНГРЕДИЕНТЫ",
            "SUBMIT", "РАЗМЕСТИТЬ"
        ]
    };

    let recipe_details_ejs = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/ejs/sidebars/recipeDetails.ejs"),
        changes: vec![
            "PRICE", "ЦЕНА"
        ]
    };

    let transaction_details_ejs = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/ejs/sidebars/transactionDetails.ejs"),
        changes: vec![
            "RECIPES", "РЕЦЕПТЫ",
            "TOTALS", "ИТОГИ"
        ]
    };

    let transaction_filter_ejs = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/ejs/sidebars/transactionFilter.ejs"),
        changes: vec![
            "FILTER TRANSACTIONS", "ФИЛЬТРОВАТЬ ТРАНЗАКЦИИ",
            "RECIPES", "РЕЦЕПТЫ",
            "Date Range", "ДИАПАЗОН ДАТ",
            "SUBMIT", "РАЗМЕСТИТЬ",
            "Recipes", "Рецепты"
        ]
    };

    let order_calculator_ejs = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/ejs/sidebars/orderCalculator.ejs"),
        changes: vec![
            "ORDER PREDICTION", "ПРОГНОЗ ЗАКАЗА",
            "Date Range", "ДИАПАЗОН ДАТ",
            "PREDICT", "ПРОГНОЗ"
        ]
    };

    vec![
        menu_ejs,
        analytics_ejs,
        home_ejs,
        ingredients_ejs,
        orders_ejs,
        recipe_book_ejs,
        transactions_ejs,
        edit_ingredient_ejs,
        edit_recipe_ejs,
        ingredient_details_ejs,
        new_ingredient_ejs,
        new_order_ejs,
        new_recipe_ejs,
        new_transaction_ejs,
        order_details_ejs,
        order_filter_ejs,
        recipe_details_ejs,
        transaction_details_ejs,
        transaction_filter_ejs,
        order_calculator_ejs
    ]
}