pub fn mask_card_number(card_number: &str) -> String {
    let len = card_number.len();
    if len <= 4 {
        card_number.to_string()
    } else {
        "*".repeat(len - 4) + &card_number[len - 4..]
    }
}
