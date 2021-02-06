use crate::change::Change;

pub fn change<'a>() -> Vec<Change<'a>> {
    let folder_location = "../InventoryManagement/";

    let sidebars_css = Change {
        location: format!("{}{}", folder_location, "views/dashboardPage/sidebars.css"),
        changes: vec![
            ".menuLogo{\n        width: 25%;", ".menuLogo{\n        width: 90%;"
        ]
    };

    vec![
        sidebars_css
    ]
}