"""
Axiom Jupyter Integration

IPython extension for interactive Axiom operations in Jupyter notebooks.

Usage:
    %load_ext axiom_jupyter
    %axiom init --path ./my_graph.db
    %axiom status
    %axiom query "find all nodes"

Cell magic for signal definitions:
    %%signal process_data
    def handler(data):
        return data * 2
"""

from .magic import AxiomMagics
from .display import install_display_formatters
from .helpers import install_result_extensions


def load_ipython_extension(ipython):
    """
    Load the IPython extension.

    This is called when `%load_ext axiom_jupyter` is executed.
    """
    # Register magic commands
    magics = AxiomMagics(ipython)
    ipython.register_magics(magics)

    # Register auto-completion
    try:
        ipython.set_hook('complete_command', magics._axiom_completions, str_key='%axiom')
    except Exception:  # noqa: S110
        pass  # Graceful degradation if completion not supported

    # Install rich display formatters
    install_display_formatters(ipython)

    # Install QueryResult extension methods
    if install_result_extensions():
        print("✅ Axiom Jupyter extension loaded")
        print("💡 Auto-completion enabled (press TAB after %axiom)")
        print("💡 DataFrame helpers: result.to_dataframe(), result.export_csv(), result.plot_distribution()")
    else:
        print("✅ Axiom Jupyter extension loaded")
        print("💡 Auto-completion enabled (press TAB after %axiom)")

    print("Try: %axiom init --path ./my_graph.db")


def unload_ipython_extension(ipython):
    """
    Unload the IPython extension.

    This is called when `%unload_ext axiom_jupyter` is executed.
    """
    print("Axiom Jupyter extension unloaded")


__version__ = "0.61.1"
__all__ = ["AxiomMagics", "load_ipython_extension", "unload_ipython_extension"]
