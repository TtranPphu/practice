from deck_of_cards.deck_of_cards import Deck

from utils import default_logger
from typing import override


class BJDeck(Deck):
    @override
    def deal(self, no_hands=6, per_hands=2):
        return [
            BJDeck(cards=self[n : per_hands * no_hands : no_hands])
            for n in range(no_hands)
        ], BJDeck(cards=self[-1 : per_hands * no_hands - 1 : -1])


def demo():
    deck = BJDeck().build_default_deck().shuffle()
    default_logger.debug(deck.json_string())
    hands, leftover = deck.deal()

    for i, hand in enumerate(hands):
        default_logger.debug(hand.json_string(f"Player {i}"))
    default_logger.debug(leftover.json_string("Draw"))
