fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    let mut new_vector = vec![];
    for i in v.iter_mut() {
        new_vector.push(*i * 2);
    }
    return new_vector;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
