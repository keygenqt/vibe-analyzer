# Процесс релиза

## Версионирование

Следуем Semantic Versioning: `MAJOR.MINOR.PATCH`

- **MAJOR** — обратно несовместимые изменения
- **MINOR** — новая функциональность, обратно совместимо
- **PATCH** — исправления багов, обратно совместимо

## Этапы релиза

### 1. Подготовка

- [ ] Все задачи в милестоне закрыты
- [ ] Все тесты проходят на всех платформах
- [ ] Документация обновлена
- [ ] CHANGELOG заполнен

### 2. Релизная ветка

```bash
git checkout develop
git checkout -b release/v1.2.0
```

### 3. Финальные правки

- Обновить версию в Cargo.toml / package.json
- Обновить CHANGELOG.md
- Исправить последние баги если есть

### 4. Слияние

```bash
git checkout main
git merge --no-ff release/v1.2.0
git tag -a v1.2.0 -m "Release v1.2.0"
git push --tags
git checkout develop
git merge --no-ff release/v1.2.0
git branch -d release/v1.2.0
```

### 5. После релиза

- Уведомить команду в Slack
- Проверить деплой в production
- Создать milestone для следующей версии

## Откат релиза

Если обнаружена критическая проблема:

1. Создать `hotfix` ветку от `main`
2. Исправить проблему
3. Слить в `main` и `develop`
4. Выпустить `PATCH` версию
