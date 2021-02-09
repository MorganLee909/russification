use crate::change::Change;

pub fn change<'a>() -> Vec<Change<'a>> {
    let folder_location = "../InventoryManagement/";

    let renderer_js = Change {
        location: format!("{}{}", folder_location, "controllers/renderer.js"),
        changes: vec![
            "ERROR: UNABLE TO RETRIEVE USER DATA",  "ОШИБКА: НЕ УДАЛОСЬ ПОЛУЧИТЬ ДАННЫЕ",
            "PLEASE VERIFY YOUR EMAIL ADDRESS", "ПОЖАЛУЙСТА, ПРОВЕРТЬЕ ВАШ ЭЛЕКТРОННЫЙ АДРЕС"
        ]
    };

    let merchant_data_js = Change {
        location: format!("{}{}", folder_location, "controllers/merchantData.js"),
        changes: vec![
            "PASSWORD MUST CONTAIN AT LEAST 10 CHARACTERS", "ПАРОЛЬ ДОЛЖЕН СОДЕРЖАТЬ НЕ МЕНЕЕ 10 СИМВОЛОВ",
            "USER WITH THIS EMAIL ADDRESS ALREADY EXISTS", "ПОЛЬЗОВАТЕЛЬ С ТАКИМ ЭЛЕКТРОННЫМ АДРЕСОМ УЖЕ СУЩЕСТВУЕТ",
            "ERROR: UNABLE TO RETRIEVE USER DATA",  "ОШИБКА: НЕ УДАЛОСЬ ПОЛУЧИТЬ ДАННЫЕ",
            "PASSWORD SUCCESSFULLY RESET. PLEASE LOG IN", "ПАРОЛЬ УСПЕШНО ИЗМЕНЁН. ВОЙДИТЕ, ПОЖАЛУЙСТА"
            "ERROR: UNABLE TO UPDATE YOUR PASSWORD", "ОШИБКА: НЕ УДАЛОСЬ ОБНОВИТЬ ПАРОЛЬ"
        ]
    };

    let ingredient_data_js = Change {
        location: format!("{}{}", folder_location, "controllers/ingredientData.js"),
        changes: vec![
            "ERROR: UNABLE TO RETRIEVE USER DATA",  "ОШИБКА: НЕ УДАЛОСЬ ПОЛУЧИТЬ ДАННЫЕ"
        ]
    };

    let recipe_data_js = Change {
        location: format!("{}{}", folder_location, "controllers/recipeData.js"),
        changes: vec![
            "ERROR: UNABLE TO RETRIEVE USER DATA",  "ОШИБКА: НЕ УДАЛОСЬ ПОЛУЧИТЬ ДАННЫЕ"
        ]
    };

    let other_data_js = Change {
        location: format!("{}{}", folder_location, "controllers/otherData.js"),
        changes: vec![
            "ERROR: UNABLE TO RETRIEVE USER DATA",  "ОШИБКА: НЕ УДАЛОСЬ ПОЛУЧИТЬ ДАННЫЕ",
            "INVALID EMAIL OR PASSWORD", "НЕВЕРНАЯ ПОЧТА ИЛИ ПАРОЛЬ"
        ]
    };

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
            "clientsupport@mail.thesubline.com", "no-reply@mg.sanoinventarium.com",
            "ERROR: UNABLE TO SEND VERIFICATION EMAIL", "ОШИБКА: НЕВОЗМОЖНО ОТПРАВИТЬ ПМСЬМО ДЛЯ ПРОВЕРКИ",
            "ERROR: UNABLE TO CHANGE YOUR EMAIL ADDRESS", "ОШИБКА: НЕ УДАЛОСЬ СМЕНИТЬ ЭЛЕКТРОННУЮ ПОЧТУ",
            "UNABLE TO VERIFY EMAIL ADDRESS.  INCORRECT LINK", "НЕ УДАЛОСЬ ПОДТВЕРДИТЬ ЭЛЕКТРОННУЮ ПОЧТУ",
            "ERROR: UNABLE TO VERIFY EMAIL ADDRESS", "ОШИБКА: НЕ УДАЛОСЬ ПОДТВЕРДИТЬ ЭЛЕКТРОННУЮ ПОЧТУ",
            "EMAIL VERIFIED.  PLEASE LOG IN", "ЭЛЕКТРОННАЯ ПОЧТА ПОДТВЕРЖДЁНА. ВХОД",
            "ERROR: UNABLE TO VERIFY EMAIL ADDRESS", "ОШИБКА: НЕВОЗМОЖНО ПОДТВЕРДИТЬ ЭЛЕКТРОННУЮ ПОЧТУ",
            "USER WITH THIS EMAIL ADDRESS ALREADY EXISTS", "ПОЛЬЗОВАТЕЛЬ С ТАКИМ ЭЛЕКТРОННЫМ АДРЕСОМ УЖЕ СУЩЕСТВУЕТ"
        ]
    };

    let password_reset_js = Change {
        location: format!("{}{}", folder_location, "controllers/passwordReset.js"),
        changes: vec![
            "{apiKey: process.env.MG_SUBLINE_APIKEY, domain: \"mail.thesubline.net\"}", "{apiKey: process.env.MG_PERSONAL_APIKEY, domain: \"mg.sanoinventarium.com\", host: \"api.eu.mailgun.net\"}",
            "The Subline <clientsupport@thesubline.net>", "Sano Inventarium <no-reply@sanoinventarium.com>",
            "PASSWORD MUST CONTAIN AT LEAST 10 CHARACTERS", "ПАРОЛЬ ДОЛЖЕН СОДЕРЖАТЬ НЕ МЕНЕЕ 10 СИМВОЛОВ",
            "ERROR: UNABLE TO UPDATE YOUR PASSWORD", "ОШИБКА: НЕ УДАЛОСЬ ОБНОВИТЬ ПАРОЛЬ",
            "USER WITH THIS EMAIL DOES NOT EXIST", "ПОЛЬЗОВАТЕЛЬ С ТАКИМ АДРЕСОМ НЕ СУЩЕСТВУЕТ",
            "ERROR: UNABLE TO RESET PASSWORD AT THIS TIME", "ОШИБКА: НЕ УДАЛОС СМЕНИТЬ ПАРОЛЬ",
            "YOUR ACCOUNT COULD NOT BE VERIFIED.  PLEASE CONTACT US IF THE PROBLEM PERSISTS.", "ВАШ АККАУНТ НЕ МОЖЕТ БЫТЬ ПОДТВЕРЖДЕН. ПОЖАЛУЙСТА, СВЯЖИТЕСЬ С НАМИ, ЕСЛИ ПРОБЛЕМА НЕ УСТРАНИТСЯ",
            "PASSWORD RESET EMAIL SENT", "ПИСЬМО ДЛЯ ВОССТАНОВЛЕНИЯ ПАРОЛЯ ОТПРАВЛЕНО",
            "PASSWORD SUCCESSFULLY UPDATED.  PLEASE LOG IN", "ПАРОЛЬ УСПЕШНО ИЗМЕНЕН. ВОЙТИ, ПОЖАЛУЙСТА"
        ]
    };

    let app_js = Change {
        location: format!("{}{}", folder_location, "app.js"),
        changes: vec![
            "www.thesubline.com", "www.sanoinventarium.com"
        ]
    };

    vec![
        renderer_js,
        merchant_data_js,
        ingredient_data_js,
        other_data_js,
        recipe_data_js,
        helper_js,
        email_verification_js,
        password_reset_js,
        app_js
    ]
}