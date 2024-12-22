from utils.log_util import combined_logger
import random


class Deck:
    ranks = [str(r) for r in range(2, 11)] + list("JQKA")
    suits = "spades clubs diamonds hearts".split()

    class Card:
        def __init__(self, rank, suit):
            self.rank = rank
            self.suit = suit

        def value(self) -> int:
            return Deck.ranks.index(self.rank) * 4 + Deck.suits.index(self.suit)

        def __repr__(self):
            return f"{self.rank} of {self.suit}"

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
        return self._cards.__repr__()

    def __len__(self):
        return len(self._cards)

    def deal(self, no_hands=4):
        return [(self[n : 6 * no_hands : no_hands]) for n in range(0, no_hands)]


class VCDeck(Deck):
    ranks = [str(r) for r in range(3, 11)] + list("JQKA2")
    suits = "spades clubs diamonds hearts".split()

    class VCCard(Deck.Card):
        def value(self):
            return VCDeck.ranks.index(self.rank) * 4 + VCDeck.suits.index(self.suit)

    def __init__(self):
        self._cards = [
            VCDeck.VCCard(rank, suit) for rank in self.ranks for suit in self.suits
        ]

    def deal(self, no_hands=4):
        return [(self[n : 13 * no_hands : no_hands]) for n in range(0, no_hands)]


def demo():
    deck = VCDeck()
    random.shuffle(deck)
    combined_logger.debug(f"Deck ({len(deck)}): {deck}")

    hands = deck.deal(3)
    for i, hand in enumerate(hands):
        combined_logger.debug(f"Player {i} hand: {sorted(hand)}")
