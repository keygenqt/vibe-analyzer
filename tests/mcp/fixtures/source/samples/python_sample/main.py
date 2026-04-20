#!/usr/bin/env python3
"""
Main entry point for the sample application
"""

from utils import add, multiply
from models import User


def main():
    """Main function"""
    result = add(5, 10)
    print(f"5 + 10 = {result}")

    product = multiply(4, 7)
    print(f"4 * 7 = {product}")

    user = User(name="Alice", age=30)
    print(f"User: {user.name()}, age: {user.age()}")


if __name__ == "__main__":
    main()
