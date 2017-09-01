from itertools import zip_longest

def float_or_none(x):
    try:
        return float(x)
    except ValueError:
        return None

def pairs(iterable):
    return grouped(iterable, 2)

def grouped(iterable, n):
    return zip_longest(*[iter(iterable)]*n, fillvalue=None)