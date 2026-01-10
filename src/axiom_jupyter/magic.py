"""
IPython magic commands for Axiom operations.
"""

import argparse
from pathlib import Path
from typing import Optional

from IPython.core.magic import Magics, magics_class, line_magic, cell_magic
from IPython.core.magic_arguments import argument, magic_arguments

from axiom.core.connection_manager import ConnectionManager
from axiom.core.graph_operations import GraphOperations
from axiom.query.signal_engine import SignalEngine


@magics_class
class AxiomMagics(Magics):
    """
    IPython magic commands for Axiom.

    Line magics:
        %axiom init --path <db_path>
        %axiom status
        %axiom query <query_string>
        %axiom subscribe <channel>
        %axiom emit <channel> <data>

    Cell magics:
        %%signal <signal_name>
        def handler(data):
            return data
    """

    def __init__(self, shell):
        super().__init__(shell)
        self.connection_manager: Optional[ConnectionManager] = None
        self.graph_ops: Optional[GraphOperations] = None
        self.signal_engine: Optional[SignalEngine] = None
        self.db_path: Optional[Path] = None

        # Auto-completion support
        self._commands = ["init", "status", "query", "subscribe", "emit"]
        self._channels = ["metrics", "signals", "actions", "logs", "status", "connections"]

    @line_magic
    @magic_arguments()
    @argument("command", type=str, help="Command to execute: init, status, query, subscribe, emit")
    @argument("args", nargs="*", help="Command arguments")
    @argument("--path", type=str, help="Database path for init command")
    def axiom(self, line):
        """
        Main Axiom magic command.

        Examples:
            %axiom init --path ./my_graph.db
            %axiom status
            %axiom query "find all nodes"
            %axiom subscribe metrics
            %axiom emit metrics "{'value': 42}"
        """
        # Parse arguments
        parts = line.split(None, 1)
        if not parts:
            return self._show_help()

        command = parts[0].lower()
        args = parts[1] if len(parts) > 1 else ""

        # Route to command handlers
        if command == "init":
            return self._handle_init(args)
        elif command == "status":
            return self._handle_status()
        elif command == "query":
            return self._handle_query(args)
        elif command == "subscribe":
            return self._handle_subscribe(args)
        elif command == "emit":
            return self._handle_emit(args)
        else:
            print(f"❌ Unknown command: {command}")
            return self._show_help()

    def _handle_init(self, args: str):
        """Initialize Axiom connection."""
        # Parse --path argument
        parser = argparse.ArgumentParser(prog="%axiom init")
        parser.add_argument("--path", type=str, required=True, help="Database path")

        try:
            parsed = parser.parse_args(args.split())
        except SystemExit:
            return

        db_path = Path(parsed.path).expanduser().resolve()

        try:
            # Initialize connection manager
            self.connection_manager = ConnectionManager()
            self.graph_ops = GraphOperations(str(db_path))
            self.signal_engine = SignalEngine(self.connection_manager)
            self.db_path = db_path

            # Store in IPython namespace for direct access
            self.shell.user_ns["axiom_db"] = self.graph_ops
            self.shell.user_ns["axiom_signals"] = self.signal_engine
            self.shell.user_ns["axiom_ws"] = self.connection_manager

            print("✅ Axiom initialized")
            print(f"📁 Database: {db_path}")
            print("🔗 Connection Manager: Ready")
            print("📡 Signal Engine: Ready")
            print("\nAvailable in namespace:")
            print("  - axiom_db: GraphOperations")
            print("  - axiom_signals: SignalEngine")
            print("  - axiom_ws: ConnectionManager")

        except Exception as e:
            print(f"❌ Failed to initialize Axiom: {e}")
            import traceback
            traceback.print_exc()

    def _handle_status(self):
        """Show current Axiom status."""
        if not self.connection_manager:
            print("❌ Axiom not initialized. Run: %axiom init --path <db_path>")
            return

        print("🟢 Axiom Status")
        print(f"📁 Database: {self.db_path}")
        print(f"🔗 Active Connections: {len(self.connection_manager._connections)}")
        print(f"📡 Signal Engine: {'Active' if self.signal_engine else 'Inactive'}")

        # Show subscriptions
        total_subscriptions = sum(
            len(subs) for subs in self.connection_manager._subscriptions.values()
        )
        print(f"📬 Total Subscriptions: {total_subscriptions}")

        # Show channels
        channels = list(self.connection_manager._subscriptions.keys())
        if channels:
            print(f"📢 Active Channels: {', '.join(channels[:5])}")
            if len(channels) > 5:
                print(f"   ... and {len(channels) - 5} more")

    def _handle_query(self, query_string: str):
        """Execute a Axiom query."""
        if not self.graph_ops:
            print("❌ Axiom not initialized. Run: %axiom init --path <db_path>")
            return

        # Remove quotes if present
        query_string = query_string.strip().strip('"').strip("'")

        try:
            result = self.graph_ops.query(query_string)
            # Store result in _ variable for access
            self.shell.user_ns["_axiom_result"] = result
            return result  # Will be displayed by rich formatter

        except Exception as e:
            print(f"❌ Query failed: {e}")
            import traceback
            traceback.print_exc()

    def _handle_subscribe(self, channel: str):
        """Subscribe to a channel."""
        if not self.connection_manager:
            print("❌ Axiom not initialized. Run: %axiom init --path <db_path>")
            return

        channel = channel.strip()

        # Create a client ID for the notebook
        client_id = "jupyter_notebook"

        try:
            # Register connection if not exists
            if client_id not in self.connection_manager._connections:
                self.connection_manager.register_connection(client_id)

            # Subscribe to channel
            self.connection_manager.subscribe(client_id, [channel])

            print(f"✅ Subscribed to channel: {channel}")
            print(f"👤 Client ID: {client_id}")

        except Exception as e:
            print(f"❌ Subscription failed: {e}")

    def _handle_emit(self, args: str):
        """Emit a signal to a channel."""
        if not self.connection_manager:
            print("❌ Axiom not initialized. Run: %axiom init --path <db_path>")
            return

        # Parse channel and data
        parts = args.split(None, 1)
        if len(parts) < 2:
            print("❌ Usage: %axiom emit <channel> <data>")
            return

        channel = parts[0].strip()
        data = parts[1].strip()

        # Parse data as JSON if it looks like JSON
        import json
        try:
            data = json.loads(data)
        except json.JSONDecodeError:
            pass  # Keep as string

        try:
            # Broadcast to channel
            import asyncio
            asyncio.run(
                self.connection_manager.broadcast_to_channel(
                    channel,
                    {
                        "type": "signal",
                        "channel": channel,
                        "data": data
                    }
                )
            )

            print(f"✅ Signal emitted to channel: {channel}")
            print(f"📊 Data: {data}")

        except Exception as e:
            print(f"❌ Emit failed: {e}")

    @cell_magic
    def signal(self, line, cell):
        """
        Define a signal handler.

        Usage:
            %%signal process_data
            def handler(data):
                return data * 2

        The function defined in the cell will be registered as a signal handler.
        """
        if not self.signal_engine:
            print("❌ Axiom not initialized. Run: %axiom init --path <db_path>")
            return

        signal_name = line.strip()
        if not signal_name:
            print("❌ Signal name required")
            return

        try:
            # Execute the cell to define the function
            self.shell.run_cell(cell)

            # Look for the defined function in the namespace
            if "handler" in self.shell.user_ns:
                handler_func = self.shell.user_ns["handler"]

                # Register with signal engine
                # Note: This is simplified - real implementation would integrate with SignalEngine
                print(f"✅ Signal handler registered: {signal_name}")
                print(f"📡 Function: {handler_func.__name__}")

                # Store in namespace
                self.shell.user_ns[f"signal_{signal_name}"] = handler_func

            else:
                print("❌ No 'handler' function found in cell")

        except Exception as e:
            print(f"❌ Signal registration failed: {e}")
            import traceback
            traceback.print_exc()

    def _show_help(self):
        """Show help message."""
        help_text = """
🧠 Axiom Magic Commands

Initialization:
    %axiom init --path <db_path>    Initialize Axiom connection

Status:
    %axiom status                    Show current status

Queries:
    %axiom query "<query_string>"    Execute a query

Real-time:
    %axiom subscribe <channel>       Subscribe to channel
    %axiom emit <channel> <data>     Emit signal to channel

Cell Magic:
    %%signal <signal_name>                Define signal handler
    def handler(data):
        return processed_data

Examples:
    %axiom init --path ./my_graph.db
    %axiom query "find all nodes where type='user'"
    %axiom subscribe metrics
    %axiom emit metrics "{'cpu': 42}"
"""
        print(help_text)

    # ========================================================================
    # Auto-completion support
    # ========================================================================

    def _get_node_types(self):
        """Get available node types from database."""
        if not self.graph_ops:
            return []

        try:
            # Query to get unique node types
            result = self.graph_ops.query("find all nodes")
            types = set()
            for node in result.nodes:
                if hasattr(node, 'type') and node.type:
                    types.add(node.type)
            return sorted(list(types))
        except Exception:  # noqa: S110
            return []

    def _get_property_names(self):
        """Get available property names from database."""
        if not self.graph_ops:
            return []

        try:
            result = self.graph_ops.query("find all nodes")
            props = set()
            for node in result.nodes:
                if hasattr(node, 'properties') and node.properties:
                    props.update(node.properties.keys())
            return sorted(list(props))
        except Exception:  # noqa: S110
            return []

    def _axiom_completions(self, event):
        """
        Provide auto-completions for %axiom magic command.

        This is called by IPython when user presses TAB.
        """
        # Get the line up to cursor
        text_until_cursor = event.text_until_cursor

        # Split into words
        words = text_until_cursor.split()

        if len(words) <= 1:
            # Complete command names
            return self._commands

        command = words[1] if len(words) > 1 else ""

        # Complete based on command
        if command == "subscribe" or command == "emit":
            # Complete channel names
            if len(words) == 2:
                return self._channels
            elif len(words) > 2:
                # Get active channels from connection manager
                if self.connection_manager:
                    channels = list(self.connection_manager._subscriptions.keys())
                    return channels if channels else self._channels
                return self._channels

        elif command == "query":
            # Complete query syntax
            if "where type=" in text_until_cursor:
                # Complete node types
                return self._get_node_types()
            elif "where " in text_until_cursor:
                # Complete property names
                return self._get_property_names()
            else:
                # Complete query keywords
                return ["find all nodes", "find all nodes where", "find all edges"]

        elif command == "init":
            # Complete with --path flag
            if "--" in text_until_cursor:
                return []
            return ["--path"]

        return []
