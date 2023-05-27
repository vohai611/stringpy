
import inspect
from functools import wraps
from typing import Callable, List
import pyarrow as pa
from stringpy import _stringpy


def check_same_length(lists: List[pa.Array]):
    it = iter(lists)
    the_len = len(next(it))
    if not all(len(i) == the_len for i in it):
        raise ValueError('Not all array have same length!')


def sync_kw(kwargs, kw_with_defaults):
    for k in kwargs:
        if k in kw_with_defaults:
            kw_with_defaults[k] = kwargs[k]
        else:
            raise KeyError(f"No such argument {k}")


def is_scala(x: any):
    return isinstance(x, (int, float, str, bool))


def exporter(_func=None, vectorize_arg: List = None):
    """Use this when accepting only one array as argument"""
    def decorator_register(func: Callable):
        @wraps(func)
        def inner(array, **kwargs):
            rust_func = func.__name__
            # use default kwargs in python function
            kw_with_defaults = {k: v.default for k, v in dict(inspect.signature(
                func).parameters).items() if v.default is not inspect._empty}

            # FIXME need to check length =1 , length = array, corece to array length if = 1
            array = pa.array(array) if not isinstance(array, pa.Array) else array
            sync_kw(kwargs, kw_with_defaults)

            if vectorize_arg is not None:
                for i in vectorize_arg:
                    if is_scala(kw_with_defaults[i]):
                        kw_with_defaults[i] = [kw_with_defaults[i]]
                    elif type(kw_with_defaults[i]) is not list:
                        kw_with_defaults[i] = list(kw_with_defaults[i])
                    else:
                        TypeError(
                            f'Can not corce {i} to list, please check your input a')

                    if (len(kw_with_defaults[i]) != 1) | (len(kw_with_defaults[i]) != len(array)):
                        ValueError( f"Length of {i} must be equal to 1 or to length of array")
                        

            return getattr(_stringpy, rust_func)(array, **kw_with_defaults)
        return inner
    if _func is None:
        return decorator_register
    else:
        return decorator_register(_func)


def exporter2(func: Callable):
    """Use this when accepting multiple (arbitrary number of) arrays as arguments"""
    @wraps(func)
    def inner(*args, **kwargs):
        rust_func = func.__name__
        # use default kwargs in python function
        kw_with_defaults = {k: v.default for k, v in dict(inspect.signature(
            func).parameters).items() if v.default is not inspect._empty}

        if len(args) == 1:
            raise ValueError("At least two arrays are required")
        check_same_length(args)

        args = list(args)
        args = [pa.array(a) if not isinstance(a, pa.Array) else a for a in args]
        args = tuple(args)
        sync_kw(kwargs, kw_with_defaults)

        return getattr(_stringpy, rust_func)(*args, **kw_with_defaults)
    return inner
