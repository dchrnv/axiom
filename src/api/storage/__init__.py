
# Axiom - Высокопроизводительная система пространственных вычислений на основе токенов.
# Copyright (C) 2024-2025 Chernov Denys

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.

# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.

"""
Storage Abstraction Layer

Provides implementations for token, grid, and CDNA storage.
Supports multiple backends: in-memory (current) and runtime (future).
"""

import sys
from pathlib import Path

# Add src/core to path for Token import
src_path = Path(__file__).parent.parent.parent
sys.path.insert(0, str(src_path))
# ruff: noqa: E402

from core.token.token_v2 import Token
