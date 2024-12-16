from logging import getLogger, DEBUG, INFO, WARNING, Formatter, StreamHandler
from logging.handlers import TimedRotatingFileHandler, SysLogHandler
from sys import stdout
from os import path, mknod

local_logger = getLogger("local")
local_logger.setLevel(DEBUG)
local_formater = Formatter(
    (
        "%(asctime)s | [%(levelname)s] | %(funcName)s "
        "| %(filename)s:%(lineno)s | %(message)s"
    ),
    "%Y-%m-%d %H:%M:%S",
)

stream_handler = StreamHandler(stdout)
stream_handler.setFormatter(local_formater)
stream_handler.setLevel(DEBUG)

this_path = path.dirname(__file__)
log_path = path.join(this_path, "log.log")
if not path.exists(log_path):
    mknod(log_path)
# file_handler = RotatingFileHandler(log_path, "a", 1024)
file_handler = TimedRotatingFileHandler(log_path, when="midnight")
file_handler.setFormatter(local_formater)
file_handler.setLevel(INFO)

local_logger.addHandler(stream_handler)
local_logger.addHandler(file_handler)

PAPERTRAIL_HOST = "logs6.papertrailapp.com"
PAPERTRAIL_PORT = 11443
papertrail_logger = getLogger("papertrail")
papertrail_logger.setLevel(WARNING)
papertrail_handler = SysLogHandler(address=(PAPERTRAIL_HOST, PAPERTRAIL_PORT))
papertrail_logger.addHandler(papertrail_handler)

combined_logger = getLogger("combined")
combined_logger.setLevel(DEBUG)
combined_logger.addHandler(file_handler)
combined_logger.addHandler(stream_handler)
combined_logger.addHandler(papertrail_handler)
