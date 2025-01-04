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
        self._grid[i][j].valid.clear()
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
                        else:
                            for v in set(self._grid[i][j].valid):
                                if all(
                                    v not in self._grid[i][k].valid
                                    for k in range(9)
                                    if k != j
                                ):
                                    self._grid[i][j].value = v
                                    self._update_valid(i, j)
                                    updated = True
                                    break
                                if all(
                                    v not in self._grid[k][j].valid
                                    for k in range(9)
                                    if k != i
                                ):
                                    self._grid[i][j].value = v
                                    self._update_valid(i, j)
                                    updated = True
                                    break
                                if all(
                                    v
                                    not in self._grid[(i // 3) * 3 + k][
                                        (j // 3) * 3 + l
                                    ].valid
                                    for k in range(3)
                                    for l in range(3)
                                    if k != i % 3 or l != j % 3
                                ):
                                    self._grid[i][j].value = v
                                    self._update_valid(i, j)
                                    updated = True
                                    break
            if not updated:
                break

    def __str__(self):
        return "\n".join(
            " ".join(str(cell.value) for cell in row) for row in self._grid
        )


def demo():
    problem = Grid(
        [
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
        ]
    )
    problem.solve()
    default_logger.debug(f"\n{problem}\n")
