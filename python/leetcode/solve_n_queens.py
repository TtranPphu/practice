class Solution:
    def solveNQueens(self, n: int) -> list[list[str]]:
        results: list[list[int]] = []

        def valid(result: list[int], row: int, col: int) -> bool:
            for i, j in enumerate(result):
                if j == col or abs(i - row) == abs(j - col):
                    return False
            return True

        def next(current: list[int], row: int) -> list[list[int]]:
            if row == n:
                results.append(current)
                return
            valid_col = [col for col in range(n) if valid(current, row, col)]
            for col in valid_col:
                next(current + [col], row + 1)

        next([], 0)

        str_results = [
            ["." * i + "Q" + "." * (n - i - 1) for i in result] for result in results
        ]
        return str_results
