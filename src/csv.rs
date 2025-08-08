use crate::utils::intersperse;
use std::{
    fs,
    io::{BufWriter, Write},
};

pub fn create_test_csv(predciates: u32, statements: u32) {
    let params = (0..predciates).map(|n| format!("param_{n:#02}"));

    let header: String = intersperse(params.clone(), ",") + "\n";

    let radix = 20;
    let mut radix_array = crate::radix_array::RadixArray::new(radix, predciates as i32);
    let param_array = params.clone().collect::<Vec<String>>();

    let gen_range = || {
        let a = rand::random_range(0..=40);
        let b = rand::random_range(60..100);
        (a, b)
    };

    let mut count = 0;

    let test_csv = fs::File::create("./test.csv").unwrap();
    let mut bufwrite = BufWriter::new(test_csv);
    bufwrite.write(header.as_bytes()).unwrap();
    for _ in 0..statements {
        let line = {
            let iter = param_array.iter().map(|p| {
                let (min, max) = gen_range();
                format!("{p} > {min} && {p} <= {max}")
            });
            let predicate = intersperse(iter, " && ");

            let result = radix_array.format_array();
            radix_array.add(1);

            count.to_string() + "," + predicate.as_str() + "," + result.as_str() + "\n"
        };
        count += 1;
        bufwrite.write(line.as_bytes()).unwrap();
    }
}
