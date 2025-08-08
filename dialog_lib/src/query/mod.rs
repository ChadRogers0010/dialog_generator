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
mod mod_010;
mod mod_011;
mod mod_012;
mod mod_013;
mod mod_014;
mod mod_015;
mod mod_016;
mod mod_017;
mod mod_018;
mod mod_019;
mod mod_020;
mod mod_021;
mod mod_022;
mod mod_023;
mod mod_024;

pub fn query() -> Vec<for<'a, 'b> fn(&'a [i32], &'b mut Vec<&'static str>)> {
    println!("statements: 2500, predicates: 50, modules: 25");
    
vec![

        mod_000::mod_000,
        mod_001::mod_001,
        mod_002::mod_002,
        mod_003::mod_003,
        mod_004::mod_004,
        mod_005::mod_005,
        mod_006::mod_006,
        mod_007::mod_007,
        mod_008::mod_008,
        mod_009::mod_009,
        mod_010::mod_010,
        mod_011::mod_011,
        mod_012::mod_012,
        mod_013::mod_013,
        mod_014::mod_014,
        mod_015::mod_015,
        mod_016::mod_016,
        mod_017::mod_017,
        mod_018::mod_018,
        mod_019::mod_019,
        mod_020::mod_020,
        mod_021::mod_021,
        mod_022::mod_022,
        mod_023::mod_023,
        mod_024::mod_024,
    ]
}