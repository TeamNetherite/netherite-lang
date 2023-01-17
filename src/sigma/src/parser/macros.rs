macro_rules! check_token {
    ($p: ident, $t: ident, $expected: expr, $expected_for: expr) => {
        if mem::discriminant(&$p.$t.value) != mem::discriminant(&$expected) {
            let c = ColorGenerator::new().next();

            let mut label_message = format!("expected {}", $expected.to_string().fg(c));

            if let Some(_) = $expected_for {
                label_message.push_str(format!(" for {}", $expected_for.unwrap()).as_str());
            }

            let mut builder = Report::build(ReportKind::Error, $p.filename, $p.$t.span.range.start)
                .with_code(1)
                .with_message(format!("unexpected {}", $p.$t.value))
                .with_label(
                    Label::new(($p.filename, $p.$t.span.range.to_owned()))
                        .with_message(label_message)
                        .with_color(c),
                );

            builder = match (&$p.$t.value, &$expected) {
                (&RawToken::Identifier(ref s), &RawToken::String(_)) => builder.with_note(format!(
                    "consider wrapping {} inside double quotes and replace with {}.",
                    c.paint(s),
                    c.paint(format!("\"{}\"", s))
                )),
                _ => builder,
            };

            builder
                .finish()
                .print(($p.filename, Source::from($p.contents)))
                .unwrap();

            None
        } else {
            Some(())
        }
    };
}

macro_rules! empty_string {
    () => {
        String::from("")
    };
}
