from deck_of_cards.deck_of_cards import *

from itertools import combinations

from utils import default_logger
from typing import override


class PKDeck(Deck):
    @override
    def deal(self, no_hands=2, per_hands=2):
        return [
            self[n : per_hands * no_hands : no_hands] for n in range(no_hands)
        ], self[per_hands * no_hands : per_hands * no_hands + 5 :]

    straight_ref = [set(Deck.ranks[i : i + 5 :]) for i in range(0, 9)]
    default_logger.debug(f"straight_ref: {straight_ref}")

    @staticmethod
    def combination_value(combination):
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

        straight = set(ranks) in PKDeck.straight_ref
        mini_straight = set(ranks) == set("A2345")
        flush = len(set(suits)) == 1
        if straight and flush:
            return value(STRAIGHT_FLUSH, values)
        if mini_straight and flush:
            return value(STRAIGHT_FLUSH, values, [-2, -3, -4, -5, -1])
        unique_ranks = len(set(ranks))

        # FoaK or FH
        if unique_ranks == 2:
            if ranks[0] != ranks[1]:
                return value(FOUR_OF_A_KIND, values)
            if ranks[-1] != ranks[-2]:
                return value(FOUR_OF_A_KIND, values, range(5))

            if ranks[1] != ranks[2]:
                return value(FULL_HOUSE, values)
            if ranks[-2] != ranks[-3]:
                return value(FULL_HOUSE, values, range(5))
        if flush:
            return value(FLUSH, values)
        if straight:
            return value(STRAIGHT, values)
        if mini_straight:
            return value(STRAIGHT_FLUSH, values, [-2, -3, -4, -5, -1])

        # ToaK or 2P
        if unique_ranks == 3:
            if ranks[-1] == ranks[-2] and ranks[-2] == ranks[-3]:
                return value(THREE_OF_A_KIND, values)
            if ranks[-2] == ranks[-3] and ranks[-3] == ranks[-4]:
                return value(THREE_OF_A_KIND, values, [-2, -3, -4, -1, -5])
            if ranks[-3] == ranks[-4] and ranks[-5] == ranks[-4]:
                return value(THREE_OF_A_KIND, values, [-3, -4, -5, -1, -2])
            if ranks[0] != ranks[1]:
                return value(TWO_PAIRS, values)
            if ranks[-1] != ranks[-2]:
                return value(TWO_PAIRS, values, [-2, -3, -4, -5, -1])
            if ranks[-2] != ranks[-3] and ranks[1] != ranks[2]:
                return value(TWO_PAIRS, values, [-1, -2, -4, -5, -3])
        # 1P
        if unique_ranks == 4:
            if ranks[-1] != ranks[-2]:
                return value(PAIR, values)
            if ranks[-2] != ranks[-3]:
                return value(PAIR, values, [-2, -3, -1, -4, -5])
            if ranks[-3] != ranks[-4]:
                return value(PAIR, values, [-3, -4, -1, -2, -5])
            if ranks[-4] != ranks[-5]:
                return value(PAIR, values, [-4, -5, -1, -2, -3])

        return value(HIGH_CARD, values)

    @staticmethod
    def hand_value(hand: list[Card]):
        sets = list(combinations(hand, 5))
        result = 0
        best_combination = list()
        for combination in sets:
            combination_value = PKDeck.combination_value(combination)
            if combination_value > result:
                result = combination_value
                best_combination = combination
        return result, best_combination


def demo():
    deck = PKDeck().shuffle()
    default_logger.debug(deck.json_string())

    hands, leftover = deck.deal()

    default_logger.debug(f"Board: {leftover}")
    for i, hand in enumerate(hands):
        hand_value, comb = PKDeck.hand_value(sorted(hand + leftover))
        default_logger.debug(f"Player {i}:" f" {hand} (Best: {comb} @ {hand_value})")
