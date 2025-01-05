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
        self.__grid = [[Cell(value) for value in row] for row in grid]
        for i in range(9):
            for j in range(9):
                if self.__grid[i][j].value:
                    self.__update_valid(i, j)

    def __update_valid(self, i, j):
        """
        Remove the value of the cell from the valid set of the other cells in the same
        row, column and mini grid
        """
        v = self.__grid[i][j].value
        self.__grid[i][j].valid.clear()
        for k in range(9):
            self.__grid[i][k].valid.discard(v)
            self.__grid[k][j].valid.discard(v)
        for k in range(3):
            for l in range(3):
                self.__grid[(i // 3) * 3 + k][(j // 3) * 3 + l].valid.discard(v)

    def __set_value(self, i, j, v):
        self.__grid[i][j].value = v
        self.__update_valid(i, j)

    def __check_unique(self, v, i, j):
        """Check if the value v is uniquely valid in the row, column and mini grid"""
        return (
            all(v not in self.__grid[i][k].valid for k in range(9) if k != j)
            or all(v not in self.__grid[k][j].valid for k in range(9) if k != i)
            or all(
                v not in self.__grid[(i // 3) * 3 + k][(j // 3) * 3 + l].valid
                for k in range(3)
                for l in range(3)
                if k != i % 3 or l != j % 3
            )
        )

    def __solved(self):
        return all(all(cell.value for cell in row) for row in self.__grid)

    def __apply_basic_strategies(self):
        updated = False
        for i in range(9):
            for j in range(9):
                if self.__grid[i][j].value == 0:
                    if len(self.__grid[i][j].valid) == 1:
                        self.__set_value(i, j, self.__grid[i][j].valid.pop())
                        updated = True
                    else:
                        for v in set(self.__grid[i][j].valid):
                            if self.__check_unique(v, i, j):
                                self.__set_value(i, j, v)
                                updated = True
                                break
        return updated

    def __brute_force(self):
        for i in range(9):
            for j in range(9):
                if self.__grid[i][j].value == 0:
                    for v in self.__grid[i][j].valid:
                        grid = Grid(
                            [[cell.value for cell in row] for row in self.__grid]
                        )
                        grid.__set_value(i, j, v)
                        grid.solve()
                        if grid.__solved():
                            self.__grid = grid.__grid
                            return
                    raise ValueError("Unsolvable sodoku")

    def solve(self):
        while self.__apply_basic_strategies():
            pass
        if not self.__solved():
            self.__brute_force()

    def __str__(self):
        return "\n".join(
            " ".join(str(cell.value) for cell in row) for row in self.__grid
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
