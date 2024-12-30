#! /usr/bin/python3
from deck_of_cards.blackjack_deck_of_cards import demo
from utils import default_logger
import math
import json

from urllib.parse import urlparse, parse_qs


class Vector:
    def __init__(self, x=0, y=0):
        self.x = x
        self.y = y

    def __repr__(self):
        return f"Vector({self.x!r}, {self.y!r})"

    def __add__(self, other):
        return Vector(self.x + other.x, self.y + other.y)

    def __mul__(self, scalar):
        return Vector(self.x * scalar, self.y * scalar)

    def __abs__(self):
        return math.hypot(self.x, self.y)

    def __bool__(self):
        return bool(abs(self))


def main():
    v1 = Vector(2, 3)
    v2 = Vector(1, 1)
    v3 = v1 + v2
    default_logger.debug(f"{v3!r}")
    default_logger.debug(f"{abs(v3)}")
    v4 = v3 * 3
    default_logger.debug(f"{v4!r}")
    default_logger.debug(f"{abs(v3*3)}")


def url_parse_test(init_data: str):
    result = parse_qs(init_data)
    if "user" in result:
        result["user"][0] = json.loads(result["user"][0])
    return result


if __name__ == "__main__":
    # demo()
    # main()

    init_data = "user=%7B%22id%22%3A279058397%2C%22first_name%22%3A%22Vladislav%20%2B%20-%20%3F%20%5C%2F%22%2C%22last_name%22%3A%22Kibenko%22%2C%22username%22%3A%22vdkfrost%22%2C%22language_code%22%3A%22ru%22%2C%22is_premium%22%3Atrue%2C%22allows_write_to_pm%22%3Atrue%2C%22photo_url%22%3A%22https%3A%5C%2F%5C%2Ft.me%5C%2Fi%5C%2Fuserpic%5C%2F320%5C%2F4FPEE4tmP3ATHa57u6MqTDih13LTOiMoKoLDRG4PnSA.svg%22%7D&chat_instance=8134722200314281151&chat_type=private&auth_date=1733584787&hash=2174df5b000556d044f3f020384e879c8efcab55ddea2ced4eb752e93e7080d6&signature=zL-ucjNyREiHDE8aihFwpfR9aggP2xiAo3NSpfe-p7IbCisNlDKlo7Kb6G4D0Ao2mBrSgEk4maLSdv6MLIlADQ"
    parsed = url_parse_test(init_data)
    default_logger.debug(json.dumps(parsed))
