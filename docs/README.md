# Axiom Documentation

Sphinx-based documentation for Axiom.

## Building Locally

1. Install dependencies:
   ```bash
   pip install sphinx sphinx-rtd-theme sphinx-autodoc-typehints myst-parser
   ```

2. Build HTML documentation:
   ```bash
   cd docs
   make html
   ```

3. Open in browser:
   ```bash
   open build/html/index.html
   ```

## Structure

- `source/` - Documentation source files (reStructuredText and Markdown)
  - `api/` - API reference pages (auto-generated from docstrings)
  - `conf.py` - Sphinx configuration
  - `index.rst` - Main documentation page
  - Other guides: quickstart, architecture, configuration

- `build/` - Generated HTML output (not committed to git)

## GitHub Pages

Documentation is automatically built and deployed to GitHub Pages on every push to `main` that affects:
- `docs/**`
- `src/api/**`

Workflow: `.github/workflows/docs.yml`

## Writing Documentation

### reStructuredText (.rst)

Main format for structured documentation:

```rst
Section Title
=============

Subsection
----------

- List item
- Another item

.. code-block:: python

   def example():
       return "Hello"
```

### Markdown (.md)

Supported via MyST parser for simple pages.

### Autodoc

API reference is auto-generated from Python docstrings using Google style:

```python
def my_function(arg1: str, arg2: int) -> bool:
    """Short description.

    Longer description if needed.

    Args:
        arg1: Description of arg1
        arg2: Description of arg2

    Returns:
        Description of return value

    Example:
        >>> my_function("test", 42)
        True
    """
    return True
```

## Tips

- Use `make clean` before `make html` for a fresh build
- Check for warnings in the build output
- Test links with `make linkcheck`
- View all make targets with `make help`
