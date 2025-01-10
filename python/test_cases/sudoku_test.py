import unittest
from sudoku.sudoku import Grid as Solver


class SudokuTest(unittest.TestCase):
    def test_expert_sudoku(self):
        self.expert = [
            [9, 0, 0, 0, 7, 0, 0, 2, 0],
            [0, 8, 0, 0, 6, 0, 0, 0, 3],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [4, 0, 0, 0, 0, 0, 0, 0, 1],
            [2, 0, 0, 0, 4, 6, 3, 0, 0],
            [6, 0, 0, 0, 9, 0, 0, 0, 8],
            [0, 5, 0, 0, 3, 0, 0, 0, 7],
            [0, 0, 0, 4, 0, 1, 0, 0, 0],
            [0, 9, 0, 7, 0, 0, 1, 0, 0],
        ]
        self.solver = Solver(self.expert)
        self.solver.solve()
        self.assertListEqual(
            self.solver.grid,
            [
                [9, 4, 1, 8, 7, 3, 6, 2, 5],
                [5, 8, 7, 2, 6, 4, 9, 1, 3],
                [3, 2, 6, 9, 1, 5, 8, 7, 4],
                [4, 3, 9, 5, 2, 8, 7, 6, 1],
                [2, 7, 8, 1, 4, 6, 3, 5, 9],
                [6, 1, 5, 3, 9, 7, 2, 4, 8],
                [1, 5, 2, 6, 3, 9, 4, 8, 7],
                [7, 6, 3, 4, 8, 1, 5, 9, 2],
                [8, 9, 4, 7, 5, 2, 1, 3, 6],
            ],
        )

    def test_extreme_sudoku(self):
        self.expert = [
            [2, 0, 4, 0, 0, 0, 0, 0, 0],
            [1, 0, 7, 0, 9, 0, 0, 0, 2],
            [0, 0, 0, 0, 0, 0, 7, 0, 0],
            [6, 0, 0, 0, 0, 1, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 6, 0, 3],
            [8, 0, 1, 0, 4, 9, 0, 0, 0],
            [4, 0, 0, 0, 6, 0, 0, 7, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 9],
            [0, 0, 0, 3, 0, 0, 5, 6, 0],
        ]
        self.solver = Solver(self.expert)
        self.solver.solve()
        self.assertListEqual(
            self.solver.grid,
            [
                [2, 8, 4, 7, 5, 3, 1, 9, 6],
                [1, 5, 7, 4, 9, 6, 8, 3, 2],
                [3, 9, 6, 2, 1, 8, 7, 4, 5],
                [6, 7, 2, 5, 3, 1, 9, 8, 4],
                [5, 4, 9, 8, 2, 7, 6, 1, 3],
                [8, 3, 1, 6, 4, 9, 2, 5, 7],
                [4, 1, 5, 9, 6, 2, 3, 7, 8],
                [7, 6, 3, 1, 8, 5, 4, 2, 9],
                [9, 2, 8, 3, 7, 4, 5, 6, 1],
            ],
        )
