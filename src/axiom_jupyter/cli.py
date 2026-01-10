"""
Command-line interface for Axiom Jupyter integration.
"""

import sys
import argparse


def main():
    """Main CLI entry point."""
    parser = argparse.ArgumentParser(
        description="Axiom Jupyter Integration CLI",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Load the extension in Jupyter
  %load_ext axiom_jupyter

  # Initialize Axiom
  %axiom init --path ./my_graph.db

  # Query the graph
  %axiom query "find all nodes"

  # Check status
  %axiom status

For more information, visit:
  https://github.com/chrnv/axiom-os-mvp
        """
    )

    parser.add_argument(
        '--version',
        action='version',
        version='axiom-jupyter 0.63.1'
    )

    parser.add_argument(
        'command',
        nargs='?',
        choices=['info', 'help'],
        default='help',
        help='Command to execute'
    )

    args = parser.parse_args()

    if args.command == 'info' or args.command == 'help':
        print("Axiom Jupyter Integration v0.63.1")
        print()
        print("To use in Jupyter notebooks:")
        print("  %load_ext axiom_jupyter")
        print()
        print("Available magic commands:")
        print("  %axiom init --path <db_path>  - Initialize connection")
        print("  %axiom status                 - Show system status")
        print("  %axiom query <query>          - Execute query")
        print("  %axiom subscribe <channel>    - Subscribe to channel")
        print("  %axiom emit <channel> <data>  - Emit event")
        print()
        print("For full documentation, see:")
        print("  https://github.com/chrnv/axiom-os-mvp/blob/main/README.md")
        return 0

    return 0


if __name__ == '__main__':
    sys.exit(main())
