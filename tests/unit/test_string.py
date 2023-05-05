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


def test_str_trunc():
    actual = sp.str_trunc(['toi muon mot giac ngu',
                           'doi bung qua roi nhi',
                           None], width=8).to_pylist()
    expect = ['toi muon...','doi bung...', None]

    assert actual == expect

def test_str_extract():
    actual = sp.str_extract(["apples x4", "bag of flour", "bag of sugar", "nevermind2"], pattern = '\\d').to_pylist()
    expect = ['4', None, None, "2"]
    assert actual == expect 

    actual = sp.str_extract(["apples x4x", "bag of flour", "bag of sugar", "nevermind2"], pattern = '\\d(.)', group = 1).to_pylist()
    expect = ['x', None, None, None]
    assert actual == expect 

def test_str_extract_all():

    actual = sp.str_extract_all(["apples x4 t6", "bag 3of flour",  "1", "ads", None], pattern = '\\d').to_pylist()
    actual
    expect = [['4', '6'], ['3'], ['1'], [], [None]]
    assert actual == expect

    actual = sp.str_extract_all([None, None, None, '123'], pattern = '\\d').to_pylist()
    actual
    expect = [[None], [None], [None], ['1', '2', '3']]

def test_str_split():
    actual = sp.str_split([None,'a,b,c','de,f', None], pattern =',').to_pylist()
    expect  = [[None],['a', 'b', 'c'],['de', 'f'], [None]]
    assert actual == expect
