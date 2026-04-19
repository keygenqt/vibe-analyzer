# Правила написания кода на Python

## Стиль кода

- Следуй PEP 8
- 4 пробела для отступов
- Максимальная длина строки 88 символов (форматтер Black)

## Именование

- `snake_case` для функций и переменных
- `PascalCase` для классов
- `UPPER_CASE` для констант

## Типизация

- Всегда указывай аннотации типов для сигнатур функций
- Используй `Optional[T]` вместо `T | None`
- Для сложных типов используй `TypeAlias`

## Обработка ошибок

- Используй конкретные типы исключений
- Избегай голого `except:`
- Применяй контекстные менеджеры (`with`) для ресурсов

## Документирование

- Docstring в тройных кавычках для всех публичных функций
- Описывай параметры и возвращаемое значение
- Указывай возможные исключения

## Асинхронный код

- Используй `async`/`await` для IO операций
- Не смешивай синхронный и асинхронный код без необходимости
- Применяй `asyncio.gather` для параллельных запросов

## Пример правильного кода

```python
from typing import Optional, List
from dataclasses import dataclass

@dataclass
class User:
    """User representation in the system"""
    name: str
    age: int
    email: Optional[str] = None

def calculate_average(numbers: List[float]) -> float:
    """
    Calculates the arithmetic mean of a list of numbers

    Args:
        numbers: list of numbers

    Returns:
        average value

    Raises:
        ValueError: if the list is empty
    """
    if not numbers:
        raise ValueError("Number list cannot be empty")
    return sum(numbers) / len(numbers)


async def fetch_user_data(user_id: int) -> Optional[User]:
    """
    Fetches user data from the database

    Args:
        user_id: unique user identifier

    Returns:
        User object if found, None otherwise
    """
    # Simulate async database query
    await asyncio.sleep(0.1)
    if user_id > 0:
        return User(name=f"User_{user_id}", age=25)
    return None
```
