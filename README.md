README
================
Hai Vo
5/1/23

- <a href="#introduction" id="toc-introduction">Introduction</a>
- <a href="#installation" id="toc-installation">Installation</a>
- <a href="#usage-example" id="toc-usage-example">Usage example</a>
- <a href="#random-speed-comparison"
  id="toc-random-speed-comparison">Random speed comparison</a>
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

    <pyarrow.lib.ListArray object at 0x1289ae3e0>
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

## Remove accent

<details>
<summary>Code</summary>

``` python
vietnam = ['Hà Nội', 'Hồ Chí Minh', 'Đà Nẵng', 'Hải Phòng', 'Cần Thơ', 'Biên Hòa', 'Nha Trang', 'BMT', 'Huế', 'Buôn Ma Thuột', 'Bắc Giang', 'Bắc Ninh', 'Bến Tre', 'Bình Dương', 'Bình Phước', 'Bình Thuận', 'Cà Mau', 'Cao Bằng', 'Đắk Lắk', 'Đắk Nông', 'Điện Biên', 'Đồng Nai', 'Đồng Tháp', 'Gia Lai', 'Hà Giang', 'Hà Nam', 'Hà Tĩnh', 'Hải Dương', 'Hậu Giang', 'Hòa Bình', 'Hưng Yên', 'Khánh Hòa', 'Kiên Giang', 'Kon Tum', 'Lai Châu', 'Lâm Đồng', 'Lạng Sơn', 'Lào Cai', 'Long An', 'Nam Định', 'Nghệ An', 'Ninh Bình', 'Ninh Thuận', 'Phú Thọ', 'Phú Yên', 'Quảng Bình', 'Quảng Nam', 'Quảng Ngãi', 'Quảng Ninh', 'Quảng Trị', 'Sóc Trăng', 'Sơn La'] 

sp.str_remove_ascent(vietnam)
```

</details>

    <pyarrow.lib.StringArray object at 0x12ad28220>
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
      "Ninh Thuan",
      "Phu Tho",
      "Phu Yen",
      "Quang Binh",
      "Quang Nam",
      "Quang Ngai",
      "Quang Ninh",
      "Quang Tri",
      "Soc Trang",
      "Son La"
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

    CPU times: user 434 ms, sys: 9.08 ms, total: 443 ms
    Wall time: 452 ms

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

    CPU times: user 253 ms, sys: 3.95 ms, total: 256 ms
    Wall time: 257 ms

    <pyarrow.lib.StringArray object at 0x128a297e0>
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

    CPU times: user 137 ms, sys: 3.39 ms, total: 141 ms
    Wall time: 141 ms

    0         0
    1         0
    2         0
    3         0
    4         0
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

    CPU times: user 23.7 ms, sys: 712 µs, total: 24.4 ms
    Wall time: 24.3 ms

    <pyarrow.lib.Int32Array object at 0x12ad2a140>
    [
      0,
      0,
      0,
      0,
      0,
      1,
      1,
      0,
      1,
      0,
      ...
      0,
      0,
      0,
      1,
      0,
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
