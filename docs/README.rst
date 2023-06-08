======
README
======

:Author: Hai Vo
:Authors:
   Hai Vo
:Date: Invalid Date

|Documentation Status| |image1|

Introduction
============

This project is a python package to mimic
`r::stringr <https://stringr.tidyverse.org/>`__ functionalities, the
core functions are written in Rust. Note that I write this package
mostly for personal use (convenience and speed) and learning purpose, so
please use with care!

How it works
============

-  Using arrow for data structure
-  Using pyo3 for python binding
-  Convert Python type (mostly List) to Rust type (mostly Vec) for the
   case not using arrow. This may cause some overhead, but it make the
   code more flexible. For example: many function not only vectorize
   over main array but also it arugments.

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

         <pyarrow.lib.ListArray object at 0x11df8de40>
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

Camel case to snake case
------------------------

.. container:: cell

   .. code:: python

      a = sp.str_replace_all(['ThisIsSomeCamelCase', 'ObjectNotFound'],
                            pattern='([a-z])([A-Z])', replace= '$1 $2').to_pylist() 
      sp.str_replace_all(sp.str_to_lower(a), pattern = ' ', replace = '_')

   .. container:: cell-output cell-output-display

      ::

         <pyarrow.lib.StringArray object at 0x11df8e020>
         [
           "this_is_some_camel_case",
           "object_not_found"
         ]

Remove accent
-------------

.. container:: cell

   .. code:: python

      vietnam = ['Hà Nội', 'Hồ Chí Minh', 'Đà Nẵng', 'Hải Phòng', 'Cần Thơ', 'Biên Hòa', 'Nha Trang', 'BMT', 'Huế', 'Buôn Ma Thuột', 'Bắc Giang', 'Bắc Ninh', 'Bến Tre', 'Bình Dương', 'Bình Phước', 'Bình Thuận', 'Cà Mau', 'Cao Bằng', 'Đắk Lắk', 'Đắk Nông', 'Điện Biên', 'Đồng Nai', 'Đồng Tháp'] 

      sp.str_remove_ascent(vietnam)

   .. container:: cell-output cell-output-display

      ::

         <pyarrow.lib.StringArray object at 0x11e00d4e0>
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

         CPU times: user 435 ms, sys: 5.16 ms, total: 440 ms
         Wall time: 440 ms

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

         CPU times: user 233 ms, sys: 3.93 ms, total: 237 ms
         Wall time: 237 ms

   .. container:: cell-output cell-output-display

      ::

         <pyarrow.lib.StringArray object at 0x11e00da80>
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

Subset by index
---------------

.. container:: cell

   .. code:: python

      %%time
      a_sr.str.slice(2,4)

   .. container:: cell-output cell-output-stdout

      ::

         CPU times: user 53.8 ms, sys: 3.6 ms, total: 57.4 ms
         Wall time: 57.2 ms

   .. container:: cell-output cell-output-display

      ::

         0         ry
         1         qb
         2         uq
         3         ih
         4         zd
                   ..
         599995    ql
         599996    ot
         599997    nj
         599998    ek
         599999    lf
         Length: 600000, dtype: object

.. container:: cell

   .. code:: python

      %%time
      sp.str_sub(a, start=2, end=4)

   .. container:: cell-output cell-output-stdout

      ::

         CPU times: user 24.2 ms, sys: 4.04 ms, total: 28.2 ms
         Wall time: 30.3 ms

   .. container:: cell-output cell-output-display

      ::

         <pyarrow.lib.StringArray object at 0x11e00e740>
         [
           "ry",
           "qb",
           "uq",
           "ih",
           "zd",
           "so",
           "jr",
           "rt",
           "bb",
           "zr",
           ...
           "fh",
           "gk",
           "mx",
           "io",
           "ez",
           "ql",
           "ot",
           "nj",
           "ek",
           "lf"
         ]

::

   ## Counting

   ::: {.cell execution_count=11}
   ``` {.python .cell-code}
   %%time
   a_sr.str.count('a')

.. container:: cell-output cell-output-stdout

   ::

      CPU times: user 131 ms, sys: 1.92 ms, total: 133 ms
      Wall time: 133 ms

.. container:: cell-output cell-output-display

   ::

      0         2
      1         1
      2         0
      3         0
      4         1
               ..
      599995    0
      599996    1
      599997    0
      599998    0
      599999    0
      Length: 600000, dtype: int64

:::

.. container:: cell

   .. code:: python

      %%time
      sp.str_count(a, pattern='a')

   .. container:: cell-output cell-output-stdout

      ::

         CPU times: user 23.3 ms, sys: 443 µs, total: 23.7 ms
         Wall time: 23.7 ms

   .. container:: cell-output cell-output-display

      ::

         <pyarrow.lib.Int32Array object at 0x11e00db40>
         [
           2,
           1,
           0,
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
           1,
           0,
           1,
           0,
           1,
           0,
           0,
           0
         ]

Implement list
==============

part 1
------

-  ☒ str_count

-  ☒ str_detect

-  ☒ str_extract /str_extract_all

-  [] str_locate() str_locate_all()

-  ☒ str_match() str_match_all()

-  ☒ str_replace() str_replace_all()

-  ☒ str_remove() str_remove_all()

-  ☒ str_split()

-  [] str_split_1() str_split_fixed() str_split_i()

-  ☒ str_starts() str_ends()

-  ☒ str_subset()

-  ☒ str_which()

-  ☒ str_c(), str_combine()

-  [] str_flatten() str_flatten_comma()

part 2
------

-  ☒ str_dup()
-  ☒ str_length() str_width()
-  ☒ str_pad()
-  ☒ str_sub()/ str_sub_all()
-  ☒ str_trim() str_squish()
-  ☒ str_trunc()
-  [] str_wrap()
-  ☒ str_to_upper() str_to_lower() str_to_title() str_to_sentence()
-  ☒ str_unique()
-  ☒ str_remove_ascent()

Optimize
--------

Handle case when input is scalar or vector in Rust to improve speed

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

.. |Documentation Status| image:: https://readthedocs.org/projects/stringpy/badge/?version=latest.png
   :target: https://stringpy.readthedocs.io/en/latest/?badge=latest
.. |image1| image:: https://github.com/vohai611/stringpy/actions/workflows/CI.yml/badge.svg?branch=main
   :target: https://github.com/vohai611/stringpy/actions/workflows/CI.yml
