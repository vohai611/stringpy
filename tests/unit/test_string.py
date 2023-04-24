import stringpy as sp
import pyarrow as pa

def test_str_c():
    assert sp.str_c(pa.array(['a', 'b', 'c']), collapse = '-->') == 'a-->b-->c'

def test_str_combine():
    assert sp.str_combine(pa.array(['a', 'b', 'c']), 
                    pa.array(['a', 'b', 'c']),
                    pa.array(['a', 'b', 'c']),
                    pa.array(['a', 'b', 'c']),
                    sep = '<->') == ['a<->a<->a<->a', 'b<->b<->b<->b', 'c<->c<->c<->c'] 
