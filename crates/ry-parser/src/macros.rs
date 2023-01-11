macro_rules! check_token {
    ($p: ident, $expected: expr, $expected_for: literal) => {
        if let RawToken::Invalid(e) = $p.current.value {
            Err(ParserError::ErrorToken((e, $p.current.span.clone()).into()))
        } else if !&$p.current.value.is(&$expected) {
            Err(ParserError::UnexpectedTokenExpectedX(
                $p.current.clone(),
                $expected,
                Some($expected_for.to_owned()),
            ))
        } else {
            Ok(())
        }
    };
}

macro_rules! check_token0 {
    ($p: ident, $t_dump: expr, $expected: pat, $expected_for: expr) => {
        if let RawToken::Invalid(e) = $p.current.value {
            Err(ParserError::ErrorToken((e, $p.current.span.clone()).into()))
        } else if let $expected = $p.current.value {
            Ok(())
        } else {
            Err(ParserError::UnexpectedToken(
                $p.current.clone(),
                $t_dump.into(),
                Some($expected_for.into()),
            ))
        }
    };
    ($p: ident, $expected_for: expr, $expected: pat) => {
        if let RawToken::Invalid(e) = $p.current.value {
            Err(ParserError::ErrorToken((e, $p.current.span.clone()).into()))
        } else if let $expected = $p.current.value {
            Ok(())
        } else {
            Err(ParserError::UnexpectedToken(
                $p.current.clone(),
                $expected_for.into(),
                None,
            ))
        }
    };
}

macro_rules! parse_list_of_smth {
    ($p: ident, $list: ident, $closing_token: expr, $fn: expr) => {
        parse_list_of_smth!($p, $list, $closing_token, $fn, )
    };
    ($p: ident, $list: ident, $closing_token: expr, $fn: expr, $($fn_arg:expr)*) => {
        if !$p.current.value.is($closing_token) {
            loop {
                $list.push($fn($($fn_arg)*)?);

                if $p.current.value.is($closing_token) {
                    break
                } else {
                    check_token0!($p, format!("`,` or {:?}", $closing_token), RawToken::Comma, "enum variant")?;

                    $p.advance()?; // ','

                    if $p.current.value.is($closing_token) {
                        break
                    }
                }
            }
        }
    };
}

pub(crate) use {check_token, check_token0, parse_list_of_smth};
