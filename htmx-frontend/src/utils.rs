use rust_decimal::Decimal;

pub fn display_decimal(money: Decimal) -> String {
    format!("${}", money)
}
