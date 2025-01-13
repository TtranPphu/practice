from .deck_of_cards import Card, Deck

from utils import default_logger
from typing import override, Self
from colorama import Fore, Style, init as colorama_init


class UnoCard(Card):
    def __str__(self):
        match self.suit:
            case "R":
                return f"{Fore.RED}{(self.rank)!s}{Style.RESET_ALL}"
            case "G":
                return f"{Fore.GREEN}{(self.rank)!s}{Style.RESET_ALL}"
            case "B":
                return f"{Fore.BLUE}{(self.rank)!s}{Style.RESET_ALL}"
            case "Y":
                return f"{Fore.YELLOW}{(self.rank)!s}{Style.RESET_ALL}"


class UnoDeck(Deck):
    def set_ranks(
        self,
        ranks: list[str] = [str(r) for r in range(1, 10) for _ in range(2)]
        + ["0"]
        + ["S", "R", "+2", "+4", "W"],
    ):
        return super().set_ranks(ranks=ranks)

    def set_suits(self, suits: list[str] = list("RGBY")):
        return super().set_suits(suits=suits)

    @staticmethod
    def default_value_fn(self, rank, suit, ranks, suits):
        if rank == "+4" or rank == "W":
            return 4
        else:
            return suits.index(suit)

    def build_deck(self, value_fn=Deck.default_value_fn):
        self.cards = [
            UnoCard(
                rank_suit=rank + suit,
                value=value_fn(rank, suit, self.ranks, self.suits),
            )
            for rank in self.ranks
            for suit in self.suits
        ]
        return self


def demo():
    deck = UnoDeck().build_default_deck().shuffle()
    default_logger.debug(deck.json_string())

    hands, leftover = deck.deal()

    for i, hand in enumerate(hands):
        default_logger.debug(f"Player {i} hand: {sorted(hand)}")
    default_logger.debug(f"Leftover: {leftover}")
