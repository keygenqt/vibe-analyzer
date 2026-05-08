//! Search by code classes tool tests.

#[tokio::test]
async fn test_search_classes_user_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Найди класс User в проекте 'samples'").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("search_by_code_classes"))
    );
}

#[tokio::test]
async fn test_search_classes_all_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Какие классы есть в 'samples'").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("search_by_code_classes"))
    );
}

#[tokio::test]
async fn test_search_classes_struct_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Покажи все struct в 'samples'").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("search_by_code_classes"))
    );
}

#[tokio::test]
async fn test_search_classes_all_en() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Find classes in 'samples'").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("search_by_code_classes"))
    );
}

#[tokio::test]
async fn test_search_classes_calculator_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine
        .test("Дай список файлов где есть struct Calculator")
        .await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("search_by_code_classes"))
    );
}
