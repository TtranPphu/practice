from abc import ABC, abstractmethod
from typing import Callable, Self
from copy import deepcopy

from utils import default_logger
import json

import random


class Card:
    def __init__(self, rank_suit: str, value: int):
        self.rank = rank_suit[:-1]
        self.suit = rank_suit[-1]
        self.value = value

    def __repr__(self):
        return f"Card({(self.rank+self.suit)!r})"

    def __str__(self):
        return f"{(self.rank+self.suit)!s}"

    def __eq__(self, other):
        return self.value == other.value

    def __lt__(self, other):
        return self.value < other.value


class Deck:
    @property
    def ranks(self):
        return self.__ranks

    @property
    def suits(self):
        return self.__suits

    @property
    def cards(self):
        return self.__cards

    @cards.setter
    def cards(self, cards):
        self.__cards = cards

    def __init__(self, ranks=None, suits=None, cards=None):
        self.__ranks = ranks
        self.__suits = suits
        self.__cards = deepcopy(cards) if cards else []

    def set_ranks(
        self, ranks: list[str] = [str(r) for r in range(2, 11)] + list("JQKA")
    ):
        self.__ranks = ranks
        return self

    def set_suits(self, suits: list[str] = list("♠♣♦♥")):
        self.__suits = suits
        return self

    @staticmethod
    def default_value_fn(rank, suit, ranks, suits):
        return ranks.index(rank)

    def build_deck(
        self,
        value_fn: Callable[[str, str, list[str], list[str]], int] = default_value_fn,
    ):
        self.__cards = [
            Card(
                rank_suit=rank + suit,
                value=value_fn(rank, suit, self.ranks, self.suits),
            )
            for rank in self.ranks
            for suit in self.suits
        ]
        return self

    def build_default_deck(self):
        return self.set_ranks().set_suits().build_deck()

    def __len__(self):
        return len(self.__cards)

    def __getitem__(self, position):
        return self.__cards[position]

    def __setitem__(self, position, card):
        self.__cards[position] = card

    def __repr__(self):
        return f"{self.__cards!r}"

    def __str__(self):
        return f"{[str(card) for card in self.__cards]}"

    def __len__(self):
        return len(self.__cards)

    def json(self, name="Deck"):
        return {name: [repr(card) for card in self.__cards]}

    def json_string(self, name="Deck"):
        return json.dumps(self.json(name), ensure_ascii=False)

    def shuffle(self, seed: float = random.random(), times: int = 3) -> Self:
        default_logger.debug(json.dumps({"Seed": seed}))
        random.seed(seed)
        for _ in range(times):
            random.shuffle(self.__cards)
        return self

    def deal(self, no_hands: int = 0, per_hands: int = 0) -> tuple[list[Self], Self]:
        return [
            Deck(cards=self.cards[n : per_hands * no_hands : no_hands])
            for n in range(no_hands)
        ], Deck(cards=self.cards[per_hands * no_hands : :])


class CTDeck(Deck):
    def deal(self, no_hands=4, per_hands=6):
        return super().deal(no_hands, per_hands)


class VCDeck(Deck):
    def set_ranks(self, ranks=[str(r) for r in range(3, 11)] + list("JQKA2")):
        return super().set_ranks(ranks)

    @staticmethod
    def default_value_fn(rank, suit, ranks, suits):
        return ranks.index(rank) * len(suits) + suits.index(suit)

    def build_deck(self, value_fn=default_value_fn):
        return super().build_deck(value_fn)

    def deal(self, no_hands=4, per_hands=13):
        return super().deal(no_hands, per_hands)


def demo():
    deck = VCDeck().build_default_deck().shuffle()
    default_logger.debug(deck.json_string())

    hands, leftover = deck.deal()

    for i, hand in enumerate(hands):
        default_logger.debug(f"Player {i} hand: {sorted(hand)}")
    default_logger.debug(f"Leftover: {leftover}")
