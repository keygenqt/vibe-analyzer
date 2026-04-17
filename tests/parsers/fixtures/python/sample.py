"""
Module documentation for Python testing
"""

import os
import sys
from datetime import datetime
from typing import List, Optional

# Regular comment - ignored


def add(a: int, b: int) -> int:
    """Adds two numbers"""
    return a + b


def multiply(a: int, b: int) -> int:
    """Multiplies two numbers"""
    return a * b


async def fetch_data(url: str) -> str:
    """Async function example"""
    return "data"


class User:
    """User class"""

    def __init__(self, name: str, age: int):
        """Constructor"""
        self.name = name
        self.age = age

    def get_name(self) -> str:
        """Get user name"""
        return self.name


class Config:
    """Config class"""
    debug: bool = False
    max_size: int = 1024


class Color:
    """Color enum (using class constants)"""
    RED = 1
    GREEN = 2
    BLUE = 3


MAX_SIZE: int = 1024
DEFAULT_TIMEOUT: int = 30

APP_NAME: str = "vibe-analyzer"

# Regular comment at the end - ignored
