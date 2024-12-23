from logging import getLogger
import logging.config
import json5 as json


with open("utils/log_config.json") as file:
    log_config = json.load(file)

logging.config.dictConfig(config=log_config)

default_logger = getLogger("default")
