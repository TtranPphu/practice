from utils.log_util import combined_logger
import random


class Deck:
    ranks = [str(r) for r in range(2, 11)] + list("JQKA")
    suits = list("♠♣♦♥")

    class Card:
        def __init__(self, rank, suit):
            self.rank = rank
            self.suit = suit

        def value(self) -> int:
            return Deck.ranks.index(self.rank) * 4 + Deck.suits.index(self.suit)

        def __repr__(self):
            return f"{(self.rank+self.suit)!r}"

        def __eq__(self, other):
            return self.value() == other.value()

        def __lt__(self, other):
            return self.value() < other.value()

    def __init__(self):
        self._cards = [
            Deck.Card(rank, suit) for rank in self.ranks for suit in self.suits
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

    def deal(self, no_hands=4, per_hands=6):
        return [
            (self[n : per_hands * no_hands : no_hands]) for n in range(0, no_hands)
        ], self[per_hands * no_hands : :]


class VCDeck(Deck):
    ranks = [str(r) for r in range(3, 11)] + list("JQKA2")

    class VCCard(Deck.Card):
        def value(self):
            return VCDeck.ranks.index(self.rank) * 4 + VCDeck.suits.index(self.suit)

    def __init__(self):
        self._cards = [
            VCDeck.VCCard(rank, suit) for rank in self.ranks for suit in self.suits
        ]

    def deal(self, no_hands=4, per_hands=13):
        return super().deal(no_hands, per_hands)


def demo():
    deck = VCDeck()
    random.shuffle(deck)
    combined_logger.debug(f"Deck: {deck} ({len(deck)} cards)")

    hands, leftover = deck.deal(no_hands=3)
    for i, hand in enumerate(hands):
        combined_logger.debug(f"Player {i} hand: {sorted(hand)} ({len(hand)} cards)")
    combined_logger.debug(f"Leftover: {leftover} ({len(leftover)} cards)")
