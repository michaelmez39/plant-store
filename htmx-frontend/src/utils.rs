use bigdecimal::BigDecimal;

pub fn display_decimal(money: &BigDecimal) -> String {
    format!("${}", money)
}
