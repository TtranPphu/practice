mod sudoku {
    #![allow(dead_code)]

    #[test]
    fn test_solve() {
        use practice::sudoku::solve;
        use std::collections::HashMap;
        let mut expert = HashMap::new();
        expert.insert(
            "problem",
            vec![
                vec![9, 0, 0, 0, 7, 0, 0, 2, 0],
                vec![0, 8, 0, 0, 6, 0, 0, 0, 3],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![4, 0, 0, 0, 0, 0, 0, 0, 1],
                vec![2, 0, 0, 0, 4, 6, 3, 0, 0],
                vec![6, 0, 0, 0, 9, 0, 0, 0, 8],
                vec![0, 5, 0, 0, 3, 0, 0, 0, 7],
                vec![0, 0, 0, 4, 0, 1, 0, 0, 0],
                vec![0, 9, 0, 7, 0, 0, 1, 0, 0],
            ],
        );
        expert.insert(
            "expected",
            vec![
                vec![9, 4, 1, 8, 7, 3, 6, 2, 5],
                vec![5, 8, 7, 2, 6, 4, 9, 1, 3],
                vec![3, 2, 6, 9, 1, 5, 8, 7, 4],
                vec![4, 3, 9, 5, 2, 8, 7, 6, 1],
                vec![2, 7, 8, 1, 4, 6, 3, 5, 9],
                vec![6, 1, 5, 3, 9, 7, 2, 4, 8],
                vec![1, 5, 2, 6, 3, 9, 4, 8, 7],
                vec![7, 6, 3, 4, 8, 1, 5, 9, 2],
                vec![8, 9, 4, 7, 5, 2, 1, 3, 6],
            ],
        );
        let mut extreme = HashMap::new();
        extreme.insert(
            "problem",
            vec![
                vec![2, 0, 4, 0, 0, 0, 0, 0, 0],
                vec![1, 0, 7, 0, 9, 0, 0, 0, 2],
                vec![0, 0, 0, 0, 0, 0, 7, 0, 0],
                vec![6, 0, 0, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 0, 0, 2, 0, 6, 0, 3],
                vec![8, 0, 1, 0, 4, 9, 0, 0, 0],
                vec![4, 0, 0, 0, 6, 0, 0, 7, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 9],
                vec![0, 0, 0, 3, 0, 0, 5, 6, 0],
            ],
        );
        extreme.insert(
            "expected",
            vec![
                vec![2, 8, 4, 7, 5, 3, 1, 9, 6],
                vec![1, 5, 7, 4, 9, 6, 8, 3, 2],
                vec![3, 9, 6, 2, 1, 8, 7, 4, 5],
                vec![6, 7, 2, 5, 3, 1, 9, 8, 4],
                vec![5, 4, 9, 8, 2, 7, 6, 1, 3],
                vec![8, 3, 1, 6, 4, 9, 2, 5, 7],
                vec![4, 1, 5, 9, 6, 2, 3, 7, 8],
                vec![7, 6, 3, 1, 8, 5, 4, 2, 9],
                vec![9, 2, 8, 3, 7, 4, 5, 6, 1],
            ],
        );
        let tests = vec![expert, extreme];
        for test in tests {
            let problem = test.get("problem").unwrap();
            let expected = test.get("expected").unwrap();
            match solve(problem.clone()) {
                Ok(grid) => assert_eq!(grid.result(), *expected),
                Err(_) => panic!("Unsolvable!"),
            }
        }
    }
}
