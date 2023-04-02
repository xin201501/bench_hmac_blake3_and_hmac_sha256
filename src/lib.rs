use rand::prelude::*;

// struct random_generator{
//     rng:ThreadRng
// }
// impl

pub fn generate_object_collections<const N: usize>() -> [i32; N] {
    let mut rng = rand::thread_rng();
    let mut result = [0; N];
    for i in 0..N {
        result[i] = rng.gen();
    }
    result
}

pub fn mock_generate_string<const N: usize>(n: usize) -> Vec<Vec<String>> {
    let mut counter = 1;
    let mut result = Vec::with_capacity(N);
    for _ in 0..N {
        let mut vec = Vec::with_capacity(n);
        for _ in 0..n {
            vec.push(counter.to_string());
            counter += 1;
        }
        result.push(vec);
    }
    result
}
pub fn mock_generate_int<const N: usize>(n: usize) -> Vec<Vec<i32>> {
    let mut counter = 1;
    let mut result = Vec::with_capacity(N);
    for _ in 0..N {
        let mut vec = Vec::with_capacity(n);
        for _ in 0..n {
            vec.push(counter);
            counter += 1;
        }
        result.push(vec);
    }
    result
}
