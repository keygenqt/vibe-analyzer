"""
Utility functions for math operations
"""


def add(a: int, b: int) -> int:
    """Returns sum of two numbers"""
    return a + b


def multiply(a: int, b: int) -> int:
    """Returns product of two numbers"""
    return a * b


def divide(a: int, b: int) -> float:
    """
    Divides two numbers safely

    Args:
        a: dividend
        b: divisor

    Returns:
        division result

    Raises:
        ValueError: if divisor is zero
    """
    if b == 0:
        raise ValueError("Division by zero")
    return a / b
