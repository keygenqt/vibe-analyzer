//! Search documentation tool tests.

#[tokio::test]
async fn test_search_docs_zizikosh_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Кто такой 'Зизикош'?").await;
    // Assert
    assert!(result.success);
    assert!(result.models.iter().all(|m| {
        let tool = m.tool.as_deref();
        tool == Some("search_documentation") || tool == Some("search_knowledge")
    }));
}

#[tokio::test]
async fn test_search_docs_agyr_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Найди информацию о профессии 'Агыра'").await;
    // Assert
    assert!(result.success);
    assert!(result.models.iter().all(|m| {
        let tool = m.tool.as_deref();
        tool == Some("search_documentation") || tool == Some("search_knowledge")
    }));
}

#[tokio::test]
async fn test_search_docs_kukyrbur_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Расскажи про способности 'Кукырбура'").await;
    // Assert
    assert!(result.success);
    assert!(result.models.iter().all(|m| {
        let tool = m.tool.as_deref();
        tool == Some("search_documentation") || tool == Some("search_knowledge")
    }));
}

#[tokio::test]
async fn test_search_docs_chukoruk_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("О чём пишет 'Чукорук'?").await;
    // Assert
    assert!(result.success);
    assert!(result.models.iter().all(|m| {
        let tool = m.tool.as_deref();
        tool == Some("search_documentation") || tool == Some("search_knowledge")
    }));
}

#[tokio::test]
async fn test_search_docs_bakobur_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Какая мечта у 'Бакобура'?").await;
    // Assert
    assert!(result.success);
    assert!(result.models.iter().all(|m| {
        let tool = m.tool.as_deref();
        tool == Some("search_documentation") || tool == Some("search_knowledge")
    }));
}

#[tokio::test]
async fn test_search_docs_git_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Найди документацию по 'Git' воркфлоу").await;
    // Assert
    assert!(result.success);
    assert!(result.models.iter().all(|m| {
        let tool = m.tool.as_deref();
        tool == Some("search_documentation") || tool == Some("search_knowledge")
    }));
}

#[tokio::test]
async fn test_search_docs_python_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine
        .test("Найди правила написания кода на 'Python'")
        .await;
    // Assert
    assert!(result.success);
    assert!(result.models.iter().all(|m| {
        let tool = m.tool.as_deref();
        tool == Some("search_documentation") || tool == Some("search_knowledge")
    }));
}

#[tokio::test]
async fn test_search_docs_review_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Найди чеклист 'код-ревью'").await;
    // Assert
    assert!(result.success);
    assert!(result.models.iter().all(|m| {
        let tool = m.tool.as_deref();
        tool == Some("search_documentation") || tool == Some("search_knowledge")
    }));
}

#[tokio::test]
async fn test_search_docs_rust_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine
        .test("Найди руководство по тестированию на 'Rust'")
        .await;
    // Assert
    assert!(result.success);
    assert!(result.models.iter().all(|m| {
        let tool = m.tool.as_deref();
        tool == Some("search_documentation") || tool == Some("search_knowledge")
    }));
}

#[tokio::test]
async fn test_search_docs_release_ru() {
    // Arrange
    let engine = vibe_tests::engine().await;
    // Act
    let result = engine.test("Покажи процесс 'релиза'").await;
    // Assert
    assert!(result.success);
    assert!(result.models.iter().all(|m| {
        let tool = m.tool.as_deref();
        tool == Some("search_documentation") || tool == Some("search_knowledge")
    }));
}
