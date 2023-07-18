README
================
Hai Vo
5/1/23

- <a href="#introduction" id="toc-introduction">Introduction</a>
- <a href="#how-it-works" id="toc-how-it-works">How it works</a>
- <a href="#installation" id="toc-installation">Installation</a>
- <a href="#usage-example" id="toc-usage-example">Usage example</a>
- <a href="#random-speed-comparison"
  id="toc-random-speed-comparison">Random speed comparison</a>
- <a href="#implement-list" id="toc-implement-list">Implement list</a>
- <a href="#different-type-of-io" id="toc-different-type-of-io">Different
  type of i/o</a>

[![doc](https://readthedocs.org/projects/stringpy/badge/?version=latest.png)](https://stringpy.readthedocs.io/en/latest/?badge=latest)
[![](https://github.com/vohai611/stringpy/actions/workflows/CI.yml/badge.svg?branch=main)](https://github.com/vohai611/stringpy/actions/workflows/CI.yml)
[![codecov](https://codecov.io/gh/vohai611/stringpy/branch/main/graph/badge.svg?token=5QNSE2HMHM)](https://codecov.io/gh/vohai611/stringpy)

# Introduction

This project is a python package to mimic
[r::stringr](https://stringr.tidyverse.org/) functionalities, the core
functions are written in Rust. Note that I write this package mostly for
personal use (convenience and speed) and learning purpose, so please use
with care!

# How it works

- Using arrow for data structure
- Using pyo3 for python binding
- Convert Python type (mostly List) to Rust type (mostly Vec) for the
  case not using arrow. This may cause some overhead, but it make the
  code more flexible. For example: many function not only vectorize over
  main array but also it arugments.

# Installation

This package is not on Pipy yet, so you need to compile from source.

First you need rust compiler:

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Then install this package as normal python package:

    git clone https://github.com/vohai611/stringpy.git
    pip3 install ./stringpy

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

    <pyarrow.lib.ListArray object at 0x1077ebfa0>
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

    <pyarrow.lib.StringArray object at 0x1077eb160>
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

    <pyarrow.lib.StringArray object at 0x1077ebb80>
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

    CPU times: user 444 ms, sys: 6.47 ms, total: 451 ms
    Wall time: 450 ms

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

    CPU times: user 234 ms, sys: 4.66 ms, total: 238 ms
    Wall time: 238 ms

    <pyarrow.lib.StringArray object at 0x1077ebe20>
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

    CPU times: user 54.4 ms, sys: 3.95 ms, total: 58.4 ms
    Wall time: 58.2 ms

    0         lw
    1         jg
    2         vb
    3         vc
    4         jd
              ..
    599995    sm
    599996    eb
    599997    kh
    599998    zh
    599999    pq
    Length: 600000, dtype: object

<details>
<summary>Code</summary>

``` python
%%time
sp.str_sub(a, start=2, end=4)
```

</details>

    CPU times: user 23.9 ms, sys: 2.7 ms, total: 26.7 ms
    Wall time: 26.6 ms

    <pyarrow.lib.StringArray object at 0x14a175480>
    [
      "lw",
      "jg",
      "vb",
      "vc",
      "jd",
      "mx",
      "aa",
      "nx",
      "gf",
      "uo",
      ...
      "bi",
      "fv",
      "er",
      "fu",
      "bz",
      "sm",
      "eb",
      "kh",
      "zh",
      "pq"
    ]

    ## Counting

    ::: {.cell execution_count=11}
    ``` {.python .cell-code}
    %%time
    a_sr.str.count('a')

<div class="cell-output cell-output-stdout">

    CPU times: user 132 ms, sys: 2.52 ms, total: 134 ms
    Wall time: 134 ms

</div>

<div class="cell-output cell-output-display" execution_count="23">

    0         1
    1         0
    2         1
    3         0
    4         1
             ..
    599995    0
    599996    0
    599997    1
    599998    0
    599999    1
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

    CPU times: user 35 ms, sys: 861 µs, total: 35.8 ms
    Wall time: 35.8 ms

    <pyarrow.lib.Int32Array object at 0x14a1751e0>
    [
      1,
      0,
      1,
      0,
      1,
      0,
      2,
      0,
      0,
      0,
      ...
      0,
      0,
      0,
      1,
      0,
      0,
      0,
      1,
      0,
      1
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

## Optimize

Handle case when input is scalar or vector in Rust to improve speed

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
