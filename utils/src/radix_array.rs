use super::intersperse;

#[derive(Debug)]
pub struct RadixArray {
    pub radix: i32,
    pub array: Vec<i32>,
}

#[allow(unused)]
impl RadixArray {
    pub fn new(radix: i32, indicies: i32) -> Self {
        let mut vec = Vec::with_capacity(indicies as usize);
        for _ in 0..indicies {
            vec.push(0);
        }
        Self { radix, array: vec }
    }

    pub fn init(radix: i32, indicies: i32, value: i32) -> Self {
        let mut vec = Vec::with_capacity(indicies as usize);
        for _ in 0..indicies {
            vec.push(value);
        }
        Self { radix, array: vec }
    }

    pub fn is_maxed(&self) -> bool {
        for i in self.array.iter() {
            if *i != self.radix - 1 {
                return false;
            }
        }
        true
    }

    pub fn add(&mut self, rhs: i32) -> bool {
        let mut overflow = 0;

        let mut iter = self.array.iter_mut().rev();

        {
            let first = iter.next().unwrap();
            *first += rhs;
            overflow = *first / self.radix;
            *first %= self.radix;
        }

        for i in iter {
            *i += overflow;
            overflow = *i / self.radix;
            *i %= self.radix;

            if overflow == 0 {
                break;
            }
        }

        overflow > 0
    }

    fn increment_start(&mut self) -> bool {
        false
    }

    pub fn format_array(&self) -> String {
        intersperse(self.array.iter().map(|i| i.to_string()), " ")
    }
}

#[test]
fn radix_is_maxed() {
    let maxed = RadixArray::init(5, 5, 4);
    assert!(maxed.is_maxed());

    let min = RadixArray::init(5, 5, 0);
    assert!(!min.is_maxed());
}
#[test]
fn radix_add() {
    let mut radix_array = RadixArray::new(5, 2);
    radix_array.add(5);
    assert!(radix_array.array[0] == 1);
    radix_array.add(2);
    assert!(radix_array.array[1] == 2);
    radix_array.add(3);
    assert!(radix_array.array[0] == 2);
    assert!(radix_array.array[1] == 0);
}
