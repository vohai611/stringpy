'''This module provide a set of vectorized function to manipulate string, mostly mimic the main functionality of stringr package in R.
As this package use pyarrow as a bridge to communicate with Rust, it ONLY work for any input that can convert to pyarray. And the resust is also a pyarry in most of cases.'''

from stringpy import _stringpy
from pyarrow import Array, ListArray
import pyarrow as pa
from typing import Callable, List
import inspect
from functools import wraps


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
    @wraps(func)
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


@exporter
def str_c(array: Array, collapse: str = None) -> str:
    """Collapse a vector of str into a single string

    Parameters
    ----------
    array : Array
        _description_
    collapse : str

    Examples
    --------
    >>> str_c(['abc', 'def', 'ghi'])
    'abcdefghi'

    >>> str_c(['abc', 'def', 'ghi'], collapse = '-')
    'abc-def-ghi'

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

    Examples
    --------
    >>> str_combine(['a', 'b', 'c'], ['d', 'e', 'f'], sep = '-')
    ['a-d', 'b-e', 'c-f']

    Returns
    -------
    Array
    """


@exporter
def str_count(array: Array, pattern: str = None) -> Array:
    """Count the number of times a pattern occurs in each string

    Parameters
    ----------
    array : Array
    pattern : str

    Examples
    --------
    >>> str_count(['abc', 'def', 'ghi'], pattern = 'a').to_pylist()
    [1, 0, 0]

    Returns
    -------
    Array
    """


@exporter
def str_replace(array: Array, pattern: str = None, replace: str = None) -> Array:
    """Replace a first matching pattern in string array 

    Parameters
    ----------
    array : Array
    pattern : str
    replace : str

    Examples
    --------
    >>> str_replace(['abc', 'def', 'ghi'], pattern = '\w', replace = 'x').to_pylist()
    ['xbc', 'xef', 'xhi']

    Returns
    -------
    Array
    """


@exporter
def str_remove(array: Array, pattern: str = None) -> Array:
    """Remove a first matching pattern in string array 

    Parameters
    ----------
    array : Array
    pattern : str

    Examples
    --------
    >>> str_remove(['abc 12', 'def 23', 'ghi 34'], pattern = '\d').to_pylist()
    ['abc 2', 'def 3', 'ghi 4']

    Returns
    -------
    Array
    """


@exporter
def str_remove_all(array: Array, pattern: str = None) -> Array:
    """Remove all matching pattern in string array 

    Parameters
    ----------
    array : Array
    pattern : str

    Examples
    --------
    >>> str_remove_all(['abc 1', 'def 2', 'ghi 3'], pattern = '\d').to_pylist()
    ['abc ', 'def ', 'ghi ']

    Returns
    -------
    Array
    """


@exporter
def str_replace_all(array: Array, pattern: str = None, replace: str = None) -> Array:
    """Replace all matching pattern in string array 

    Parameters
    ----------
    array : Array
    pattern : str
    replace : str

    Examples
    --------
    >>> str_replace_all(['abc 122', 'def 233', 'ghi 344'], pattern = '\d', replace = 'x').to_pylist()
    ['abc xxx', 'def xxx', 'ghi xxx']


    Returns
    -------
    Array
    """


@exporter
def str_squish(array: Array) -> Array:
    """Remove all leading, trailing and in between word whitespace from each string

    Parameters
    ----------
    array : Array

    Examples
    --------
    >>> str_squish([' abc  def', ' def    ghi', 'ijk row  ']).to_pylist()
    ['abc def', 'def ghi', 'ijk row']

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

    Examples
    --------
    >>> str_remove_ascent(['sài gòn', 'thời tiết', 'cảm lạnh']).to_pylist()
    ['sai gon', 'thoi tiet', 'cam lanh']

    Returns
    -------
    Array
    """


@exporter
def str_detect(array: Array, pattern: str = None) -> Array:
    """Detect if each string match a pattern, return a boolean array

    Parameters
    ----------
    array : Array
    pattern : str

    Examples
    --------
    >>> str_detect(['abc', 'def', 'ghi'], pattern = 'a').to_pylist()
    [True, False, False]

    Returns
    -------
    Array
    """


@exporter
def str_trim(array: Array, side='both') -> Array:
    """Remove leading and trailing whitespace from each string

    Parameters
    ----------
    array : Array

    Examples
    --------
    >>> str_trim([' abc  def', ' def    ghi', 'ijk row  ']).to_pylist()
    ['abc  def', 'def    ghi', 'ijk row']


    Returns
    -------
    Array
    """


@exporter
def str_trunc(array: Array, width: int = None, side='left', ellipsis='...') -> Array:
    """Truncate each string to a given width, note that this function does NOT support non-ascii characters yet.

    Parameters
    ----------
    array : Array
    width : int
    side : str
        One of 'left', 'right', 'center'
    ellipsis : str
        Content of ellipsis that indicates content has been removed.

    Examples
    --------
    >>> str_trunc(['abc def', 'def ghi', 'ijk row'], width = 5).to_pylist()
    ['abc d...', 'def g...', 'ijk r...']

    Returns
    -------
    Array
    """


@exporter
def str_extract(array: Array, pattern: str = None, group: int = None) -> Array:
    """Extract a first matching pattern in string array 

    Parameters
    ----------
    array : Array
    pattern : str
    group : int
        Group number to extract, by default not use

    Examples
    --------
    >>> str_extract(['abc', 'def', 'ghi'], pattern = '\w').to_pylist()
    ['a', 'd', 'g']

    Returns
    -------
    Array
    """


@exporter
def str_extract_all(array: Array, pattern: str = None, group: int = None) -> ListArray:
    """Extract all matching pattern in string array, for each string input return list of matching output

    Parameters
    ----------
    array : Array
    pattern : str
    group : int
        Group number to extract, by default not use

    Examples
    --------
    >>> str_extract_all(['abc12', 'd13ef', 'gh23i'], pattern = '\d').to_pylist()
    [['1', '2'], ['1', '3'], ['2', '3']]

    Returns
    -------
    ListArray
    """


@exporter
def str_split(array: Array, pattern: str = None) -> ListArray:
    """Split each string by a pattern, return a list[array], each array in the list is correspond to a string in input array

    Parameters
    ----------
    array : Array
    pattern : str

    Returns
    -------
    ListArray
    """


@exporter
def str_starts(array: Array, pattern: str = None, negate: bool = False) -> Array:
    """Detect if each string starts with a pattern, return a boolean array

    Parameters
    ----------
    array : Array
    pattern : str
        Expect a literal string, not a regex, all regex special characters will be escaped
    negate : bool
        Negate the result
    Examples
    --------
    >>> str_starts(['abc', 'def', 'ghi'], pattern = 'a').to_pylist()
    [True, False, False]

    >>> str_starts(['a.bc', 'adef', 'aghi'], pattern = 'a.').to_pylist()
    [True, False, False]

    Returns
    -------
    Array
    """


@exporter
def str_ends(array: Array, pattern: str = None, negate: bool = False) -> Array:
    """Detect if each string ends with a pattern, return a boolean array

    Parameters
    ----------
    array : Array
    pattern : str
        Expect a literal string, not a regex, all regex special characters will be escaped
    negate : bool
        Negate the result
    Examples
    --------
    >>> str_ends(['abc', 'def', 'ghi'], pattern = 'c').to_pylist()
    [True, False, False]

    >>> str_ends(['ab.c', 'defc', 'ghic'], pattern = '.c').to_pylist()
    [True, False, False]

    Returns
    -------
    Array
    """


@exporter
def str_subset(array: Array, pattern: str = None, negate: bool = False) -> Array:
    """Subset (filter) array with a pattern, return string array

    Parameters
    ----------
    array : Array
    pattern : str
        Expect a literal string, not a regex, all regex special characters will be escaped
    negate : bool
        Negate the result
    Examples
    --------
    >>> str_subset(['apple', 'banana', 'pear', 'pineapple'], pattern = '^a').to_pylist()
    ['apple']

    >>> str_subset(['abc', 'def', 'ghi'], pattern = 'a', negate = True).to_pylist()
    ['def', 'ghi']

    Returns
    -------
    Array
    """


@exporter
def str_dup(array: Array, times: int = 1) -> Array:
    """Duplicate each string in array by times

    Parameters
    ----------
    array : Array
    times : int

    Examples
    --------
    >>> str_dup(['abc', 'def', 'ghi'], times = 2).to_pylist()
    ['abcabc', 'defdef', 'ghighi']

    Returns
    -------
    Array
    """


@exporter
def str_length(array: Array) -> Array:
    """Get length of each string in array.These are the individual elements (which are often, but not always letters)
    For example length of "Hà Nội" will be 6


    Parameters
    ----------
    array : Array

    Examples
    --------
    >>> str_length(['abc', 'def', 'ghi', None ,'']).to_pylist()
    [3, 3, 3, None, 0]

    Returns
    -------
    Array
    """


@exporter
def str_unique(array: Array) -> Array:
    """Get unique strings in array

    Parameters
    ----------
    array : Array

    Examples
    --------
    >>> str_unique(['abc', 'def', 'ghi', 'abc', 'def']).to_pylist()
    ['abc', 'def', 'ghi']

    Returns
    -------
    Array
    """


@exporter
def str_to_lower(array: Array) -> Array:
    """Convert each string to lower case

    Parameters
    ----------
    array : Array

    Examples
    --------
    >>> str_to_lower(['ABC', 'Def', 'Ghi']).to_pylist()
    ['abc', 'def', 'ghi']

    Returns
    -------
    Array
    """


@exporter
def str_to_upper(array: Array) -> Array:
    """Convert each string to upper case

    Parameters
    ----------
    array : Array

    Examples
    --------
    >>> str_to_upper(['abc', 'Def', 'Ghi']).to_pylist()
    ['ABC', 'DEF', 'GHI']

    Returns
    -------
    Array
    """


@exporter
def str_to_title(array: Array) -> Array:
    """Convert each string to title case

    Parameters
    ----------
    array : Array

    Examples
    --------
    >>> str_to_title(['abc', 'def', 'ghi']).to_pylist()
    ['Abc', 'Def', 'Ghi']

    Returns
    -------
    Array
    """


@exporter
def str_to_sentence(array: Array) -> Array:
    """Convert each string to sentence case

    Parameters
    ----------
    array : Array

    Examples
    --------
    >>> str_to_sentence(['i need you here. right now!']).to_pylist()
    ['I need you here. Right now!']

    Returns
    -------
    Array
    """


@exporter
def str_pad(array: Array, width: int=None, side: str = 'left', pad: str = ' ') -> Array:
    """Pad each string with pad string to width

    Parameters
    ----------
    array : Array
    width : int
    side : str
        'left', 'right', 'both'
    pad : str

    Examples
    --------
    >>> str_pad(['abc', 'def', 'ghi'], width = 5, side = 'left', pad = '0').to_pylist()
    ['00abc', '00def', '00ghi']

    Returns
    -------
    Array
    """
