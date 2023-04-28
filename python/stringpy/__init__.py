from stringpy import _stringpy
from pyarrow import Array
import pyarrow as pa
from typing import Callable, List
import inspect


def check_same_length(lists: List[Array]):
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
    

def exporter(func: Callable):
    """Use this when accepting only one array as argument"""
    def inner(array, **kwargs):
        rust_func = func.__name__
        # use default kwargs in python function
        kw_with_defaults = {k: v.default for k, v in dict(inspect.signature(
            func).parameters).items() if v.default is not inspect._empty}
        
        array = pa.array(array) if not isinstance(array, pa.Array) else array
        sync_kw(kwargs, kw_with_defaults)

        return getattr(_stringpy, rust_func)(array, **kw_with_defaults)
    return inner

def exporter2(func: Callable):
    """Use this when accepting multiple arrays as arguments"""
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

@exporter
def str_c(array: Array, collapse: str = None) -> str:
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


@exporter2
def str_combine(*args, sep: str = None) -> List:
    """Combine multiple arrays into a one array of strings

    Parameters
    ----------
    args: Array to combine
    sep : str
        separator

    Returns
    -------
    Array
    """


@exporter
def str_count(array: Array, pattern: str=None) -> Array:
    """Count the number of times a pattern occurs in each string

    Parameters
    ----------
    array : Array
    pattern : str

    Returns
    -------
    Array
    """


@exporter
def str_replace(array: Array, pattern: str=None, replace: str=None) -> Array:
    """Replace a first matching pattern in string array 

    Parameters
    ----------
    array : Array
    pattern : str
    replace : str

    Returns
    -------
    Array
    """


@exporter
def str_replace_all(array: Array, pattern: str=None, replace: str=None) -> Array:
    """Replace all matching pattern in string array 

    Parameters
    ----------
    array : Array
    pattern : str
    replace : str

    Returns
    -------
    Array
    """


@exporter
def str_squish(array: Array) -> Array:
    """Remove all leading and trailing whitespace from each string

    Parameters
    ----------
    array : Array

    Returns
    -------
    Array
    """


@exporter
def str_remove_ascent(array: List) -> Array:
    """Remove all accents from each string

    Parameters
    ----------
    array : Array

    Returns
    -------
    Array
    """


@exporter
def str_detect(array: Array, pattern: str=None) -> Array:
    """Detect if each string match a pattern, return a boolean array

    Parameters
    ----------
    array : Array
    pattern : str

    Returns
    -------
    Array
    """

@exporter
def str_trim(array: Array, side = 'both') -> Array:
    """Remove all leading and trailing whitespace from each string

    Parameters
    ----------
    array : Array

    Returns
    -------
    Array
    """