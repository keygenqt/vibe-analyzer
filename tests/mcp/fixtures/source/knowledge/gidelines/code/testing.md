# Правила тестирования

## Общие принципы

- Тестируй поведение, а не реализацию
- Один тест — одна проверка
- Название теста должно описывать сценарий

## Структура теста

Следуй паттерну AAA:

1. **Arrange** — подготовка данных
2. **Act** — выполнение действия
3. **Assert** — проверка результата

## Что тестировать

- ✅ Публичный API
- ✅ Граничные условия
- ✅ Обработку ошибок
- ❌ Приватные методы (тестируются через публичные)
- ❌ Стандартную библиотеку

## Покрытие кода

- Минимальное покрытие — 80%
- Критические модули — 100%
- Не гонись за цифрами, тестируй важное

## Виды тестов

1. **Модульные** — отдельные функции и методы
2. **Интеграционные** — взаимодействие модулей
3. **E2E** — полные пользовательские сценарии

## Пример хорошего теста (Rust)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_positive_numbers_works() {
        // Arrange
        let a = 2;
        let b = 3;
        let expected = 5;

        // Act
        let result = add(a, b);

        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    fn divide_by_zero_returns_error() {
        // Arrange
        let dividend = 10;
        let divisor = 0;

        // Act
        let result = divide(dividend, divisor);

        // Assert
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Division by zero");
    }
}
```
