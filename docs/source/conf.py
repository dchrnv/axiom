# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

import os
import sys

# Add the project root to the path so autodoc can find modules
sys.path.insert(0, os.path.abspath('../../src'))

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = 'NeuroGraph'
copyright = '2024-2026, Chernov Denys'
author = 'Chernov Denys'

version = '0.65.0'
release = '0.65.0'

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

extensions = [
    'sphinx.ext.autodoc',           # Auto-generate docs from docstrings
    'sphinx.ext.viewcode',          # Add links to source code
    'sphinx.ext.napoleon',          # Support Google/NumPy docstring styles
    'sphinx_autodoc_typehints',     # Type hints in documentation
    'myst_parser',                  # Markdown support
    'sphinx.ext.intersphinx',       # Link to other project docs
    'sphinx.ext.todo',              # Todo notes
    'sphinx.ext.coverage',          # Documentation coverage
]

# Napoleon settings for Google-style docstrings
napoleon_google_docstring = True
napoleon_numpy_docstring = False
napoleon_include_init_with_doc = True
napoleon_include_private_with_doc = False
napoleon_include_special_with_doc = True
napoleon_use_admonition_for_examples = True
napoleon_use_admonition_for_notes = True
napoleon_use_admonition_for_references = False
napoleon_use_ivar = False
napoleon_use_param = True
napoleon_use_rtype = True
napoleon_preprocess_types = False
napoleon_type_aliases = None
napoleon_attr_annotations = True

# Autodoc settings
autodoc_default_options = {
    'members': True,
    'member-order': 'bysource',
    'special-members': '__init__',
    'undoc-members': True,
    'exclude-members': '__weakref__'
}
autodoc_typehints = 'description'
autodoc_typehints_description_target = 'documented'

# MyST Parser settings (for Markdown)
myst_enable_extensions = [
    "deflist",
    "tasklist",
    "colon_fence",
]

# Intersphinx mapping (link to other docs)
intersphinx_mapping = {
    'python': ('https://docs.python.org/3', None),
    'fastapi': ('https://fastapi.tiangolo.com', None),
}

# Template and source settings
templates_path = ['_templates']
exclude_patterns = []
source_suffix = {
    '.rst': 'restructuredtext',
    '.md': 'markdown',
}

# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = 'sphinx_rtd_theme'
html_static_path = ['_static']

# Theme options
html_theme_options = {
    'logo_only': False,
    'display_version': True,
    'prev_next_buttons_location': 'bottom',
    'style_external_links': False,
    'style_nav_header_background': '#2980B9',
    # Toc options
    'collapse_navigation': False,
    'sticky_navigation': True,
    'navigation_depth': 4,
    'includehidden': True,
    'titles_only': False
}

# Project metadata
html_title = f"{project} {version}"
html_short_title = project
html_show_sourcelink = True
html_show_sphinx = True
html_show_copyright = True

# Additional HTML context
html_context = {
    "display_github": True,
    "github_user": "chrnv",
    "github_repo": "neurograph-os-mvp",
    "github_version": "main",
    "conf_py_path": "/docs/source/",
}
