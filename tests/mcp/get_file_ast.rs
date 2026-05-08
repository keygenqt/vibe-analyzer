//! Get file AST tool tests.

#[tokio::test]
async fn test_get_file_ast_lib_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Покажи AST файла 'src/lib.rs'").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("get_file_ast"))
    );
}

#[tokio::test]
async fn test_get_file_ast_structure_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Дай структуру файла 'main.py'").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("get_file_ast"))
    );
}

#[tokio::test]
async fn test_get_file_ast_functions_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine
        .test("Покажи структуру и функции в файле 'src/main.rs'")
        .await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("get_file_ast"))
    );
}

#[tokio::test]
async fn test_get_file_ast_lib_en() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Show AST of file 'lib.rs'").await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("get_file_ast"))
    );
}

#[tokio::test]
async fn test_get_file_ast_models_en() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine
        .test("Find abstract syntax tree of file 'models.py'")
        .await;
    // Assert
    assert!(result.success);
    assert!(
        result
            .models
            .iter()
            .all(|m| m.tool.as_deref() == Some("get_file_ast"))
    );
}
