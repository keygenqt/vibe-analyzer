//! Get file content tool tests.

#[tokio::test]
async fn test_get_file_content_main_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Покажи содержимое файла 'src/main.rs'").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("get_file_content"))
    );
}

#[tokio::test]
async fn test_get_file_content_lib_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Дай полный код файла 'src/lib.rs'").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("get_file_content"))
    );
}

#[tokio::test]
async fn test_get_file_content_main_en() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Show file content of 'main.py'").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("get_file_content"))
    );
}

#[tokio::test]
async fn test_get_file_content_models_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Выведи файл 'models.py'").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("get_file_content"))
    );
}

#[tokio::test]
async fn test_get_file_content_utils_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Открой файл 'utils.py'").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("get_file_content"))
    );
}
