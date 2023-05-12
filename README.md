Stringpy
================
Hai Vo
5/1/23

- <a href="#introduction" id="toc-introduction">Introduction</a>
- <a href="#installation" id="toc-installation">Installation</a>
- <a href="#usage-example" id="toc-usage-example">Usage example</a>
- <a href="#speed-comparison" id="toc-speed-comparison">Speed
  comparison</a>
- <a href="#implement-list" id="toc-implement-list">Implement list</a>
- <a href="#different-type-of-io" id="toc-different-type-of-io">Different
  type of i/o</a>

[![Documentation
Status](https://readthedocs.org/projects/stringpy/badge/?version=latest.png)](https://stringpy.readthedocs.io/en/latest/?badge=latest)
[![CI](https://github.com/vohai611/stringpy/actions/workflows/CI.yml/badge.svg?branch=main)](https://github.com/vohai611/stringpy/actions/workflows/CI.yml)

# Introduction

This project is a python package to mimic
[r::stringr](https://stringr.tidyverse.org/) functionalities, the core
functions are written in Rust. Note that I write this package mostly for
personal use (convenience and speed) and learning purpose, so please use
with care!

# Installation

This package is not on Pipy yet, so you need to compile from source.

First you need rust compiler:

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Then install this package as normal python package:

    git clone https://github.com/vohai611/stringpy.git
    pip3 install ./stringpy

# Usage example

# Speed comparison

Although this package is not aim to speed optimization, but in most
case, it still get a decent speed up compare with pandas, thank to Rust!

Below are some of random comparison between `stringpy` and `pandas`:

<details>
<summary>Code</summary>

``` python
import stringpy as sp
import pandas as pd
import numpy as np
import random
import string

letters = string.ascii_lowercase
a = [''.join(random.choice(letters) for i in range(10))  for i in range(600_000)]

a_sr = pd.Series(a)
```

</details>

## Replace pattern

<details>
<summary>Code</summary>

``` python
%%time
a_sr.str.replace('\w', 'b', regex=True)
```

</details>

    CPU times: user 439 ms, sys: 11.8 ms, total: 451 ms
    Wall time: 462 ms

    0         bbbbbbbbbb
    1         bbbbbbbbbb
    2         bbbbbbbbbb
    3         bbbbbbbbbb
    4         bbbbbbbbbb
                 ...    
    599995    bbbbbbbbbb
    599996    bbbbbbbbbb
    599997    bbbbbbbbbb
    599998    bbbbbbbbbb
    599999    bbbbbbbbbb
    Length: 600000, dtype: object

<details>
<summary>Code</summary>

``` python
%%time
sp.str_replace_all(a, pattern='\w', replace= 'b')
```

</details>

    CPU times: user 254 ms, sys: 10.7 ms, total: 265 ms
    Wall time: 274 ms

    <pyarrow.lib.StringArray object at 0x1170973a0>
    [
      "bbbbbbbbbb",
      "bbbbbbbbbb",
      "bbbbbbbbbb",
      "bbbbbbbbbb",
      "bbbbbbbbbb",
      "bbbbbbbbbb",
      "bbbbbbbbbb",
      "bbbbbbbbbb",
      "bbbbbbbbbb",
      "bbbbbbbbbb",
      ...
      "bbbbbbbbbb",
      "bbbbbbbbbb",
      "bbbbbbbbbb",
      "bbbbbbbbbb",
      "bbbbbbbbbb",
      "bbbbbbbbbb",
      "bbbbbbbbbb",
      "bbbbbbbbbb",
      "bbbbbbbbbb",
      "bbbbbbbbbb"
    ]

## Counting

<details>
<summary>Code</summary>

``` python
%%time
a_sr.str.count('a')
```

</details>

    CPU times: user 141 ms, sys: 6.63 ms, total: 148 ms
    Wall time: 161 ms

    0         0
    1         0
    2         0
    3         1
    4         1
             ..
    599995    1
    599996    0
    599997    0
    599998    0
    599999    0
    Length: 600000, dtype: int64

<details>
<summary>Code</summary>

``` python
%%time
sp.str_count(a, pattern='a')
```

</details>

    CPU times: user 24.7 ms, sys: 1.94 ms, total: 26.6 ms


    Wall time: 28.3 ms

    <pyarrow.lib.Int32Array object at 0x117097460>
    [
      0,
      0,
      0,
      1,
      1,
      1,
      0,
      0,
      0,
      1,
      ...
      1,
      0,
      1,
      0,
      1,
      1,
      0,
      0,
      0,
      0
    ]

# Implement list

- [x] str_count

- [x] str_detect

- \[\] str_extract /str_extract_all

- \[\] str_locate() str_locate_all()

- \[\] str_match() str_match_all()

- [x] str_replace() str_replace_all()

- [x] str_remove() str_remove_all()

- \[\] str_split() str_split_1() str_split_fixed() str_split_i()

- \[\] str_starts() str_ends()

- \[\] str_subset()

- \[\] str_which()

- [x] str_c(), str_combine()

- \[\] str_flatten() str_flatten_comma()

- \[\] str_dup()

- \[\] str_length() str_width()

- \[\] str_pad()

- \[\] str_sub()/ str_sub_all()

- [x] str_trim() str_squish()

- [x] str_trunc()

- \[\] str_wrap()

- \[\] str_to_upper() str_to_lower() str_to_title() str_to_sentence()

- \[\] str_unique()

- [x] str_remove_ascent()

# Different type of i/o

## Python

- `@export`: one array in, one array out

- `@export2`: multiple array in, one array out

## Rust

- `apply_utf8!()`  
- `apply_utf8_bool!()`
- `apply_utf8_lst!()`

1.  vec<str> in vec<str> out

- Use apply_utf8!() macro
- @export

2.  vec<str>+ in vec<str> out

- Use apply_utf8!() macro
- @export2

3.  vec<str> in vec<bool> out

- Use apply_utf8_bool!() macro
- @export

4.  vec<str> in vec\<vec<str>\> out

- Use apply_utf8_lst!() macro
- @export
