//! Search by code functions tool tests.

#[tokio::test]
async fn test_search_functions_add_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Найди функции add в проекте 'samples'").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("search_by_code_functions"))
    );
}

#[tokio::test]
async fn test_search_functions_methods_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Какие методы есть в 'samples'").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("search_by_code_functions"))
    );
}

#[tokio::test]
async fn test_search_functions_main_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Покажи все main функции в 'samples'").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("search_by_code_functions"))
    );
}

#[tokio::test]
async fn test_search_functions_calculate_en() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Find calculate functions in 'samples'").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("search_by_code_functions"))
    );
}

#[tokio::test]
async fn test_search_functions_multiply_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine
        .test("Дай список файлов где есть функция multiply")
        .await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("search_by_code_functions"))
    );
}
