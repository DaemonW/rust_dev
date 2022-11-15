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
