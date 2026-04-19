"""
Data models for the application
"""

from dataclasses import dataclass
from typing import Optional


@dataclass
class User:
    """User representation"""
    name: str
    age: int
    email: Optional[str] = None

    def name(self) -> str:
        """Returns user name"""
        return self.name

    def age(self) -> int:
        """Returns user age"""
        return self.age

    def is_adult(self) -> bool:
        """Checks if user is adult (18+)"""
        return self.age >= 18


class Calculator:
    """Calculator with memory"""

    def __init__(self):
        self._value = 0

    def add(self, x: int) -> None:
        """Adds number to current value"""
        self._value += x

    def multiply(self, x: int) -> None:
        """Multiplies current value by number"""
        self._value *= x

    def value(self) -> int:
        """Returns current value"""
        return self._value

    def reset(self) -> None:
        """Resets calculator to zero"""
        self._value = 0
