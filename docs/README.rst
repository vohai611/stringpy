======
README
======

:Author: Hai Vo
:Authors:
   Hai Vo
:Date: 5/1/23

.. container::

   |image1|

Introduction
============

This project is a python package to mimic
`r::stringr <https://stringr.tidyverse.org/>`__ functionalities, the
core functions are written in Rust. Note that I write this package
mostly for personal use (convenience and speed) and learning purpose, so
please use with care!

Installation
============

This package is not on Pipy yet, so you need to compile from source.

First you need rust compiler:

::

   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

Then install this package as normal python package:

::

   git clone https://github.com/vohai611/stringpy.git
   pip3 install ./stringpy

Usage example
=============

.. container:: cell

   .. code:: python

      # setup
      import stringpy as sp
      import pandas as pd
      import numpy as np
      import random
      import string

Combine string within group
---------------------------

.. container:: cell

   .. code:: python

      df = pd.DataFrame({'group': ['a', 'b', 'a', 'b', 'a', 'b', 'a', 'b', 'a', 'b'],
                    'value': ['one', 'two', 'three', 'four',None, 'six', 'seven', 'eight', 'nine', 'ten']})

      df2 = df.groupby('group').agg(lambda x: sp.str_c(x, collapse='->'))

      df2

   .. container:: cell-output cell-output-display

      ===== ==========================
      \     value
      ===== ==========================
      group 
      a     one->three->->seven->nine
      b     two->four->six->eight->ten
      ===== ==========================

Split string
------------

.. container:: cell

   .. code:: python

      sp.str_split(df2['value'], pattern='->')

   .. container:: cell-output cell-output-display

      ::

         <pyarrow.lib.ListArray object at 0x122354e20>
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

Remove accent
-------------

.. container:: cell

   .. code:: python

      vietnam = ['Hà Nội', 'Hồ Chí Minh', 'Đà Nẵng', 'Hải Phòng', 'Cần Thơ', 'Biên Hòa', 'Nha Trang', 'BMT', 'Huế', 'Buôn Ma Thuột', 'Bắc Giang', 'Bắc Ninh', 'Bến Tre', 'Bình Dương', 'Bình Phước', 'Bình Thuận', 'Cà Mau', 'Cao Bằng', 'Đắk Lắk', 'Đắk Nông', 'Điện Biên', 'Đồng Nai', 'Đồng Tháp', 'Gia Lai', 'Hà Giang', 'Hà Nam', 'Hà Tĩnh', 'Hải Dương', 'Hậu Giang', 'Hòa Bình', 'Hưng Yên', 'Khánh Hòa', 'Kiên Giang', 'Kon Tum', 'Lai Châu', 'Lâm Đồng', 'Lạng Sơn', 'Lào Cai', 'Long An', 'Nam Định', 'Nghệ An', 'Ninh Bình', 'Ninh Thuận', 'Phú Thọ', 'Phú Yên', 'Quảng Bình', 'Quảng Nam', 'Quảng Ngãi', 'Quảng Ninh', 'Quảng Trị', 'Sóc Trăng', 'Sơn La'] 

      sp.str_remove_ascent(vietnam)

   .. container:: cell-output cell-output-display

      ::

         <pyarrow.lib.StringArray object at 0x113794ee0>
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

Random speed comparison
=======================

Although this package is not aim to speed optimization, but in most
case, it still get a decent speed up compare with pandas, thank to Rust!

Below are some of random comparison between ``stringpy`` and ``pandas``:

.. container:: cell

   .. code:: python

      letters = string.ascii_lowercase
      a = [''.join(random.choice(letters) for i in range(10))  for i in range(600_000)]

      a_sr = pd.Series(a)

Replace pattern
---------------

.. container:: cell

   .. code:: python

      %%time
      a_sr.str.replace('\w', 'b', regex=True)

   .. container:: cell-output cell-output-stdout

      ::

         CPU times: user 442 ms, sys: 12.5 ms, total: 454 ms
         Wall time: 476 ms

   .. container:: cell-output cell-output-display

      ::

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

.. container:: cell

   .. code:: python

      %%time
      sp.str_replace_all(a, pattern='\w', replace= 'b')

   .. container:: cell-output cell-output-stdout

      ::

         CPU times: user 265 ms, sys: 13.3 ms, total: 278 ms
         Wall time: 326 ms

   .. container:: cell-output cell-output-display

      ::

         <pyarrow.lib.StringArray object at 0x105e77a00>
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

Counting
--------

.. container:: cell

   .. code:: python

      %%time
      a_sr.str.count('a')

   .. container:: cell-output cell-output-stdout

      ::

         CPU times: user 141 ms, sys: 4.55 ms, total: 145 ms
         Wall time: 146 ms

   .. container:: cell-output cell-output-display

      ::

         0         0
         1         0
         2         0
         3         2
         4         0
                  ..
         599995    2
         599996    0
         599997    1
         599998    3
         599999    0
         Length: 600000, dtype: int64

.. container:: cell

   .. code:: python

      %%time
      sp.str_count(a, pattern='a')

   .. container:: cell-output cell-output-stdout

      ::

         CPU times: user 25.4 ms, sys: 1.53 ms, total: 26.9 ms
         Wall time: 29.9 ms

   .. container:: cell-output cell-output-display

      ::

         <pyarrow.lib.Int32Array object at 0x105e76080>
         [
           0,
           0,
           0,
           2,
           0,
           0,
           0,
           0,
           0,
           2,
           ...
           1,
           0,
           1,
           0,
           0,
           2,
           0,
           1,
           3,
           0
         ]

Implement list
==============

-  ☒ str_count

-  ☒ str_detect

-  ☒ str_extract /str_extract_all

-  [] str_locate() str_locate_all()

-  [] str_match() str_match_all()

-  ☒ str_replace() str_replace_all()

-  ☒ str_remove() str_remove_all()

-  [] str_split() str_split_1() str_split_fixed() str_split_i()

-  ☒ str_starts() str_ends()

-  [] str_subset()

-  [] str_which()

-  ☒ str_c(), str_combine()

-  [] str_flatten() str_flatten_comma()

-  [] str_dup()

-  [] str_length() str_width()

-  [] str_pad()

-  [] str_sub()/ str_sub_all()

-  ☒ str_trim() str_squish()

-  ☒ str_trunc()

-  [] str_wrap()

-  [] str_to_upper() str_to_lower() str_to_title() str_to_sentence()

-  [] str_unique()

-  ☒ str_remove_ascent()

Different type of i/o
=====================

Python
------

-  ``@export``: one array in, one array out

-  ``@export2``: multiple array in, one array out

Rust
----

-  ``apply_utf8!()``
-  ``apply_utf8_bool!()``
-  ``apply_utf8_lst!()``

1. vec in vec out

-  Use apply_utf8!() macro
-  @export

2. vec+ in vec out

-  Use apply_utf8!() macro
-  @export2

3. vec in vec out

-  Use apply_utf8_bool!() macro
-  @export

4. vec in vec<vec> out

-  Use apply_utf8_lst!() macro
-  @export

.. |image1| image:: https://github.com/vohai611/stringpy/actions/workflows/CI.yml/badge.svg?branch=main
   :target: https://github.com/vohai611/stringpy/actions/workflows/CI.yml
