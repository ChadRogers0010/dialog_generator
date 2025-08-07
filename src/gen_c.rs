pub fn build_query_c(path: impl Into<String>, _lines_per_module: usize) {
    let file = std::fs::read_to_string(path.into()).unwrap();

    let mut lines = file.lines();
    let header = lines.next().unwrap();
    let predicate_count = header.split_terminator(",").count();
    let statement_count = lines.clone().count();

    let mut fuct_text = vec![
        String::from("#include \"stddef.h\""),
        String::from("#include \"./vector.h\""),
        String::from("#include <stdio.h>"),
    ];

    fuct_text.push(String::from(
        "void query(
  int p[],
  Vector *response
) {",
    ));
    fuct_text.push(format!(
        "printf(\"predicates: {predicate_count}\\nstatements: {statement_count}\\n\");\n",
    ));

    for h in header
        .split_terminator(",")
        .enumerate()
        .map(|(i, s)| format!("  const size_t {} = {i};", s.to_uppercase()))
    {
        fuct_text.push(h);
    }
    // add newline
    fuct_text.push(String::new());

    for e in lines {
        let (predicate, string) = e.split_once(',').unwrap();

        let predicate_string = predicate
            .split(" ")
            .map(|s| {
                let first = s.as_bytes()[0] as char;
                match first {
                    'a'..'z' | 'A'..'Z' => {
                        format!("p[{}]", s.to_uppercase())
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
            "    if ({predicate_string}) {{\n    char *s = \"{string}\";\n    vector_push(response, s);\n    }}
        ",
        );
        fuct_text.push(statement);
    }

    fuct_text.push(String::from("}"));

    let text = fuct_text.join("\n");

    std::fs::write("./c/health.c", &text).unwrap();
}
