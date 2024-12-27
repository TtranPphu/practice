import logging
import logging.handlers
import atexit


class CustomQueueHandler(logging.handlers.QueueHandler):
    def __init__(self, queue):
        super().__init__(queue)
