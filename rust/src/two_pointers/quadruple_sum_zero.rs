fn quadruple_sum_zero(mut input: Vec<i32>) -> Vec<Vec<i32>> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_unique_quadruple_sum_zero() {
        let test_cases = [
            (
                vec![4, 1, 2, -1, 1, -3],
                vec![vec![-3, -1, 1, 4], vec![-3, 1, 1, 2]],
            ),
            (
                vec![2, 0, -1, 1, -2, 2],
                vec![vec![-2, 0, 2, 2], vec![-1, 0, 1, 2]],
            ),
        ];

        for (input, expected_result) in test_cases {
            println!("Running test for {:?}", input);
            let result = quadruple_sum_zero(input);
            assert_eq!(expected_result.len(), result.len());
            assert_eq!(expected_result, normalize(result));
        }
    }

    fn normalize(mut data: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for inner in data.iter_mut() {
            inner.sort();
        }

        data.sort();

        data
    }
}
