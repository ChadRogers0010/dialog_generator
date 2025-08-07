#[allow(unused)]
pub fn build_query(path: impl Into<String>) {
    let file = std::fs::read_to_string(path.into()).unwrap();

    let mut lines = file.lines();
    let header = lines.next().unwrap();
    let predicate_count = header.split_terminator(",").count();
    let statement_count = lines.clone().count();

    let mut fuct_text = vec![
        String::from(
            "#[allow(unused)]\npub fn query(\n    predicates: &[i32],\n    response: &mut Vec<&'static str>,\n) -> Option<&'static str> {",
        ),
        String::from("unsafe{"),
        format!("println!(\"predicates: {predicate_count}\\nstatements: {statement_count}\");\n"),
    ];
    for h in header
        .split_terminator(",")
        .enumerate()
        .map(|(i, s)| format!("    const {}: usize = {i};", s.to_uppercase()))
    {
        fuct_text.push(h);
    }

    fuct_text.push("\n".to_string());

    for e in lines {
        let (predicate, string) = e.split_once(',').unwrap();
        let predicate_string = predicate
            .split(" ")
            .map(|s| {
                let first = s.as_bytes()[0] as char;
                match first {
                    'a'..'z' | 'A'..'Z' => {
                        format!("*predicates.get_unchecked({})", s.to_uppercase())
                    }
                    _ => s.to_string(),
                }
            })
            .fold(String::new(), |mut s, n| {
                s.push_str(n.as_str());
                s.push(' ');
                s
            });

        let statement = format!(
            "    if {predicate_string}{{\n        response.push({string:?});\n    }}
        ",
        );
        fuct_text.push(statement);
    }

    fuct_text.push(String::from("}"));

    let mut text = fuct_text.join("\n");
    text.push_str(
        "\r
    if response.len() == 0 {
        return None;
    }
    let rand_idx = rand::random_range(0..response.len());
    Some(response[rand_idx])
",
    );
    text.push_str("\r\n}");

    std::fs::write("./src/health.rs", &text).unwrap();
}
