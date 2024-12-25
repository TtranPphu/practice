#! /usr/bin/python3
from deck_of_cards.deck_of_cards import demo
from utils.log_util import default_logger
import math


class Vector:
    def __init__(self, x=0, y=0):
        self.x = x
        self.y = y

    def __repr__(self):
        return f"Vector({self.x!r}, {self.y!r})"

    def __add__(self, other):
        return Vector(self.x + other.x, self.y + other.y)

    def __mul__(self, scalar):
        return Vector(self.x * scalar, self.y * scalar)

    def __abs__(self):
        return math.hypot(self.x, self.y)

    def __bool__(self):
        return bool(abs(self))


def main():
    v1 = Vector(2, 3)
    v2 = Vector(1, 1)
    v3 = v1 + v2
    default_logger.debug(f"{v3!r}")
    default_logger.debug(f"{abs(v3)}")
    v4 = v3 * 3
    default_logger.debug(f"{v4!r}")
    default_logger.debug(f"{abs(v3*3)}")


if __name__ == "__main__":
    demo()
    # main()
