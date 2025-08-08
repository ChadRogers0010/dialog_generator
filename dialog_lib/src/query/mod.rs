mod mod_000;
mod mod_001;
mod mod_002;
mod mod_003;
mod mod_004;
mod mod_constants;

pub fn query() -> Vec<for<'a, 'b> fn(&'a [i32], &'b mut Vec<&'static str>)> {
    println!("statements: 5000, predicates: 50, modules: 5");

    vec![
        mod_000::mod_000,
        mod_001::mod_001,
        mod_002::mod_002,
        mod_003::mod_003,
        mod_004::mod_004,
    ]
}