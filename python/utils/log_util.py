from logging import getLogger
import logging.config
import json5 as json
import atexit


def run_once(f):
    def wrapper(*args, **kwargs):
        if not wrapper.has_run:
            wrapper.has_run = True
            return f(*args, **kwargs)

    wrapper.has_run = False
    return wrapper


@run_once
def init_logging():
    with open("utils/log_config.json") as file:
        log_config = json.load(file)

    logging.config.dictConfig(config=log_config)
    root_logger = getLogger("root")
    queue_handler = root_logger.handlers[0]
    queue_handler.listener.start()
    atexit.register(queue_handler.listener.stop)


init_logging()
default_logger = getLogger("default")
