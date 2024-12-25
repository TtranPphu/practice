from deck_of_cards.deck_of_cards import *

from itertools import combinations

from utils.log_util import default_logger
import random


class PKDeck(Deck):
    def deal(self, no_hands=5, per_hands=2):
        return [
            (self[n : per_hands * no_hands : no_hands]) for n in range(no_hands)
        ], self[per_hands * no_hands : per_hands * no_hands + 5 :]

    straight_ref = [set(Deck.ranks[i : i + 5 :]) for i in range(0, 9)]
    default_logger.debug(f"straight_ref: {straight_ref}")

    def comb_value(comb):
        ranks = [card.rank for card in comb]
        suits = [card.suit for card in comb]
        values = [card.value for card in comb]
        comb_base_value = 0
        for v in reversed(values):
            comb_base_value *= 13
            comb_base_value += v
        straight = set(ranks) in PKDeck.straight_ref
        flush = len(set(suits)) == 1
        if straight and flush:
            return 8 * (13**5) + comb_base_value
        unique_ranks = len(set(ranks))
        # FoaK or FH
        if unique_ranks == 2:
            if ranks[0] != ranks[1]:
                return 7 * (13**5) + comb_base_value
            if ranks[1] != ranks[2]:
                return 6 * (13**5) + comb_base_value
            rcomb_base_value = 0
            for v in values:
                rcomb_base_value *= 13
                rcomb_base_value += v
            if ranks[-1] != ranks[-2]:
                return 7 * (13**5) + rcomb_base_value
            if ranks[-2] != ranks[-3]:
                return 6 * (13**5) + rcomb_base_value
        if flush:
            return 5 * (13**5) + comb_base_value
        if straight:
            return 4 * (13**5) + comb_base_value
        # ToaK or 2P
        if unique_ranks == 3:
            if ranks[-1] == ranks[-2] and ranks[-2] == ranks[-3]:
                return 3 * (13**5) + comb_base_value
            if ranks[-2] == ranks[-3] and ranks[-3] == ranks[-4]:
                comb_spec_value = 0
                comb_spec_value = comb_base_value * 13 + values[-2]
                comb_spec_value = comb_base_value * 13 + values[-3]
                comb_spec_value = comb_base_value * 13 + values[-4]
                comb_spec_value = comb_base_value * 13 + values[-1]
                comb_spec_value = comb_base_value * 13 + values[-5]
                return 3 * (13**5) + comb_spec_value
            if ranks[-3] == ranks[-4] and ranks[-5] == ranks[-4]:
                comb_spec_value = 0
                comb_spec_value = comb_base_value * 13 + values[-3]
                comb_spec_value = comb_base_value * 13 + values[-4]
                comb_spec_value = comb_base_value * 13 + values[-5]
                comb_spec_value = comb_base_value * 13 + values[-1]
                comb_spec_value = comb_base_value * 13 + values[-2]
                return 3 * (13**5) + comb_spec_value
            if ranks[0] != ranks[1]:
                return 2 * (13**5) + comb_base_value
            if ranks[-1] != ranks[-2]:
                comb_spec_value = 0
                comb_spec_value = comb_base_value * 13 + values[-2]
                comb_spec_value = comb_base_value * 13 + values[-3]
                comb_spec_value = comb_base_value * 13 + values[-4]
                comb_spec_value = comb_base_value * 13 + values[-5]
                comb_spec_value = comb_base_value * 13 + values[-1]
                return 2 * (13**5) + comb_spec_value
            if ranks[-2] != ranks[-3] and ranks[1] != ranks[2]:
                comb_spec_value = 0
                comb_spec_value = comb_base_value * 13 + values[-1]
                comb_spec_value = comb_base_value * 13 + values[-2]
                comb_spec_value = comb_base_value * 13 + values[-4]
                comb_spec_value = comb_base_value * 13 + values[-5]
                comb_spec_value = comb_base_value * 13 + values[-3]
                return 2 * (13**5) + comb_spec_value
        if unique_ranks == 4:
            return 1 * (13**5) + comb_base_value
        return comb_base_value

    @staticmethod
    def hand_value(hand: list[Card]):
        combs = list(combinations(hand, 5))
        result = 0
        best_comb = list()
        for comb in combs:
            comb_value = PKDeck.comb_value(comb)
            if comb_value > result:
                result = comb_value
                best_comb = comb
        return result, comb


def demo():
    deck = PKDeck()
    random.shuffle(deck)
    default_logger.debug(f"Deck: {deck}")

    hands, leftover = deck.deal()

    default_logger.debug(f"Board: {leftover}")
    for i, hand in enumerate(hands):
        hand_value, comb = PKDeck.hand_value(sorted(hand+leftover))
        default_logger.debug(
            f"Player {i} hand:"
            f" {sorted(hand)} (Best comb: {comb}, {hand_value} in value)"
        )
