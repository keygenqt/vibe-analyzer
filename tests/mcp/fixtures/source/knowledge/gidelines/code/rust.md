# Правила написания кода на Rust

## Соглашения об именовании

- `snake_case` для функций, переменных, модулей
- `CamelCase` для структур, перечислений, типажей
- `SCREAMING_SNAKE_CASE` для констант

## Обработка ошибок

- Используй `Result<T, E>` для восстанавливаемых ошибок
- Избегай `unwrap()` в production коде
- Предпочитай `anyhow::Result` для прикладного кода
- Используй `thiserror` для библиотечных типов ошибок

## Документирование

- Каждая публичная функция должна иметь `///` комментарий
- Добавляй примеры в doc-тесты
- Документируй условия паники

## Тестирование

- Модульные тесты в том же файле с `#[cfg(test)]`
- Интеграционные тесты в директории `tests/`
- Используй `assert_eq!` для сравнений

## Время жизни и заимствование

- Избегай излишнего клонирования
- Используй ссылки где возможно
- Применяй `Cow` для данных которые могут быть заимствованными или владеющими

## Пример правильного кода

````rust
/// Calculates sum of two numbers
///
/// # Examples
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// User structure with validation
pub struct User {
    name: String,
    age: u8,
}

impl User {
    /// Creates a new user
    ///
    /// # Panics
    /// Panics if age is greater than 150
    pub fn new(name: String, age: u8) -> Self {
        assert!(age <= 150, "Unrealistic age");
        Self { name, age }
    }
}
````
