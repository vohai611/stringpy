README
================
Hai Vo
7/21/23

- <a href="#introduction" id="toc-introduction">Introduction</a>
- <a href="#how-it-works" id="toc-how-it-works">How it works</a>
- <a href="#installation" id="toc-installation">Installation</a>
- <a href="#milestone" id="toc-milestone">Milestone</a>
- <a href="#documentation" id="toc-documentation">Documentation</a>
- <a href="#usage-example" id="toc-usage-example">Usage example</a>
- <a href="#random-speed-comparison"
  id="toc-random-speed-comparison">Random speed comparison</a>
- <a href="#implement-list" id="toc-implement-list">Implement list</a>
- <a href="#different-type-of-io" id="toc-different-type-of-io">Different
  type of i/o</a>

[![doc](https://readthedocs.org/projects/stringpy/badge/?version=latest.png)](https://stringpy.readthedocs.io/en/latest/?badge=latest)
[![build](https://github.com/vohai611/stringpy/actions/workflows/build.yml/badge.svg)](https://github.com/vohai611/stringpy/actions/workflows/build.yml)
[![codecov](https://codecov.io/gh/vohai611/stringpy/branch/main/graph/badge.svg?token=5QNSE2HMHM)](https://codecov.io/gh/vohai611/stringpy)

# Introduction

This project is a python package to mimic
[r::stringr](https://stringr.tidyverse.org/) functionalities, the core
functions are written in Rust and then export to Python. Note that I
write this package mostly for personal use (convenience and speed) and
learning purpose, so please use with care!

Any type of contribution are welcome!

# How it works

- Using arrow format to store main input array.
- Using pyo3 for python binding
- Convert Python type (mostly List) to Rust type (mostly Vec) for the
  case not using arrow. This may cause some overhead, but it make the
  code more flexible. For example: many function not only vectorize over
  main array but also it arugments.

# Installation

This package is not on PyPi yet, so you need to compile from source.

First you need rust compiler:

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Then install this package as normal python package:

    git clone https://github.com/vohai611/stringpy.git
    pip3 install ./stringpy

Or you can download and install from **prebuild** wheels under [github
action artifact](https://github.com/vohai611/stringpy/actions)

# Milestone

## v0.1.0

- [x] Implement basic function
- [x] Add document
- [x] Add test
- [x] Add CI/CD
- [x] Add example
- [x] Add codecov
- \[\] Release PyPi

## v0.2.0

- \[\] Add benchmark
- \[\] Vectorize on arguments

# Documentation

The documentation can be found at
[here](https://stringpy.readthedocs.io/en/latest/)

# Usage example

<details>
<summary>Code</summary>

``` python
# setup
import stringpy as sp
import pandas as pd
import numpy as np
import random
import string
```

</details>

## Combine string within group

<details>
<summary>Code</summary>

``` python
df = pd.DataFrame({'group': ['a', 'b', 'a', 'b', 'a', 'b', 'a', 'b', 'a', 'b'],
              'value': ['one', 'two', 'three', 'four',None, 'six', 'seven', 'eight', 'nine', 'ten']})

df2 = df.groupby('group').agg(lambda x: sp.str_c(x, collapse='->'))

df2
```

</details>
<div>
<style scoped>
    .dataframe tbody tr th:only-of-type {
        vertical-align: middle;
    }

    .dataframe tbody tr th {
        vertical-align: top;
    }

    .dataframe thead th {
        text-align: right;
    }
</style>
<table border="1" class="dataframe">
  <thead>
    <tr style="text-align: right;">
      <th></th>
      <th>value</th>
    </tr>
    <tr>
      <th>group</th>
      <th></th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <th>a</th>
      <td>one-&gt;three-&gt;-&gt;seven-&gt;nine</td>
    </tr>
    <tr>
      <th>b</th>
      <td>two-&gt;four-&gt;six-&gt;eight-&gt;ten</td>
    </tr>
  </tbody>
</table>
</div>

## Split string

<details>
<summary>Code</summary>

``` python
sp.str_split(df2['value'], pattern='->')
```

</details>

    <pyarrow.lib.ListArray object at 0x134b45360>
    [
      [
        "one",
        "three",
        "",
        "seven",
        "nine"
      ],
      [
        "two",
        "four",
        "six",
        "eight",
        "ten"
      ]
    ]

## Camel case to snake case

<details>
<summary>Code</summary>

``` python
a = sp.str_replace_all(['ThisIsSomeCamelCase', 'ObjectNotFound'],
                      pattern='([a-z])([A-Z])', replace= '$1 $2').to_pylist() 
sp.str_replace_all(sp.str_to_lower(a), pattern = ' ', replace = '_')
```

</details>

    <pyarrow.lib.StringArray object at 0x104077e20>
    [
      "this_is_some_camel_case",
      "object_not_found"
    ]

## Remove accent

<details>
<summary>Code</summary>

``` python
vietnam = ['Hà Nội', 'Hồ Chí Minh', 'Đà Nẵng', 'Hải Phòng', 'Cần Thơ', 'Biên Hòa', 'Nha Trang', 'BMT', 'Huế', 'Buôn Ma Thuột', 'Bắc Giang', 'Bắc Ninh', 'Bến Tre', 'Bình Dương', 'Bình Phước', 'Bình Thuận', 'Cà Mau', 'Cao Bằng', 'Đắk Lắk', 'Đắk Nông', 'Điện Biên', 'Đồng Nai', 'Đồng Tháp'] 

sp.str_remove_ascent(vietnam)
```

</details>

    <pyarrow.lib.StringArray object at 0x134b45de0>
    [
      "Ha Noi",
      "Ho Chi Minh",
      "Da Nang",
      "Hai Phong",
      "Can Tho",
      "Bien Hoa",
      "Nha Trang",
      "BMT",
      "Hue",
      "Buon Ma Thuot",
      ...
      "Binh Duong",
      "Binh Phuoc",
      "Binh Thuan",
      "Ca Mau",
      "Cao Bang",
      "Dak Lak",
      "Dak Nong",
      "Dien Bien",
      "Dong Nai",
      "Dong Thap"
    ]

# Random speed comparison

Although this package is not aim to speed optimization, but in most
case, it still get a decent speed up compare with pandas, thank to Rust!

Below are some of random comparison between `stringpy` and `pandas`:

<details>
<summary>Code</summary>

``` python
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

    CPU times: user 447 ms, sys: 7.09 ms, total: 454 ms
    Wall time: 454 ms

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

    CPU times: user 4.95 s, sys: 27 ms, total: 4.98 s
    Wall time: 4.98 s

    <pyarrow.lib.StringArray object at 0x104077ca0>
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

## Subset by index

<details>
<summary>Code</summary>

``` python
%%time
a_sr.str.slice(2,4)
```

</details>

    CPU times: user 55.7 ms, sys: 4.04 ms, total: 59.8 ms
    Wall time: 59.7 ms

    0         az
    1         aj
    2         qr
    3         wr
    4         ao
              ..
    599995    ky
    599996    tn
    599997    dj
    599998    dg
    599999    ny
    Length: 600000, dtype: object

<details>
<summary>Code</summary>

``` python
%%time
sp.str_sub(a, start=2, end=4)
```

</details>

    CPU times: user 272 ms, sys: 4.64 ms, total: 277 ms
    Wall time: 276 ms

    <pyarrow.lib.StringArray object at 0x134b44100>
    [
      "az",
      "aj",
      "qr",
      "wr",
      "ao",
      "ds",
      "ef",
      "br",
      "pi",
      "dg",
      ...
      "ps",
      "mn",
      "mm",
      "dt",
      "co",
      "ky",
      "tn",
      "dj",
      "dg",
      "ny"
    ]

    ## Counting

    ::: {.cell execution_count=11}
    ``` {.python .cell-code}
    %%time
    a_sr.str.count('a')

<div class="cell-output cell-output-stdout">

    CPU times: user 132 ms, sys: 3.08 ms, total: 135 ms
    Wall time: 135 ms

</div>

<div class="cell-output cell-output-display" execution_count="23">

    0         2
    1         1
    2         0
    3         1
    4         1
             ..
    599995    2
    599996    0
    599997    0
    599998    1
    599999    0
    Length: 600000, dtype: int64

</div>

:::

<details>
<summary>Code</summary>

``` python
%%time
sp.str_count(a, pattern='a')
```

</details>

    CPU times: user 427 ms, sys: 2.26 ms, total: 430 ms
    Wall time: 430 ms

    <pyarrow.lib.Int32Array object at 0x134b458a0>
    [
      2,
      1,
      0,
      1,
      1,
      0,
      1,
      0,
      1,
      1,
      ...
      1,
      0,
      0,
      0,
      0,
      2,
      0,
      0,
      1,
      0
    ]

# Implement list

## part 1

- [x] str_count

- [x] str_detect

- [x] str_extract /str_extract_all

- \[\] str_locate() str_locate_all()

- [x] str_match() str_match_all()

- [x] str_replace() str_replace_all()

- [x] str_remove() str_remove_all()

- [x] str_split()

- \[\] str_split_1() str_split_fixed() str_split_i()

- [x] str_starts() str_ends()

- [x] str_subset()

- [x] str_which()

- [x] str_c(), str_combine()

- \[\] str_flatten() str_flatten_comma()

## part 2

- [x] str_dup()
- [x] str_length() str_width()
- [x] str_pad()
- [x] str_sub()/ str_sub_all()
- [x] str_trim() str_squish()
- [x] str_trunc()
- \[\] str_wrap()
- [x] str_to_upper() str_to_lower() str_to_title() str_to_sentence()
- [x] str_unique()
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

- apply_utf8!()
- @export

2.  vec<str>+ in vec<str> out

- apply_utf8!()
- @export2

3.  vec<str> in vec<bool> out

- apply_utf8_bool!()
- @export

4.  vec<str> in vec\<vec<str>\> out

- apply_utf8_lst!()
- @export
