use common::*;

pub fn compute_functional(count: u32) -> u32 {
    (0..count)
        .map(|_e| return_5()-1)
        .fold(0, |sum, val| sum * 6 + val)
}

pub fn compute_functional_opt(count: u32) -> u32 {
    (0..count)
        .map(|_e| return_5()-1)
        .fold(0, |sum, val| (sum << 2) + (sum <<1) + val)
}

pub fn compute_imperative(count: u32) -> u32 {
    let mut value: u32 = 0;
    for _i in { 0..count } {
        value *= 6;
        value += (return_5()) - 1;
    }
    value
}

pub fn compute_imperative_opt(count: u32) -> u32 {
    let mut value: u32 = 0;
    for _i in { 0..count } {
        value = (value <<2) + (value << 1);
        value += (return_5()) - 1;
    }
    value
}

pub fn compute_recursive(count: u32) -> u32 {
    compute_recursive_internal(count-1, 0)
}

fn compute_recursive_internal(count: u32, current: u32) -> u32 {
    let new_current = current * 6 + return_5() -1;
    match count  {
        0 => return new_current,
        _ => compute_recursive_internal(count-1, new_current)
    }
}

pub fn compute_recursive_opt(count: u32) -> u32 {
    compute_recursive_internal_opt(count-1, 0)
}

fn compute_recursive_internal_opt(count: u32, current: u32) -> u32 {
    let new_current = (current << 2) + (current << 1)
        + return_5() - 1;
    match count  {
        0 => return new_current,
        _ => compute_recursive_internal_opt(count-1, new_current)
    }
}


#[cfg(test)]
mod computational_test {
    #[test]
    fn compare() {
        for _i in { 0..100 } {
            let functional = super::compute_functional(5);
            let functional_opt = super::compute_functional(5);
            let imperative = super::compute_imperative(5);
            let imperative_opt = super::compute_imperative_opt(5);
            let recursive = super::compute_recursive(5);
            let recursive_opt = super::compute_recursive(5);
            assert_eq!(functional, imperative);
            assert_eq!(functional, functional_opt);
            assert_eq!(recursive, imperative);
            assert_eq!(recursive, recursive_opt);
            assert_eq!(imperative, imperative_opt);
        }
    }
}

