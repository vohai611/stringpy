import stringpy as sp
import pyarrow as pa


def test_str_c():
    assert sp.str_c(pa.array(['a', 'b', 'c']), collapse='-->') == 'a-->b-->c'


def test_str_combine():
    assert sp.str_combine(pa.array(['a', 'b', 'c']),
                          pa.array(['a', 'b', 'c']),
                          pa.array(['a', 'b', 'c']),
                          pa.array(['a', 'b', 'c']),
                          sep='<->') == ['a<->a<->a<->a', 'b<->b<->b<->b', 'c<->c<->c<->c']


def test_str_count():
    actual = sp.str_count(['aa', 'bb', ' cc', None], pattern = '\\w').to_pylist()
    expect = [2, 2, 2, None]
    assert actual == expect
    actual = sp.str_count([None, 'a', 'b'], pattern='a').to_pylist()
    expect = [None, 1, 0]
    assert actual == expect

def test_str_remove_ascent():
    actual = sp.str_remove_ascent(['tôi thấy mệt mỏi', 'hôm nay trời nắng', None]).to_pylist()
    expect = ['toi thay met moi', 'hom nay troi nang', None]
    assert actual == expect

def test_str_replace():
    actual = sp.str_replace(pa.array(['aa', 'bb', 'cc']), pattern= 'a', replace= 'b').to_pylist()
    expect = ['ba', 'bb', 'cc']
    assert actual == expect


def test_str_replace_all():
    actual = sp.str_replace_all(['aa', 'bb', 'cc', None], pattern='a', replace= 'b').to_pylist()
    expect = ['bb', 'bb', 'cc', None]
    assert actual == expect


def test_str_trim():
    actual = sp.str_trim(['  aa ', ' bb  ', '  cc  ']).to_pylist()
    expect = ['aa', 'bb', 'cc']
    assert actual == expect

    actual = sp.str_trim(['  aa ', ' bb  ', '  cc  '], side = 'left').to_pylist()
    expect = ['aa ', 'bb  ', 'cc  ']
    assert actual == expect

    actual = sp.str_trim(['  aa ', ' bb  ', '  cc  '], side = 'right').to_pylist()
    expect = ['  aa', ' bb', '  cc']
    assert actual == expect

def test_str_detect():
    actual = sp.str_detect(['aa', 'bb', 'cc', None], pattern='a').to_pylist()
    expect = [True, False, False, None]
    assert actual == expect

