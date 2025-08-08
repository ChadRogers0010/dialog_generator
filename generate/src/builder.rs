use std::{
    fs::{self, File},
    io::{BufWriter, Write},
};

pub struct Builder {
    pub path: String,
    pub dir_path: String,
    pub header: String,
    pub module_handles: Vec<String>,
}

pub fn build_query(path: impl Into<String> + Clone, lines_per_module: usize) {
    println!("lines_per_module: {lines_per_module}");
    let csv_file = std::fs::read_to_string(path.clone().into()).unwrap();
    let mut lines = csv_file.lines();
    let header = lines.next().unwrap().to_string();

    let print_statement = format!(
        "\n    println!(\"statements: {}, predicates: {}, modules: {}\");\n",
        lines.clone().count(),
        header.clone().split(",").count(),
        lines.clone().count() / lines_per_module
    );

    let dir_path = "./dialog_lib/src/query/".to_string();
    fs::remove_dir_all(dir_path.clone()).unwrap();
    let _ = fs::create_dir(&dir_path);

    let mut builder = Builder {
        path: path.into(),
        dir_path: dir_path.to_string(),
        module_handles: Vec::new(),
        header,
    };

    //
    let (mut current_module, first_mod) =
        new_module(dir_path.clone(), builder.module_handles.len());
    builder.module_handles.push(first_mod);

    let mut counter = 0;
    while let Some(line) = lines.next() {
        if counter == lines_per_module {
            current_module.write("}\n}".as_bytes()).unwrap();
            counter = 0;
            let (new_bufwriter, new_mod) =
                new_module(dir_path.clone(), builder.module_handles.len());
            current_module = new_bufwriter;
            builder.module_handles.push(new_mod);
        }
        counter += 1;
        let if_string = generate_if(line.to_string());
        current_module.write(if_string.as_bytes()).unwrap();
    }
    current_module.write("\t}\n}".as_bytes()).unwrap();

    // create constant module
    let constants = dir_path.clone() + "mod_constants.rs";
    let constants_file = fs::File::create(&constants).unwrap();
    let mut constant_buf = BufWriter::new(constants_file);

    let header_iter = builder.header.split(",");
    let const_string = header_iter
        .enumerate()
        .map(|(i, s)| format!("pub const {}: usize = {i};", s.to_uppercase()))
        .fold(String::new(), |s, n| s + n.as_str() + "\n");
    constant_buf.write(const_string.as_bytes()).unwrap();

    // Create top module
    let main_mod = fs::File::create(dir_path + "mod.rs").unwrap();
    let mut main_buf = BufWriter::new(main_mod);

    // make module declaration
    main_buf.write("mod mod_constants;\n".as_bytes()).unwrap();
    let module_headers = builder
        .module_handles
        .iter()
        .enumerate()
        .map(|(i, _f)| format!("mod mod_{:#03};", i))
        .fold(String::new(), |f, s| f + &s + "\n");
    main_buf.write((module_headers + "\n").as_bytes()).unwrap();

    // function declaration
    let mod_header = String::from(
        "pub fn query() -> Vec<for<'a, 'b> fn(&'a [i32], &'b mut Vec<&'static str>)> {",
    );
    main_buf.write(mod_header.as_bytes()).unwrap();
    main_buf.write(print_statement.as_bytes()).unwrap();

    main_buf.write("    \nvec![\n".as_bytes()).unwrap();
    let modules_body = builder
        .module_handles
        .iter()
        .enumerate()
        .map(|(i, _f)| format!("        mod_{i:#03}::mod_{i:#03},"))
        .fold(String::new(), |f, s| f + "\n" + &s);

    main_buf.write(modules_body.as_bytes()).unwrap();
    main_buf.write("\n    ]\n}".as_bytes()).unwrap();
}

fn new_module(dir_path: String, n: usize) -> (BufWriter<File>, String) {
    let mod_name = format!("mod_{:#03}", n);
    let new_mod = dir_path.clone() + &mod_name + ".rs";
    let new_file = fs::File::create(&new_mod).unwrap();

    let mut writer = BufWriter::new(new_file);

    let mod_header = format!(
        "use super::mod_constants::*;\npub fn {}(predicates: &[i32], response: &mut Vec<&'static str>) {{\nunsafe {{",
        mod_name
    );
    writer.write(mod_header.as_bytes()).unwrap();
    (writer, new_mod)
}

#[allow(unused)]
fn generate_if(string: String) -> String {
    let mut split_commas = string.split(",");
    assert!(
        split_commas.clone().count() == 3,
        "{string}\nCount was not 3"
    );
    let score = match split_commas.next() {
        Some(s) => s,
        None => {
            println!("{}\nmissing score", string.clone());
            panic!();
        }
    };
    let predicate = match split_commas.next() {
        Some(s) => s,
        None => {
            println!("{}\nmissing predicate", string.clone());
            panic!();
        }
    };
    let dialog_line = match split_commas.next() {
        Some(s) => s,
        None => {
            println!("{}\ndialog_line", string.clone());
            panic!();
        }
    };

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
    statement
}
