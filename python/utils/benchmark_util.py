from .log_util import default_logger
from time import perf_counter
from functools import wraps


def benchmark(func):
    @wraps(func)
    def wrapper(*args, **kwargs):
        start = perf_counter()
        result = func(*args, **kwargs)
        end = perf_counter()
        default_logger.debug(f"{func.__name__} executed in {(end - start):.8f} seconds")
        return result

    return wrapper
