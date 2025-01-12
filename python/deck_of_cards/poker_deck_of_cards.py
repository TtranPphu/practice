from deck_of_cards.deck_of_cards import Card, Deck

from itertools import combinations

from utils import default_logger
from typing import override, Self


class PKDeck(Deck):
    @override
    def deal(self, no_hands=5, per_hands=2):
        return [
            PKDeck(cards=self[n : per_hands * no_hands : no_hands])
            for n in range(no_hands)
        ], PKDeck(cards=self[per_hands * no_hands : per_hands * no_hands + 5 :])

    def __add__(self, other: Self):
        return PKDeck(cards=self.cards + other.cards)

    __STRAIGHT_REF = [
        set(([str(r) for r in range(2, 11)] + list("JQKA"))[i : i + 5 :])
        for i in range(9)
    ]
    default_logger.debug(f"__STRAIGHT_REF: {__STRAIGHT_REF}")

    @staticmethod
    def combination_value(combination: list[Card]):
        ranks = [card.rank for card in combination]
        suits = [card.suit for card in combination]
        values = [card.value for card in combination]

        def value(set_value, values, indexes=[-1, -2, -3, -4, -5]):
            result = set_value
            for i in indexes:
                result = result * 13 + values[i]
            return result

        STRAIGHT_FLUSH = 8
        FOUR_OF_A_KIND = 7
        FULL_HOUSE = 6
        FLUSH = 5
        STRAIGHT = 4
        THREE_OF_A_KIND = 3
        TWO_PAIRS = 2
        PAIR = 1
        HIGH_CARD = 0

        set_ranks = set(ranks)
        straight = set_ranks in PKDeck.__STRAIGHT_REF
        mini_straight = set_ranks == set("A2345")
        flush = len(set(suits)) == 1
        if straight and flush:
            return value(STRAIGHT_FLUSH, values), (
                "Royal Straight Flush" if "A" in set_ranks else "Straight Flush"
            )
        if mini_straight and flush:
            return (
                value(STRAIGHT_FLUSH, values, [-2, -3, -4, -5, -1]),
                "Mini Straight Flush",
            )
        unique_ranks = len(set(ranks))

        # FoaK or FH
        if unique_ranks == 2:
            if ranks[0] != ranks[1]:
                return value(FOUR_OF_A_KIND, values), "Four of a Kind"
            if ranks[-1] != ranks[-2]:
                return value(FOUR_OF_A_KIND, values, range(5)), "Four of a Kind"

            if ranks[1] != ranks[2]:
                return value(FULL_HOUSE, values), "Full House"
            if ranks[-2] != ranks[-3]:
                return value(FULL_HOUSE, values, range(5)), "Full House"

        if flush:
            return value(FLUSH, values), "Flush"
        if straight:
            return value(STRAIGHT, values), "Straight"
        if mini_straight:
            return (
                value(STRAIGHT_FLUSH, values, [-2, -3, -4, -5, -1]),
                "Mini Straight",
            )

        # ToaK or 2P
        if unique_ranks == 3:
            if ranks[-1] == ranks[-2] and ranks[-2] == ranks[-3]:
                return value(THREE_OF_A_KIND, values), "Three of a Kind"
            if ranks[-2] == ranks[-3] and ranks[-3] == ranks[-4]:
                return (
                    value(THREE_OF_A_KIND, values, [-2, -3, -4, -1, -5]),
                    "Three of a Kind",
                )
            if ranks[-3] == ranks[-4] and ranks[-5] == ranks[-4]:
                return (
                    value(THREE_OF_A_KIND, values, [-3, -4, -5, -1, -2]),
                    "Three of a Kind",
                )
            if ranks[0] != ranks[1]:
                return value(TWO_PAIRS, values), "Two Pairs"
            if ranks[-1] != ranks[-2]:
                return value(TWO_PAIRS, values, [-2, -3, -4, -5, -1]), "Two Pairs"
            if ranks[-2] != ranks[-3] and ranks[1] != ranks[2]:
                return value(TWO_PAIRS, values, [-1, -2, -4, -5, -3]), "Two Pairs"
        # 1P
        if unique_ranks == 4:
            if ranks[-1] != ranks[-2]:
                return value(PAIR, values), "Pair"
            if ranks[-2] != ranks[-3]:
                return value(PAIR, values, [-2, -3, -1, -4, -5]), "Pair"
            if ranks[-3] != ranks[-4]:
                return value(PAIR, values, [-3, -4, -1, -2, -5]), "Pair"
            if ranks[-4] != ranks[-5]:
                return value(PAIR, values, [-4, -5, -1, -2, -3]), "Pair"

        return value(HIGH_CARD, values), "High Card"

    @staticmethod
    def hand_value(hand: list[Card]):
        sets = list(combinations(hand, 5))
        result = 0
        best_combination = list()
        best_set = "High Card"
        for combination in sets:
            combination_value, combination_set = PKDeck.combination_value(combination)
            if combination_value > result:
                result = combination_value
                best_combination = combination
                best_set = combination_set
        return result, best_combination, best_set


def demo():
    deck = PKDeck().build_default_deck().shuffle()
    default_logger.debug(deck.json_string())

    hands, leftover = deck.deal()

    default_logger.debug(f"Board: {sorted(leftover)}")
    for i, hand in enumerate(hands):
        hand_value, best_combination, best_set = PKDeck.hand_value(
            sorted(hand + leftover)
        )
        default_logger.debug(
            f"Player {i}:"
            f" {hand} (Best: {best_combination} -> {best_set} @ {hand_value})"
        )
