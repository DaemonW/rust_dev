use once_cell::sync::Lazy;
use regex::Regex;

const EXPR_USERNAME: &str = r"^[a-z0-9_]{8,20}$";
const EXPR_PASSWORD: &str = r"^[a-zA-Z0-9,\./<>\?!@\\#\$%\^&\*\(\)_=\-\+\[\]\{\}]{8,20}$";
const EXPR_EMAIL: &str =
    r"^[_A-Za-z0-9-]+(\.[_A-Za-z0-9-]+)*@[A-Za-z0-9]+(\.[A-Za-z0-9]+)*(\.[A-Za-z]{2,})$";
const EXPR_PHONE_CN: &str = r"^(\+86\s?)?(\d{11})$";

static PATTERN_USERNAME: Lazy<Regex> = Lazy::new(|| Regex::new(EXPR_USERNAME).unwrap());
static PATTERN_PHONE: Lazy<Regex> = Lazy::new(|| Regex::new(EXPR_PHONE_CN).unwrap());
static PATTERN_EMAIL: Lazy<Regex> = Lazy::new(|| Regex::new(EXPR_EMAIL).unwrap());
static PATTERN_PASSWORD: Lazy<Regex> = Lazy::new(|| Regex::new(EXPR_PASSWORD).unwrap());

pub fn username(s: impl AsRef<str>) -> Result<(), &'static str> {
    if PATTERN_USERNAME.is_match(s.as_ref()) {
        Ok(())
    } else {
        Err("username should be 8-20 length, construct with letter, number or underline")
    }
}

pub fn password(s: impl AsRef<str>) -> Result<(), &'static str> {
    if PATTERN_PASSWORD.is_match(s.as_ref()) {
        Ok(())
    } else {
        Err("password should be 8-24 english characters")
    }
}

pub fn email(s: impl AsRef<str>) -> Result<(), &'static str> {
    if PATTERN_EMAIL.is_match(s.as_ref()) {
        Ok(())
    } else {
        Err("email format is error")
    }
}

pub fn phone(s: impl AsRef<str>) -> Result<(), &'static str> {
    if PATTERN_PHONE.is_match(s.as_ref()) {
        Ok(())
    } else {
        Err("phone number format is error")
    }
}

#[cfg(test)]
mod test {
    use crate::utils::pattern::{email, password, phone};

    #[test]
    fn test_pattern() {
        let err_email = "test@email";
        let err_email2 = "testemail.com";
        let ok_email = "test@email.com";
        assert_eq!(email(err_email).is_err(), true);
        assert_eq!(email(err_email2).is_err(), true);
        assert_eq!(email(ok_email).is_ok(), true);

        let err_pass = "123457";
        let err_pass2 = "7564  454545";
        let ok_pass = "fw7fwFF998nn";
        assert_eq!(password(err_pass).is_err(), true);
        assert_eq!(password(err_pass2).is_err(), true);
        assert_eq!(password(ok_pass).is_ok(), true);

        let err_phone = "1573787455";
        let err_phone2 = "+8518647861584";
        let ok_phone = "18647861584";
        let ok_phone2 = "+8617684681547";
        assert_eq!(phone(err_phone).is_err(), true);
        assert_eq!(phone(err_phone2).is_err(), true);
        assert_eq!(phone(ok_phone).is_ok(), true);
        assert_eq!(phone(ok_phone2).is_ok(), true);
    }
}
