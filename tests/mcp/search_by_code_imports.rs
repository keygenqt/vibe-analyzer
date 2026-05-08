//! Search by code imports tool tests.

#[tokio::test]
async fn test_search_imports_thiserror_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine
        .test("Найди импорты thiserror в проекте 'samples'")
        .await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("search_by_code_imports"))
    );
}

#[tokio::test]
async fn test_search_imports_libraries_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine
        .test("Какие библиотеки используются в 'samples'")
        .await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("search_by_code_imports"))
    );
}

#[tokio::test]
async fn test_search_imports_typing_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Покажи все импорты typing в 'samples'").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("search_by_code_imports"))
    );
}

#[tokio::test]
async fn test_search_imports_dataclasses_en() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine
        .test("Find imports of dataclasses in 'samples'")
        .await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("search_by_code_imports"))
    );
}

#[tokio::test]
async fn test_search_imports_utils_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine
        .test("Дай список файлов где есть импорты utils")
        .await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("search_by_code_imports"))
    );
}
