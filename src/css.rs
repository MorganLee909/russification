use crate::change::Change;

pub fn change<'a>() -> Vec<Change<'a>> {
    let folder_location = "../InventoryManagement/";

    let sidebars_css = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/sidebars.css"),
        changes: vec![
            ".menuLogo{\n        width: 25%;", ".menuLogo{\n        width: 90%;",
            "rgb(0, 27, 45)", "rgb(33, 35, 55)",
            "rgb(178, 91, 209)", "rgb(70, 72, 97)"
        ]
    };

    let shared_css = Change {
        location: format!("{}{}", folder_location, "views/shared/shared.css"),
        changes: vec![
            "*{margin:0;padding:0;font-family:'Saira',sans-serif;}", "*{margin:0;padding:0;font-family:'Montserrat',sans-serif;}",
            "rgb(0, 27, 45)", "rgb(33, 35, 55)",
            "rgb(178, 91, 209)", "rgb(70, 72, 97)"
        ]
    };

    let dashboard_css = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/dashboard.css"),
        changes: vec![
            "rgb(0, 27, 45)", "rgb(33, 35, 55)",
            "rgb(178, 91, 209)", "rgb(70, 72, 97)"
        ]
    };

    vec![
        sidebars_css,
        shared_css,
        dashboard_css
    ]
}