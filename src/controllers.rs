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
            "{apiKey: process.env.MG_SUBLINE_APIKEY, domain: \"mail.thesubline.net\"}", "{apiKey: process.env.MG_PERSONAL_APIKEY, domain: \"mg.sanoinventarium.com\", host: \"api.eu.mailgun.net\"}",
            "The Subline <clientsupport@thesubline.net>", "Sano Inventarium <no-reply@sanoinventarium.com>",
            "clientsupport@mail.thesubline.com", "no-reply@mg.sanoinventarium.com"
        ]
    };

    let password_reset_js = Change {
        location: format!("{}{}", folder_location, "controllers/passwordReset.js"),
        changes: vec![
            "{apiKey: process.env.MG_SUBLINE_APIKEY, domain: \"mail.thesubline.net\"}", "{apiKey: process.env.MG_PERSONAL_APIKEY, domain: \"mg.sanoinventarium.com\", host: \"api.eu.mailgun.net\"}",
            "The Subline <clientsupport@thesubline.net>", "Sano Inventarium <no-reply@sanoinventarium.com>"
        ]
    };

    let app_js = Change {
        location: format!("{}{}", folder_location, "app.js"),
        changes: vec![
            "www.thesubline.com", "www.sanoinventarium.com"
        ]
    };

    vec![
        helper_js,
        email_verification_js,
        password_reset_js,
        app_js
    ]
}