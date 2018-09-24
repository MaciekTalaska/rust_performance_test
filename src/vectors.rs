use common::*;

pub fn build_vector_imperative(size: u32) -> Vec<u32> {
    let mut vector = Vec::with_capacity(size as usize);
    for _i in { 0..size } {
        vector.push(self::return_5());
    }

    vector
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

#[cfg(test)]
mod build_vector_test {
    const VECTOR_SIZE: u32 = 100;

    #[test]
    fn compare_vectors() {
        let vector_f = super::build_vector_functional(VECTOR_SIZE);
        let vector_i = super::build_vector_imperative(VECTOR_SIZE);
        let vector_r = super::build_vector_recursive(VECTOR_SIZE);
        assert_eq!(vector_f, vector_i);
        assert_eq!(vector_i, vector_r);
    }
}
