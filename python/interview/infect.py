from utils import default_logger


class Solution:
    def infect(
        self,
        n: int,
        initially_infected: set[int],
        interactions: list[tuple[int, int, int | None]],
    ) -> list[int]:
        infected = [-1 if x not in initially_infected else 0 for x in range(n)]  # O(n)
        immunes = set()

        def is_infected(p):
            return infected[p] != -1

        for t, (type, p1, *p2) in enumerate(interactions):  # O(n*log(n))
            match type:
                case 0:
                    assert p2[0] is not None
                    p2 = p2[0]
                    if (
                        is_infected(p1) and not is_infected(p2) and p2 not in immunes
                    ):  # O(log(n))
                        infected[p2] = t + 1
                    elif (
                        is_infected(p2) and not is_infected(p1) and p1 not in immunes
                    ):  # O(log(n))
                        infected[p1] = t + 1
                case 1:
                    immunes.add(p1)  # O(log(n))
        return infected
