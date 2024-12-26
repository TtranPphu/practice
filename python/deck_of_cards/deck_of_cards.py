from abc import ABC, abstractmethod

from utils.log_util import default_logger
import json

import random


class Card:
    def __init__(self, rank_suit, value):
        self.rank = rank_suit[:-1]
        self.suit = rank_suit[-1]
        self.value = value

    def __repr__(self):
        return f"{(self.rank+self.suit)!r}"

    def __str__(self):
        return f"{self.rank+self.suit}"

    def __eq__(self, other):
        return self.value == other.value

    def __lt__(self, other):
        return self.value < other.value


class Deck(ABC):
    ranks = [str(r) for r in range(2, 11)] + list("JQKA")
    suits = list("♠♣♦♥")
    # suits = list("SCDH")
    _card: list[Card]

    def __init__(self, cards: list[Card] | None = None):
        if cards:
            self._cards = cards
        else:
            self._cards = [
                Card(
                    rank + suit,
                    self.ranks.index(rank),
                )
                for rank in self.ranks
                for suit in self.suits
            ]

    def __len__(self):
        return len(self._cards)

    def __getitem__(self, position):
        return self._cards[position]

    def __setitem__(self, position, card):
        self._cards[position] = card

    def __repr__(self):
        return f"{self._cards!r}"

    def __len__(self):
        return len(self._cards)

    def json(self, name="Deck"):
        return {name: [str(card) for card in self._cards]}

    def json_string(self, name="Deck"):
        return json.dumps(self.json(name), ensure_ascii=False)

    def shuffle(self):
        seed = random.random()
        default_logger.debug(json.dumps({"Seed": seed}))
        random.shuffle(self._cards, lambda: seed)
        random.shuffle(self._cards, lambda: seed)
        random.shuffle(self._cards, lambda: seed)
        return self

    def deal(self, no_hands, per_hands):
        return [
            Deck(self[n : per_hands * no_hands : no_hands]) for n in range(no_hands)
        ], Deck(self[per_hands * no_hands : :])


class CTDeck(Deck):
    def deal(self, no_hands=4, per_hands=6):
        return super().deal(no_hands, per_hands)


class VCDeck(Deck):
    ranks = [str(r) for r in range(3, 11)] + list("JQKA2")

    def __init__(self):
        self._cards = [
            Card(
                rank + suit,
                self.ranks.index(rank) * len(self.suits) + self.suits.index(suit),
            )
            for rank in self.ranks
            for suit in self.suits
        ]

    def deal(self, no_hands=4, per_hands=13):
        return super().deal(no_hands, per_hands)


def demo():
    deck = CTDeck().shuffle()
    default_logger.debug(deck.json_string())

    hands, leftover = deck.deal()

    for i, hand in enumerate(hands):
        default_logger.debug(f"Player {i} hand: {sorted(hand)}")
    default_logger.debug(f"Leftover: {leftover}")
