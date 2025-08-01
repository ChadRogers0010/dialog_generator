#[allow(unused)]
pub fn query(predicates: &[i32], response: &mut Vec<&'static str>) -> Option<&'static str> {
    println!("predicates: 20\nstatements: 100");
    if response.len() == 0 {
        return None;
    }
    let rand_idx = rand::random_range(0..response.len());
    Some(response[rand_idx])
}

