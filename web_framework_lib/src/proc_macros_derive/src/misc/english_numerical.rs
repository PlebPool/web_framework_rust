pub fn match_numerica_to_lingua_franca<'a>(c: char) -> &'a str {
    match c {
        '0' => { "ZERO" }, '1' => { "ONE" }, '2' => { "TWO" }, '3' => { "THREE" }, '4' => { "FOUR" },
        '5' => { "FIVE" }, '6' => { "SIX" }, '7' => { "SEVEN" }, '8' => { "EIGHT" }, '9' => { "NINE" },
        _ => unimplemented!()
    }
}

pub fn match_lingua_franca_to_numerical(s: &str) -> char {
    match s.to_uppercase().as_str() {
        "ZERO" => { '0' }, "ONE" => { '1' }, "TWO" => { '2' }, "THREE" => { '3' }, "FOUR" => { '4' },
        "FIVE" => { '5' }, "SIX" => { '6' }, "SEVEN" => { '7' }, "EIGHT" => { '8' }, "NINE" => { '9' },
        _ => unimplemented!()
    }
}

pub fn starts_with_numerical_lingua_franca(s: &str) -> Result<&str, ()> {
    const VEC: &[&str] = &["ZERO", "ONE", "TWO", "THREE", "FOUR", "FIVE", "SIX", "SEVEN",
        "EIGHT", "NINE"];
    for a in VEC {
        if s.starts_with(a) {
            return Ok( a )
        }
    }
    Err( () )
}