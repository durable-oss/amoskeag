"""
Amoskeag Python Runtime

This module provides runtime support for Amoskeag programs transpiled to Python.
It includes helper functions and standard library equivalents that match the
Amoskeag functional programming semantics.
"""

from typing import Any, Dict, List, Optional, Union
import math


def get_nested(obj: Any, *keys: str) -> Any:
    """
    Safely navigate nested dictionaries.

    Returns None if any key is not found or if the current value is not a dict.
    This implements Amoskeag's safe navigation semantics.
    """
    current = obj
    for key in keys:
        if isinstance(current, dict):
            current = current.get(key)
        else:
            return None
        if current is None:
            return None
    return current


def is_truthy(val: Any) -> bool:
    """
    Check if a value is truthy in Amoskeag semantics.

    - None (nil) is falsy
    - Boolean False is falsy
    - Everything else is truthy
    """
    if val is None:
        return False
    if isinstance(val, bool):
        return val
    return True


# String Functions

def upcase(s: Any) -> str:
    """Convert string to uppercase."""
    if not isinstance(s, str):
        raise TypeError(f"upcase expects a string, got {type(s).__name__}")
    return s.upper()


def downcase(s: Any) -> str:
    """Convert string to lowercase."""
    if not isinstance(s, str):
        raise TypeError(f"downcase expects a string, got {type(s).__name__}")
    return s.lower()


def capitalize(s: Any) -> str:
    """Capitalize the first letter of a string."""
    if not isinstance(s, str):
        raise TypeError(f"capitalize expects a string, got {type(s).__name__}")
    return s.capitalize()


def strip(s: Any) -> str:
    """Remove leading and trailing whitespace."""
    if not isinstance(s, str):
        raise TypeError(f"strip expects a string, got {type(s).__name__}")
    return s.strip()


def split(s: Any, sep: Any) -> List[str]:
    """Split a string by separator."""
    if not isinstance(s, str):
        raise TypeError(f"split expects a string, got {type(s).__name__}")
    if not isinstance(sep, str):
        raise TypeError(f"split separator must be a string, got {type(sep).__name__}")
    return s.split(sep)


def join(arr: Any, sep: Any) -> str:
    """Join an array of strings with a separator."""
    if not isinstance(arr, list):
        raise TypeError(f"join expects a list, got {type(arr).__name__}")
    if not isinstance(sep, str):
        raise TypeError(f"join separator must be a string, got {type(sep).__name__}")
    return sep.join(str(x) for x in arr)


def truncate(s: Any, length: Any) -> str:
    """Truncate a string to a maximum length."""
    if not isinstance(s, str):
        raise TypeError(f"truncate expects a string, got {type(s).__name__}")
    if not isinstance(length, (int, float)):
        raise TypeError(f"truncate length must be a number, got {type(length).__name__}")
    return s[:int(length)]


def replace(s: Any, find: Any, replacement: Any) -> str:
    """Replace all occurrences of find with replacement."""
    if not isinstance(s, str):
        raise TypeError(f"replace expects a string, got {type(s).__name__}")
    if not isinstance(find, str):
        raise TypeError(f"replace find must be a string, got {type(find).__name__}")
    if not isinstance(replacement, str):
        raise TypeError(f"replace replacement must be a string, got {type(replacement).__name__}")
    return s.replace(find, replacement)


# Numeric Functions

def abs_num(n: Any) -> float:
    """Return the absolute value of a number."""
    if not isinstance(n, (int, float)):
        raise TypeError(f"abs expects a number, got {type(n).__name__}")
    return abs(n)


def ceil_num(n: Any) -> float:
    """Round a number up to the nearest integer."""
    if not isinstance(n, (int, float)):
        raise TypeError(f"ceil expects a number, got {type(n).__name__}")
    return math.ceil(n)


def floor_num(n: Any) -> float:
    """Round a number down to the nearest integer."""
    if not isinstance(n, (int, float)):
        raise TypeError(f"floor expects a number, got {type(n).__name__}")
    return math.floor(n)


def round_num(n: Any, digits: Any = 0) -> float:
    """Round a number to a specified number of decimal places."""
    if not isinstance(n, (int, float)):
        raise TypeError(f"round expects a number, got {type(n).__name__}")
    if not isinstance(digits, (int, float)):
        raise TypeError(f"round digits must be a number, got {type(digits).__name__}")
    return round(n, int(digits))


def plus(a: Any, b: Any) -> float:
    """Add two numbers."""
    if not isinstance(a, (int, float)):
        raise TypeError(f"plus expects numbers, got {type(a).__name__}")
    if not isinstance(b, (int, float)):
        raise TypeError(f"plus expects numbers, got {type(b).__name__}")
    return a + b


def minus(a: Any, b: Any) -> float:
    """Subtract two numbers."""
    if not isinstance(a, (int, float)):
        raise TypeError(f"minus expects numbers, got {type(a).__name__}")
    if not isinstance(b, (int, float)):
        raise TypeError(f"minus expects numbers, got {type(b).__name__}")
    return a - b


def times(a: Any, b: Any) -> float:
    """Multiply two numbers."""
    if not isinstance(a, (int, float)):
        raise TypeError(f"times expects numbers, got {type(a).__name__}")
    if not isinstance(b, (int, float)):
        raise TypeError(f"times expects numbers, got {type(b).__name__}")
    return a * b


def divided_by(a: Any, b: Any) -> float:
    """Divide two numbers."""
    if not isinstance(a, (int, float)):
        raise TypeError(f"divided_by expects numbers, got {type(a).__name__}")
    if not isinstance(b, (int, float)):
        raise TypeError(f"divided_by expects numbers, got {type(b).__name__}")
    if b == 0:
        raise ZeroDivisionError("Division by zero")
    return a / b


def modulo(a: Any, b: Any) -> float:
    """Calculate modulo of two numbers."""
    if not isinstance(a, (int, float)):
        raise TypeError(f"modulo expects numbers, got {type(a).__name__}")
    if not isinstance(b, (int, float)):
        raise TypeError(f"modulo expects numbers, got {type(b).__name__}")
    return a % b


# Collection Functions

def size(val: Any) -> int:
    """Get the size/length of a collection or string."""
    if val is None:
        return 0
    if isinstance(val, (list, dict, str)):
        return len(val)
    raise TypeError(f"size expects a collection or string, got {type(val).__name__}")


def first(arr: Any) -> Any:
    """Get the first element of an array."""
    if not isinstance(arr, list):
        raise TypeError(f"first expects a list, got {type(arr).__name__}")
    return arr[0] if arr else None


def last(arr: Any) -> Any:
    """Get the last element of an array."""
    if not isinstance(arr, list):
        raise TypeError(f"last expects a list, got {type(arr).__name__}")
    return arr[-1] if arr else None


def contains(arr: Any, val: Any) -> bool:
    """Check if an array contains a value."""
    if not isinstance(arr, list):
        raise TypeError(f"contains expects a list, got {type(arr).__name__}")
    return val in arr


def sum_arr(arr: Any) -> float:
    """Sum all numbers in an array."""
    if not isinstance(arr, list):
        raise TypeError(f"sum expects a list, got {type(arr).__name__}")
    return sum(arr)


def avg(arr: Any) -> Optional[float]:
    """Calculate the average of numbers in an array."""
    if not isinstance(arr, list):
        raise TypeError(f"avg expects a list, got {type(arr).__name__}")
    if not arr:
        return None
    return sum(arr) / len(arr)


def sort_arr(arr: Any) -> List[Any]:
    """Sort an array."""
    if not isinstance(arr, list):
        raise TypeError(f"sort expects a list, got {type(arr).__name__}")
    return sorted(arr)


def keys(d: Any) -> List[str]:
    """Get the keys of a dictionary."""
    if not isinstance(d, dict):
        raise TypeError(f"keys expects a dictionary, got {type(d).__name__}")
    return list(d.keys())


def values(d: Any) -> List[Any]:
    """Get the values of a dictionary."""
    if not isinstance(d, dict):
        raise TypeError(f"values expects a dictionary, got {type(d).__name__}")
    return list(d.values())


def reverse(arr: Any) -> List[Any]:
    """Reverse an array."""
    if not isinstance(arr, list):
        raise TypeError(f"reverse expects a list, got {type(arr).__name__}")
    return list(reversed(arr))


def at(arr: Any, index: Any) -> Any:
    """Get element at index (0-based)."""
    if not isinstance(arr, list):
        raise TypeError(f"at expects a list, got {type(arr).__name__}")
    if not isinstance(index, (int, float)):
        raise TypeError(f"at index must be a number, got {type(index).__name__}")
    idx = int(index)
    return arr[idx] if 0 <= idx < len(arr) else None


# Logic Functions

def choose(index: Any, arr: Any) -> Any:
    """
    Select an item from a list by 1-based index (Excel-style).

    This matches Excel's CHOOSE function behavior.
    """
    if not isinstance(index, (int, float)):
        raise TypeError(f"choose index must be a number, got {type(index).__name__}")
    if not isinstance(arr, list):
        raise TypeError(f"choose expects a list, got {type(arr).__name__}")

    idx = int(index) - 1  # Convert to 0-based
    return arr[idx] if 0 <= idx < len(arr) else None


def if_then_else(condition: Any, then_val: Any, else_val: Any) -> Any:
    """Conditional expression."""
    return then_val if is_truthy(condition) else else_val


def is_number(val: Any) -> bool:
    """Check if value is a number."""
    return isinstance(val, (int, float))


def is_string(val: Any) -> bool:
    """Check if value is a string."""
    return isinstance(val, str)


def is_boolean(val: Any) -> bool:
    """Check if value is a boolean."""
    return isinstance(val, bool)


def is_nil(val: Any) -> bool:
    """Check if value is None (nil)."""
    return val is None


def is_array(val: Any) -> bool:
    """Check if value is an array (list)."""
    return isinstance(val, list)


def is_dictionary(val: Any) -> bool:
    """Check if value is a dictionary."""
    return isinstance(val, dict)


def coalesce(a: Any, b: Any) -> Any:
    """Return first non-None value."""
    return a if a is not None else b


def default(val: Any, default_val: Any) -> Any:
    """Return value or default if None."""
    return val if val is not None else default_val


# Export all public functions
__all__ = [
    'get_nested',
    'is_truthy',
    # String functions
    'upcase',
    'downcase',
    'capitalize',
    'strip',
    'split',
    'join',
    'truncate',
    'replace',
    # Numeric functions
    'abs_num',
    'ceil_num',
    'floor_num',
    'round_num',
    'plus',
    'minus',
    'times',
    'divided_by',
    'modulo',
    # Collection functions
    'size',
    'first',
    'last',
    'contains',
    'sum_arr',
    'avg',
    'sort_arr',
    'keys',
    'values',
    'reverse',
    'at',
    # Logic functions
    'choose',
    'if_then_else',
    'is_number',
    'is_string',
    'is_boolean',
    'is_nil',
    'is_array',
    'is_dictionary',
    'coalesce',
    'default',
]
