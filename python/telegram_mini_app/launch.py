# from sys import argv
# from urllib.parse import urlparse, parse_qs
# import json
# from utils import default_logger
import asyncio
import platform
from pyray import *


# def init_data_parser(init_data: str):
#     result = parse_qs(init_data)
#     if "user" in result:
#         result["user"] = [json.loads(user) for user in result["user"]]
#         return result
#     else:
#         return init_data


async def launch():
    # for arg in argv:
    #     default_logger.info(json.dumps(init_data_parser(arg)))

    init_window(500, 500, "Hello")
    platform.window.window_resize()
    while not window_should_close():
        begin_drawing()
        clear_background(WHITE)
        draw_text("Hello world", 190, 200, 20, VIOLET)
        end_drawing()
        await asyncio.sleep(0)
    close_window()
