from utils import default_logger
class Cell:

    def __init__(self, value):
        self.value = value
        if value != 0:
            self.valid = set()
        else:
            self.valid = set(range(1, 10))


class Grid:

    def __init__(self, grid):
        self._grid = [[Cell(value) for value in row] for row in grid]
        for i in range(9):
            for j in range(9):
                if self._grid[i][j].value:
                    self._update_valid(i, j)

    def _update_valid(self, i, j):
        v = self._grid[i][j].value
        for k in range(9):
            self._grid[i][k].valid.discard(v)
            self._grid[k][j].valid.discard(v)
        for k in range(3):
            for l in range(3):
                self._grid[(i // 3) * 3 + k][(j // 3) * 3 + l].valid.discard(v)

    def solve(self):
        while True:
            updated = False
            for i in range(9):
                for j in range(9):
                    if self._grid[i][j].value == 0:
                        if len(self._grid[i][j].valid) == 1:
                            self._grid[i][j].value = self._grid[i][j].valid.pop()
                            self._update_valid(i, j)
                            updated = True
            if not updated:
                break

    def __str__(self):
        return "\n".join(
            " ".join(str(cell.value) for cell in row) for row in self._grid
        )

def demo():
    problem = Grid(
        [
            [2, 5, 0, 0, 3, 0, 9, 0, 1],
            [0, 1, 0, 0, 0, 4, 0, 0, 0],
            [4, 0, 7, 0, 0, 0, 2, 0, 8],
            [0, 0, 5, 2, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 9, 8, 1, 0, 0],
            [0, 4, 0, 0, 0, 3, 0, 0, 0],
            [0, 0, 0, 3, 6, 0, 0, 7, 2],
            [0, 7, 0, 0, 0, 0, 0, 0, 3],
            [9, 0, 3, 0, 0, 0, 6, 0, 4],
        ]
    )
    problem.solve()
    default_logger.debug(f"\n{problem}\n")