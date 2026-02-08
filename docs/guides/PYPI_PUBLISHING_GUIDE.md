# PyPI Publishing Guide

Пошаговое руководство по публикации NeuroGraph на PyPI.

## Предварительные требования

```bash
# Установить build tools
pip install --upgrade pip
pip install maturin build twine

# Установить Rust (если еще не установлен)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Шаг 1: Проверка версии

Убедитесь что версия в `pyproject.toml` соответствует текущему релизу:

```toml
[project]
version = "0.63.1"  # <- Проверьте эту строку
```

## Шаг 2: Очистка предыдущих сборок

```bash
# Удалить старые артефакты
rm -rf dist/ build/ target/release/
rm -rf src/core_rust/target/release/

# Очистить Python кэш
find . -type d -name "__pycache__" -exec rm -rf {} + 2>/dev/null || true
find . -type f -name "*.pyc" -delete
find . -type f -name "*.pyo" -delete
```

## Шаг 3: Сборка пакета

### Вариант A: Только source distribution (быстро, для тестирования)

```bash
# Собрать sdist без компиляции Rust
maturin build --sdist
```

### Вариант B: Полная сборка с Rust (долго, ~5-10 минут)

```bash
# Собрать wheel с скомпилированным Rust кодом
maturin build --release --strip

# Также создать source distribution
maturin build --sdist
```

### Вариант C: Manylinux wheels для максимальной совместимости

```bash
# Требует Docker
docker pull quay.io/pypa/manylinux_2_28_x86_64
maturin build --release --manylinux 2_28
```

## Шаг 4: Проверка собранного пакета

```bash
# Проверить что файлы созданы
ls -lh dist/

# Проверить содержимое с помощью twine
twine check dist/*

# Проверить метаданные
tar -tzf dist/neurograph-0.63.1.tar.gz | head -20
```

## Шаг 5: Тестирование локальной установки

```bash
# Создать виртуальное окружение для тестирования
python -m venv test-venv
source test-venv/bin/activate

# Установить из локального файла
pip install dist/neurograph-0.63.1*.whl

# Протестировать импорт
python -c "import neurograph; print(neurograph.__version__)"

# Тестировать Jupyter integration
python -c "from neurograph_jupyter import NeuroGraphMagics; print('OK')"

# Деактивировать и удалить тест-окружение
deactivate
rm -rf test-venv
```

## Шаг 6: Публикация на TestPyPI (рекомендуется)

```bash
# Загрузить на TestPyPI для тестирования
twine upload --repository testpypi dist/*

# Для этого нужен аккаунт на https://test.pypi.org/
# API token можно создать в настройках аккаунта
```

Тестирование установки с TestPyPI:

```bash
# Создать новое окружение
python -m venv test-testpypi
source test-testpypi/bin/activate

# Установить из TestPyPI
pip install --index-url https://test.pypi.org/simple/ --extra-index-url https://pypi.org/simple/ neurograph

# Протестировать
python -c "import neurograph; print('Success!')"

deactivate
rm -rf test-testpypi
```

## Шаг 7: Публикация на Production PyPI

⚠️ **ВНИМАНИЕ:** Версия, загруженная на PyPI, не может быть удалена! Убедитесь что всё работает в TestPyPI.

```bash
# Загрузить на production PyPI
twine upload dist/*

# Для этого нужен аккаунт на https://pypi.org/
# Настоятельно рекомендуется использовать API token вместо пароля
```

## Шаг 8: Проверка на Production PyPI

```bash
# Подождать 1-2 минуты для индексации

# Создать новое окружение
python -m venv test-prod
source test-prod/bin/activate

# Установить из PyPI
pip install neurograph

# Полная установка со всеми фичами
pip install neurograph[all]

# Только Jupyter
pip install neurograph[jupyter]

# Только API server
pip install neurograph[api]

# Тестирование
python -c "import neurograph; print(neurograph.__version__)"

deactivate
rm -rf test-prod
```

## Шаг 9: Создание GitHub Release

После успешной публикации на PyPI:

```bash
# Создать git tag
git tag -a v0.63.1 -m "Release v0.63.1 - PyPI Publication"
git push origin v0.63.1

# Создать GitHub Release через веб-интерфейс
# https://github.com/chrnv/neurograph-os-mvp/releases/new
#
# - Tag: v0.63.1
# - Title: "NeuroGraph OS v0.63.1"
# - Description: Copy from CHANGELOG.md
# - Attach: dist/* files
```

## Автоматизация с GitHub Actions (будущее)

Создать `.github/workflows/publish-pypi.yml`:

```yaml
name: Publish to PyPI

on:
  release:
    types: [published]

jobs:
  build-and-publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
      - uses: dtolnay/rust-toolchain@stable

      - name: Build package
        run: |
          pip install maturin
          maturin build --release --sdist

      - name: Publish to PyPI
        env:
          TWINE_USERNAME: __token__
          TWINE_PASSWORD: ${{ secrets.PYPI_API_TOKEN }}
        run: |
          pip install twine
          twine upload dist/*
```

## Troubleshooting

### Ошибка: "Rust compiler not found"

```bash
# Установить Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Ошибка: "Version X already exists on PyPI"

- Нельзя перезаписать существующую версию на PyPI
- Нужно увеличить версию в `pyproject.toml`
- Например: 0.63.1 → 0.63.2

### Ошибка при сборке: "No such file or directory: python/neurograph"

- Убедитесь что `python-source = "python"` в `[tool.maturin]`
- Проверьте что директория `python/neurograph/` существует

### Ошибка: "Invalid classifier"

- Проверьте список допустимых classifiers: https://pypi.org/classifiers/
- Убедитесь что нет опечаток в `classifiers` секции

## Полезные ссылки

- **PyPI:** https://pypi.org/
- **TestPyPI:** https://test.pypi.org/
- **Maturin Docs:** https://www.maturin.rs/
- **Python Packaging Guide:** https://packaging.python.org/
- **Twine Docs:** https://twine.readthedocs.io/

## Контрольный список перед публикацией

- [ ] Версия обновлена в `pyproject.toml`
- [ ] `CHANGELOG.md` содержит описание изменений
- [ ] Все тесты проходят (`pytest tests/`)
- [ ] Rust tests проходят (`cargo test --manifest-path src/core_rust/Cargo.toml`)
- [ ] Пакет собирается без ошибок (`maturin build --release`)
- [ ] `twine check dist/*` проходит без warnings
- [ ] Локальная установка работает
- [ ] Тестирование на TestPyPI успешно
- [ ] README.md актуален
- [ ] Git tag создан и запушен
