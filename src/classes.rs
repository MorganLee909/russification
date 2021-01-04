use crate::change::Change;

pub fn change<'a>() -> Vec<Change<'a>> {
    let folder_location: &str = "../../javascript/InventoryManagement/";

    let merchant_js = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/classes/Merchant.js"),
        changes: vec![
            "\"g\"", "\"г\"",
            "\"kg\"", "\"кг\"",
            "\"ml\"", "\"мл\"",
            "\"l\"", "\"л\"",
            "\"mm\"", "\"мм\"",
            "\"cm\"", "\"см\"",
            "\"m\"", "\"м\""
        ]
    };

    let ingredient_js = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/classes/Ingredient.js"),
        changes: vec![
            "\"g\"", "\"г\"",
            "\"kg\"", "\"кг\"",
            "\"ml\"", "\"мл\"",
            "\"l\"", "\"л\"",
            "\"mm\"", "\"мм\"",
            "\"cm\"", "\"см\"",
            "\"m\"", "\"м\"",
            "\"bottle\"", "\"бутылка\""
        ]
    };

    let order_js = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/classes/Order.js"),
        changes: vec![
            "\"g\"", "\"г\"",
            "\"kg\"", "\"кг\"",
            "\"ml\"", "\"мл\"",
            "\"l\"", "\"л\"",
            "\"mm\"", "\"мм\"",
            "\"cm\"", "\"см\"",
            "\"m\"", "\"м\""
        ]
    };

    let recipe_js = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/js/classes/Recipe.js"),
        changes: vec![
            "\"g\"", "\"г\"",
            "\"kg\"", "\"кг\"",
            "\"ml\"", "\"мл\"",
            "\"l\"", "\"л\"",
            "\"mm\"", "\"мм\"",
            "\"cm\"", "\"см\"",
            "\"m\"", "\"м\""
        ]
    };

    vec![
        merchant_js,
        ingredient_js,
        order_js,
        recipe_js
    ]
}