"""
Unit tests for API Keys Storage.

Tests key generation, validation, expiration, and storage operations.
"""

import pytest
import tempfile
from pathlib import Path
from datetime import datetime, timedelta
import json

import sys
from pathlib import Path as PathLib
src_path = PathLib(__file__).parent.parent.parent.parent / "src"
sys.path.insert(0, str(src_path))

from api.storage.api_keys import APIKeyStorage
from api.models.auth import APIKey


class TestAPIKeyGeneration:
    """Test API key generation."""

    def test_generate_live_key(self):
        """Test generating live environment key."""
        with tempfile.TemporaryDirectory() as tmpdir:
            storage = APIKeyStorage(storage_path=f"{tmpdir}/keys.json")
            
            full_key, api_key = storage.generate_key(
                name="Test Live Key",
                scopes=["tokens:read"],
                environment="live"
            )

            assert full_key.startswith("ng_live_")
            assert len(full_key) > 20
            assert api_key.name == "Test Live Key"
            assert api_key.scopes == ["tokens:read"]
            assert api_key.key_prefix.startswith("ng_live_")
            assert api_key.disabled is False

    def test_generate_test_key(self):
        """Test generating test environment key."""
        with tempfile.TemporaryDirectory() as tmpdir:
            storage = APIKeyStorage(storage_path=f"{tmpdir}/keys.json")
            
            full_key, api_key = storage.generate_key(
                name="Test Key",
                scopes=["tokens:write"],
                environment="test"
            )

            assert full_key.startswith("ng_test_")
            assert api_key.key_prefix.startswith("ng_test_")

    def test_generate_key_with_expiration(self):
        """Test generating key with expiration."""
        with tempfile.TemporaryDirectory() as tmpdir:
            storage = APIKeyStorage(storage_path=f"{tmpdir}/keys.json")
            
            full_key, api_key = storage.generate_key(
                name="Expiring Key",
                scopes=["tokens:read"],
                expires_in_days=30
            )

            assert api_key.expires_at is not None
            expires_in = api_key.expires_at - datetime.utcnow()
            assert 29 <= expires_in.days <= 30

    def test_generate_key_without_expiration(self):
        """Test generating key without expiration."""
        with tempfile.TemporaryDirectory() as tmpdir:
            storage = APIKeyStorage(storage_path=f"{tmpdir}/keys.json")
            
            full_key, api_key = storage.generate_key(
                name="Permanent Key",
                scopes=["tokens:read"]
            )

            assert api_key.expires_at is None

    def test_generate_key_with_rate_limit(self):
        """Test generating key with custom rate limit."""
        with tempfile.TemporaryDirectory() as tmpdir:
            storage = APIKeyStorage(storage_path=f"{tmpdir}/keys.json")
            
            full_key, api_key = storage.generate_key(
                name="Limited Key",
                scopes=["tokens:read"],
                rate_limit=100
            )

            assert api_key.rate_limit == 100

    def test_generate_multiple_keys(self):
        """Test generating multiple unique keys."""
        with tempfile.TemporaryDirectory() as tmpdir:
            storage = APIKeyStorage(storage_path=f"{tmpdir}/keys.json")
            
            key1, model1 = storage.generate_key("Key 1", ["tokens:read"])
            key2, model2 = storage.generate_key("Key 2", ["tokens:write"])

            assert key1 != key2
            assert model1.key_id != model2.key_id
            assert model1.name != model2.name


class TestAPIKeyVerification:
    """Test API key verification."""

    def test_verify_valid_key(self):
        """Test verifying valid key."""
        with tempfile.TemporaryDirectory() as tmpdir:
            storage = APIKeyStorage(storage_path=f"{tmpdir}/keys.json")
            
            full_key, original = storage.generate_key(
                name="Test Key",
                scopes=["tokens:read"]
            )

            verified = storage.verify_key(full_key)
            assert verified is not None
            assert verified.key_id == original.key_id
            assert verified.name == original.name
            assert verified.scopes == original.scopes

    def test_verify_invalid_key(self):
        """Test verifying invalid key."""
        with tempfile.TemporaryDirectory() as tmpdir:
            storage = APIKeyStorage(storage_path=f"{tmpdir}/keys.json")
            
            verified = storage.verify_key("ng_live_invalid_key_12345")
            assert verified is None

    def test_verify_disabled_key(self):
        """Test verifying disabled key."""
        with tempfile.TemporaryDirectory() as tmpdir:
            storage = APIKeyStorage(storage_path=f"{tmpdir}/keys.json")
            
            full_key, api_key = storage.generate_key(
                name="Test Key",
                scopes=["tokens:read"]
            )

            # Disable key
            storage.revoke_key(api_key.key_id)

            # Should return None
            verified = storage.verify_key(full_key)
            assert verified is None

    def test_verify_expired_key(self):
        """Test verifying expired key."""
        with tempfile.TemporaryDirectory() as tmpdir:
            storage = APIKeyStorage(storage_path=f"{tmpdir}/keys.json")
            
            # Create key that expires in -1 days (already expired)
            full_key, api_key = storage.generate_key(
                name="Expired Key",
                scopes=["tokens:read"],
                expires_in_days=-1
            )

            verified = storage.verify_key(full_key)
            assert verified is None


class TestAPIKeyListing:
    """Test listing API keys."""

    def test_list_empty_keys(self):
        """Test listing when no keys exist."""
        with tempfile.TemporaryDirectory() as tmpdir:
            storage = APIKeyStorage(storage_path=f"{tmpdir}/keys.json")
            
            keys = storage.list_keys()
            assert len(keys) == 0

    def test_list_keys(self):
        """Test listing multiple keys."""
        with tempfile.TemporaryDirectory() as tmpdir:
            storage = APIKeyStorage(storage_path=f"{tmpdir}/keys.json")
            
            storage.generate_key("Key 1", ["tokens:read"])
            storage.generate_key("Key 2", ["tokens:write"])
            storage.generate_key("Key 3", ["tokens:admin"])

            keys = storage.list_keys()
            assert len(keys) == 3
            names = [k.name for k in keys]
            assert "Key 1" in names
            assert "Key 2" in names
            assert "Key 3" in names


class TestAPIKeyRetrieval:
    """Test retrieving specific API keys."""

    def test_get_existing_key(self):
        """Test retrieving existing key by ID."""
        with tempfile.TemporaryDirectory() as tmpdir:
            storage = APIKeyStorage(storage_path=f"{tmpdir}/keys.json")
            
            full_key, api_key = storage.generate_key(
                name="Test Key",
                scopes=["tokens:read"]
            )

            retrieved = storage.get_key(api_key.key_id)
            assert retrieved is not None
            assert retrieved.key_id == api_key.key_id
            assert retrieved.name == api_key.name

    def test_get_nonexistent_key(self):
        """Test retrieving non-existent key."""
        with tempfile.TemporaryDirectory() as tmpdir:
            storage = APIKeyStorage(storage_path=f"{tmpdir}/keys.json")
            
            retrieved = storage.get_key("nonexistent_key_id")
            assert retrieved is None


class TestAPIKeyDeletion:
    """Test deleting API keys."""

    def test_delete_existing_key(self):
        """Test deleting existing key."""
        with tempfile.TemporaryDirectory() as tmpdir:
            storage = APIKeyStorage(storage_path=f"{tmpdir}/keys.json")
            
            full_key, api_key = storage.generate_key(
                name="Test Key",
                scopes=["tokens:read"]
            )

            result = storage.delete_key(api_key.key_id)
            assert result is True

            # Should no longer exist
            retrieved = storage.get_key(api_key.key_id)
            assert retrieved is None

    def test_delete_nonexistent_key(self):
        """Test deleting non-existent key."""
        with tempfile.TemporaryDirectory() as tmpdir:
            storage = APIKeyStorage(storage_path=f"{tmpdir}/keys.json")
            
            result = storage.delete_key("nonexistent_key_id")
            assert result is False


    """Test persistence to file."""

    def test_persistence_across_instances(self):
        """Test that keys persist across storage instances."""
        with tempfile.TemporaryDirectory() as tmpdir:
            storage_path = f"{tmpdir}/keys.json"
            
            # Create key in first instance
            storage1 = APIKeyStorage(storage_path=storage_path)
            full_key, api_key = storage1.generate_key(
                name="Persistent Key",
                scopes=["tokens:read"]
            )

            # Load in second instance
            storage2 = APIKeyStorage(storage_path=storage_path)
            retrieved = storage2.verify_key(full_key)

            assert retrieved is not None
            assert retrieved.name == "Persistent Key"

    def test_json_format(self):
        """Test JSON file format."""
        with tempfile.TemporaryDirectory() as tmpdir:
            storage_path = Path(tmpdir) / "keys.json"
            storage = APIKeyStorage(storage_path=str(storage_path))
            
            storage.generate_key("Test Key", ["tokens:read"])

            # Read raw JSON
            with open(storage_path) as f:
                data = json.load(f)

            assert 'keys' in data
            assert len(data['keys']) == 1


class TestAPIKeyThreadSafety:
    """Test thread safety."""

    def test_concurrent_generation(self):
        """Test concurrent key generation."""
        import threading

        with tempfile.TemporaryDirectory() as tmpdir:
            storage = APIKeyStorage(storage_path=f"{tmpdir}/keys.json")
            keys = []

            def generate():
                full_key, api_key = storage.generate_key(
                    name=f"Key {threading.current_thread().name}",
                    scopes=["tokens:read"]
                )
                keys.append(full_key)

            threads = [threading.Thread(target=generate) for _ in range(10)]
            for t in threads:
                t.start()
            for t in threads:
                t.join()

            # All keys should be unique
            assert len(keys) == 10
            assert len(set(keys)) == 10

            # All should be in storage
            all_keys = storage.list_keys()
            assert len(all_keys) == 10

# Remove the disabled key tests and replace with revoke tests



class TestAPIKeyRevocation:
    """Test revoking API keys."""

    def test_revoke_key(self):
        """Test revoking key."""
        with tempfile.TemporaryDirectory() as tmpdir:
            storage = APIKeyStorage(storage_path=f"{tmpdir}/keys.json")
            
            full_key, api_key = storage.generate_key(
                name="Test Key",
                scopes=["tokens:read"]
            )

            result = storage.revoke_key(api_key.key_id)
            assert result is True

            # Key should still exist but be disabled
            retrieved = storage.get_key(api_key.key_id)
            assert retrieved is not None
            assert retrieved.disabled is True

    def test_revoke_nonexistent_key(self):
        """Test revoking non-existent key."""
        with tempfile.TemporaryDirectory() as tmpdir:
            storage = APIKeyStorage(storage_path=f"{tmpdir}/keys.json")
            
            result = storage.revoke_key("nonexistent_key_id")
            assert result is False


class TestAPIKeyPersistence:
    """Test persistence to file."""

    def test_persistence_across_instances(self):
        """Test that keys persist across storage instances."""
        with tempfile.TemporaryDirectory() as tmpdir:
            storage_path = f"{tmpdir}/keys.json"
            
            # Create key in first instance
            storage1 = APIKeyStorage(storage_path=storage_path)
            full_key, api_key = storage1.generate_key(
                name="Persistent Key",
                scopes=["tokens:read"]
            )

            # Load in second instance
            storage2 = APIKeyStorage(storage_path=storage_path)
            retrieved = storage2.verify_key(full_key)

            assert retrieved is not None
            assert retrieved.name == "Persistent Key"

    def test_json_format(self):
        """Test JSON file format."""
        with tempfile.TemporaryDirectory() as tmpdir:
            storage_path = Path(tmpdir) / "keys.json"
            storage = APIKeyStorage(storage_path=str(storage_path))
            
            storage.generate_key("Test Key", ["tokens:read"])

            # Read raw JSON
            with open(storage_path) as f:
                data = json.load(f)

            assert 'keys' in data
            assert len(data['keys']) == 1


class TestAPIKeyThreadSafety:
    """Test thread safety."""

    def test_concurrent_generation(self):
        """Test concurrent key generation."""
        import threading

        with tempfile.TemporaryDirectory() as tmpdir:
            storage = APIKeyStorage(storage_path=f"{tmpdir}/keys.json")
            keys = []

            def generate():
                full_key, api_key = storage.generate_key(
                    name=f"Key {threading.current_thread().name}",
                    scopes=["tokens:read"]
                )
                keys.append(full_key)

            threads = [threading.Thread(target=generate) for _ in range(10)]
            for t in threads:
                t.start()
            for t in threads:
                t.join()

            # All keys should be unique
            assert len(keys) == 10
            assert len(set(keys)) == 10

            # All should be in storage
            all_keys = storage.list_keys()
            assert len(all_keys) == 10
