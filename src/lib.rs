
pub fn build_vector_imperative(size: u32) -> Vec<u32> {
    let mut v = Vec::with_capacity(size as usize);
    for _i in { 0..size } {
        v.push(self::return_5());
    }

    v
}

pub fn build_vector_functional(size: u32) -> Vec<u32> {
    let v = (0..size).map(|_e| return_5());
    v.collect::<Vec<u32>>()
}

fn build_vector_recursive_internal(size: u32, v: Vec<u32>) -> Vec<u32>  {
    let new_vector = [v, vec!(self::return_5())].concat();
    match size {
        0 => new_vector,
        _ => build_vector_recursive_internal(size-1, new_vector)
    }
}

pub fn build_vector_recursive(size: u32)  -> Vec<u32> {
   build_vector_recursive_internal(size-1, Vec::new())
}

fn return_5() -> u32 {
    return 5;
}

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

#[cfg(test)]
mod build_vector_test {
    #[test]
    fn compare_vectors() {
        let vector_f = super::build_vector_functional(100);
        let vector_i = super::build_vector_imperative(100);
        let vector_r = super::build_vector_recursive(100);
        assert_eq!(vector_f, vector_i);
        assert_eq!(vector_i, vector_r);
    }
}
