mod mod_constants;
mod mod_000;
mod mod_001;
mod mod_002;
mod mod_003;
mod mod_004;
mod mod_005;
mod mod_006;
mod mod_007;
mod mod_008;
mod mod_009;

pub fn query(predicates: &[i32], response: &mut Vec<&'static str>) {
    println!("statements: 1000, predicates: 25, modules: 10");

    mod_000::mod_000(predicates, response);
    mod_001::mod_001(predicates, response);
    mod_002::mod_002(predicates, response);
    mod_003::mod_003(predicates, response);
    mod_004::mod_004(predicates, response);
    mod_005::mod_005(predicates, response);
    mod_006::mod_006(predicates, response);
    mod_007::mod_007(predicates, response);
    mod_008::mod_008(predicates, response);
    mod_009::mod_009(predicates, response);
}