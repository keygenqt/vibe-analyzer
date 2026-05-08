//! Search by code variables tool tests.

#[tokio::test]
async fn test_search_variables_all_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Найди переменные в проекте 'samples'").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("search_by_code_variables"))
    );
}

#[tokio::test]
async fn test_search_variables_constants_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Какие константы есть в 'samples'").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("search_by_code_variables"))
    );
}

#[tokio::test]
async fn test_search_variables_enums_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Покажи все enum в 'samples'").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("search_by_code_variables"))
    );
}

#[tokio::test]
async fn test_search_variables_all_en() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Find variables in 'samples'").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("search_by_code_variables"))
    );
}

#[tokio::test]
async fn test_search_variables_max_value_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine
        .test("Дай список файлов где есть константа 'MAX_VALUE'")
        .await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("search_by_code_variables"))
    );
}
