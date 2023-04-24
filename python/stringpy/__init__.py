from stringpy import _stringpy
from pyarrow import Array
from typing import Callable, List
import inspect

def exporter(func: Callable):
    def inner(*args, **kwargs):
        rust_func = func.__name__
        # print(func.__annotations__)
        # print(func.__defaults__)
        # keys = reversed(list(filter(lambda x: x != "return", func.__annotations__)))
        # values = reversed(func.__defaults__)
        #kw_with_defaults = dict(zip(keys, values))
        kw_with_defaults = {k:v.default for k,v in dict( inspect.signature(func).parameters).items() if v.default is not inspect._empty}

        for k in kw_with_defaults:
            if k in kwargs:
                kw_with_defaults[k] = kwargs.pop(k)

        return getattr(_stringpy, rust_func)(*args,**kw_with_defaults, **kwargs)
    return inner


# def f1(a: list, b: str=None, c =10, *args) -> str:
#     pass


# f1.__defaults__
# f1.__annotations__

# f1.__annotations__.pop('return')


@exporter
def str_c(array: Array, collapse: str=None) -> str :
    """Collapse a vector of str into a single string

    Parameters
    ----------
    array : Array
        _description_
    collapse : str


    Returns
    -------
    str
    """

@exporter

def str_combine(*args, sep: str=None) -> List:
    """Vectorize function to combine multiple

    Parameters
    ----------
    args: Array to combine
    sep : str
        separator

    Returns
    -------
    Array
    """

