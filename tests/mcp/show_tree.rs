//! Show tree tool tests.

#[tokio::test]
async fn test_show_tree_samples_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Покажи дерево проекта 'samples'").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("show_tree"))
    );
}

#[tokio::test]
async fn test_show_tree_knowledge_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Выведи файлы и папки в 'knowledge'").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("show_tree"))
    );
}

#[tokio::test]
async fn test_show_tree_samples_en() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Show tree of 'samples'").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("show_tree"))
    );
}

#[tokio::test]
async fn test_show_tree_knowledge_en() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Show folders of 'knowledge'").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("show_tree"))
    );
}

#[tokio::test]
async fn test_show_tree_samples_ru2() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Дай дерево проекта 'samples'").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("show_tree"))
    );
}
