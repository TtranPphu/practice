from interview import *
from utils import default_logger


def test_sentenses():
    return
    assert set(sentenses("catsanddog", {"cat", "cats", "and", "sand", "dog"})) == set(
        [
            "cats and dog",
            "cat sand dog",
        ]
    )
    assert set(
        sentenses(
            "pineapplepenapple", {"apple", "pen", "applepen", "pine", "pineapple"}
        )
    ) == set(
        [
            "pine apple pen apple",
            "pineapple pen apple",
            "pine applepen apple",
        ]
    )
