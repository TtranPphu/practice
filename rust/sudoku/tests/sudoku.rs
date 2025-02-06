#[test]
fn solve() {
    let tests = [
        (
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
        ),
        (
            vec![
                vec![0, 6, 0, 0, 0, 3, 0, 0, 7],
                vec![3, 0, 0, 6, 8, 0, 0, 1, 0],
                vec![1, 9, 0, 2, 0, 0, 0, 0, 0],
                vec![6, 8, 5, 0, 0, 0, 1, 3, 0],
                vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![2, 1, 0, 0, 0, 0, 0, 0, 0],
                vec![4, 0, 3, 0, 0, 0, 0, 0, 6],
                vec![0, 0, 0, 0, 2, 0, 0, 0, 9],
                vec![0, 0, 0, 0, 4, 0, 8, 7, 0],
            ],
            vec![
                vec![5, 6, 8, 9, 1, 3, 4, 2, 7],
                vec![3, 4, 2, 6, 8, 7, 9, 1, 5],
                vec![1, 9, 7, 2, 5, 4, 6, 8, 3],
                vec![6, 8, 5, 4, 7, 9, 1, 3, 2],
                vec![7, 3, 4, 1, 6, 2, 5, 9, 8],
                vec![2, 1, 9, 5, 3, 8, 7, 6, 4],
                vec![4, 7, 3, 8, 9, 1, 2, 5, 6],
                vec![8, 5, 1, 7, 2, 6, 3, 4, 9],
                vec![9, 2, 6, 3, 4, 5, 8, 7, 1],
            ],
        ),
        (
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
        ),
    ];
    for (problem, expected) in tests {
        assert_eq!(
            sudoku::solve(problem.clone()).unwrap(),
            expected,
            "Problem:\n{:?} Expected:\n{:?}",
            problem,
            expected
        );
    }
}

#[test]
fn free() {
    let _ = sudoku::solve(vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0],
    ])
    .unwrap();
}
