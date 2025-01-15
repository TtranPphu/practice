from utils import default_logger
from typing import Self


class Tensor:
    """A simple tensor class that supports slicing and indexing."""

    @property
    def tensor(self) -> list:
        return self.__tensor

    def __init__(self, tensor: list):
        self.__tensor = list()
        for item in tensor:
            if isinstance(item, list):
                self.__tensor.append(Tensor(item))
            else:
                self.__tensor.append(item)

    def __str__(self) -> str:
        return str(self.__tensor)

    def __repr__(self) -> str:
        return repr(self.__tensor)

    def __len__(self) -> int:
        return len(self.__tensor)

    def __hash__(self):
        return hash(self.__tensor)

    def __eq__(self, value):
        return not any(x != y for x, y in zip(self.__tensor, value))

    def __ne__(self, value):
        return not any(x == y for x, y in zip(self.__tensor, value))

    def is_similar(self, value):
        if len(self) != len(value):
            return False
        for x, y in zip(self, value):
            if isinstance(x, Tensor) and isinstance(y, Tensor) and not x.is_similar(y):
                return False
            if x != y:
                return False
        return True

    def __iter__(self):
        return iter(self.__tensor)

    def __contains__(self, value):
        return value in self.__tensor

    def __getitem__(self, index: int | slice | tuple):
        if isinstance(index, int):
            return self.__tensor[index]
        elif isinstance(index, slice):
            return self.__tensor[index.start : index.stop : index.step]
        elif isinstance(index, tuple):
            result = self
            for i in index:
                result = result[i]
            return result

    def __setitem__(self, index: int | tuple, value: int):
        if isinstance(index, int):
            self.__tensor[index] = value
        elif isinstance(index, tuple):
            self[index[:-1]].__tensor[index[-1]] = value


def demo():
    t = Tensor([[[1, 2, 3], [4, 5, 6]], [[7, 8, 9], [10, 11, 12]]])
    tt = Tensor([[[1, 2, 3], [4, 5, 6]], [[7, 8, 9], [10, 11, 12]]])
    s = Tensor([[[7, 8, 9], [10, 11, 12]], [[4, 5, 6], [3, 2, 1]]])

    default_logger.debug(f"{id(t[1])}")
    default_logger.debug(f"{id(tt[1])}")
    default_logger.debug(f"{t==tt}")
