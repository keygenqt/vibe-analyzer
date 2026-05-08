//! Show projects tool tests.

#[tokio::test]
async fn test_show_projects_list_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Какие проекты есть в базе").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("show_projects"))
    );
}

#[tokio::test]
async fn test_show_projects_all_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Покажи все доступные проекты").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("show_projects"))
    );
}

#[tokio::test]
async fn test_show_projects_what_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Что у нас есть").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("show_projects"))
    );
}

#[tokio::test]
async fn test_show_projects_list_en() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("List available projects").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("show_projects"))
    );
}

#[tokio::test]
async fn test_show_projects_what_en() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("What projects are available").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("show_projects"))
    );
}
