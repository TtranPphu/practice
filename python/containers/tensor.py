from utils import default_logger
from typing import Any


class Tensor:
    """A simple tensor class that supports slicing and indexing."""

    @property
    def tensor(self):
        return self.__tensor

    @property
    def dimensions(self):
        return self.__dimensions

    def __init__(self, tensor: list):
        self.__tensor = tensor
        self.__dimensions = []
        while isinstance(tensor, list):
            self.__dimensions.append(len(tensor))
            if isinstance(tensor[0], list):
                assert all(len(t) == len(tensor[0]) for t in tensor)
            tensor = tensor[0]

    def __getitem__(self, indexes: tuple):
        result = self.tensor
        for index, dimension in zip(indexes, self.dimensions):
            if isinstance(index, int):
                assert 0 <= index < dimension, f"Index {index} out of bounds."
                result = result[index]
            elif isinstance(index, slice):
                assert (
                    not index.start or 0 <= index.start < dimension
                ), f"Index {index.start} out of bounds."
                assert (
                    not index.stop or 0 <= index.stop < dimension
                ), f"Index {index.stop} out of bounds."
                result = result[index.start : index.stop : index.step]
        return result

    def __setitem__(self, indexes: tuple, value: Any):
        result = self.tensor
        for index in indexes[:-1]:
            result = result[index]
        result[indexes[-1]] = value

    def __repr__(self):
        return f"{self.tensor!r}"

    def __str__(self):
        return f"{self.tensor}"


class TensorDict:
    @property
    def tensor_dict(self):
        return self.__tensor_dict

    @tensor_dict.setter
    def tensor_dict(self, tensor_dict: dict):
        self.__tensor_dict = tensor_dict

    def __init__(self, tensor_dict: dict):
        self.__tensor_dict = tensor_dict

    def __getitem__(self, keys: tuple):
        result = self.tensor_dict
        for key in keys:
            result = result[key]
        return result

    def __setitem__(self, keys: tuple, value: Any):
        result = self.tensor_dict
        for key in keys[:-1]:
            if key not in result:
                result[key] = {}
            result = result[key]
        result[keys[-1]] = value

    def __delitem__(self, keys: tuple):
        result = self.tensor_dict
        for key in keys[:-1]:
            result = result[key]
        del result[keys[-1]]

    def __iter__(self):
        pass

    def __repr__(self):
        return f"{self.tensor_dict!r}"

    def __str__(self):
        return f"{self.tensor_dict}"


def demo():
    t = Tensor([[[1, 2, 3], [4, 5, 6]], [[7, 8, 9], [10, 11, 12]]])
    default_logger.debug(t[0, 1])
    default_logger.debug(t[1, 0, 2])
    default_logger.debug(t[1, 1, 0:2])
    default_logger.debug(t[1, 1, 0::2])
    t[1, 1, 0] = 100
    default_logger.debug(t)
