
fn return_5() -> u32 {
    return 5;
}

pub fn compute_functional(count: u8) -> u32 {
    let elements = (0..count).collect::<Vec<u8>>().iter()
        .map(|_e| return_5()-1).collect::<Vec<u32>>();
    let value = elements.iter()
        .fold(0, |sum, val| sum * 6 + val);

    value
}

pub fn compute_imperative(count: u8) -> u32 {
    let mut value: u32 = 0;
    for _i in { 0..count } {
        value *= 6;
        value += (return_5()) - 1;
    }
    value
}


pub fn compute_recursive(count: u8) -> u32 {
    compute_recursive_internal(count-1, 0)
}

fn compute_recursive_internal(count: u8, current: u32) -> u32 {
    let new_current = current * 6 + return_5() - 1;
    match count  {
        0 => return new_current,
        _ => compute_recursive_internal(count-1, new_current)
    }
}

#[cfg(test)]
mod computational_test {
    #[test]
    fn compare() {
        for _i in { 0..100 } {
            let functional = super::compute_functional(5);
            let imperative = super::compute_imperative(5);
            let recursive = super::compute_recursive(5);
            assert_eq!(functional, imperative);
            assert_eq!(recursive, imperative);
        }
    }
}
