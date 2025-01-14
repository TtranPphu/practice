from math import isclose
from typing import Callable


def notatest(obj):
    obj.__test__ = False
    return obj


@notatest
def test_equal(cases: list[dict], solution: Callable[..., int | str]):
    for case in cases:
        expect = case.pop("expect")
        assert solution(**case) == expect


@notatest
def test_almost_equal(cases: list[dict], solution: Callable[..., float]):
    for case in cases:
        expect = case.pop("expect")
        assert isclose(solution(**case), expect)


@notatest
def test_ordered_list_equal(cases: list[dict], solution: Callable[..., list]):
    for case in cases:
        expect = case.pop("expect")
        result = solution(**case)
        assert len(result) == len(expect)
        for x, y in zip(result, expect):
            assert x == y


