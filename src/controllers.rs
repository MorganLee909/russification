use crate::change::Change;

pub fn change<'a>() -> Vec<Change<'a>> {
    let folder_location = "../InventoryManagement/";

    let helper_js = Change {
        location: format!("{}{}", folder_location, "controllers/helper.js"),
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

    let email_verification_js = Change {
        location: format!("{}{}", folder_location, "controllers/emailVerification.js"),
        changes: vec![
            "MG_SUBLINE_APIKEY", "MG_PERSONAL_APIKEY",
            "mail.thesubline.net", "mg.sanoinventarium.com",
            "<clientsupport@thesubline.net>", "<no-reply@sanoinventarium.com>",
            "clientsupport@mail.thesubline.com", "no-reply@mg.sanoinventarium.com"
        ]
    };

    let password_reset_js = Change {
        location: format!("{}{}", folder_location, "controllers/passwordReset.js"),
        changes: vec![
            "MG_SUBLINE_APIKEY", "MG_PERSONAL_APIKEY",
            "mail.thesubline.net", "mg.sanoinventarium.com",
            "<clientsupport@thesubline.net>", "<no-reply@sanoinventarium.com>"
        ]
    };

    vec![
        helper_js,
        email_verification_js,
        password_reset_js
    ]
}