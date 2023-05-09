[![Documentation Status](https://readthedocs.org/projects/stringpy/badge/?version=latest)](https://stringpy.readthedocs.io/en/latest/?badge=latest)

# Introduction

This project is a python package to mimic r::stringr

# Installation

This package is not on Pipy yet, so you need to compile from source. 

First you need rust compiler:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then install this package as normal python package:
```
git clone https://github.com/vohai611/stringpy.git
pip3 install ./stringpy
```

# Usage example

# Speed comparison
# Implement list
- [x] str_count
- [x] str_detect
- [] str_extract /str_extract_all
- [] str_locate() str_locate_all()
- [] str_match() str_match_all()
- [x] str_replace() str_replace_all()
- [x] str_remove() str_remove_all()
- [] str_split() str_split_1() str_split_fixed() str_split_i()
- [x] str_starts() str_ends()

- [] str_subset()
- [] str_which()

- [x] str_c(), str_combine()
- [] str_flatten() str_flatten_comma()

- [] str_dup()
- [] str_length() str_width()
- [] str_pad()
- [] str_sub()/  str_sub_all()
- [x] str_trim() str_squish()
- [x] str_trunc()
- [] str_wrap()

- [] str_to_upper() str_to_lower() str_to_title() str_to_sentence()
- [] str_unique()
- [x] str_remove_ascent()

# Different type of i/o

## Python
- `@export`: one array in, one array out

- `@export2`: multiple array in, one array out

## Rust
- `apply_utf8!()`   
- `apply_utf8_bool!()`
- `apply_utf8_lst!()`

1. vec<str> in vec<str> out
- Use apply_utf8!() macro
- @export

2. vec<str>+ in vec<str> out
- Use apply_utf8!() macro
- @export2

3. vec<str> in vec<bool> out
- Use apply_utf8_bool!() macro
- @export

4. vec<str> in vec<vec<str>> out
- Use apply_utf8_lst!() macro
- @export
