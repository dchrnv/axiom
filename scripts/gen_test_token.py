import sys
from pathlib import Path
from datetime import datetime, timedelta
import jwt

# Add src to path
sys.path.insert(0, str(Path(__file__).parent.parent))

SECRET_KEY = "CHANGE_THIS_SECRET_KEY_IN_PRODUCTION"
ALGORITHM = "HS256"


def generate_admin_token():
    now = datetime.utcnow()
    expire = now + timedelta(hours=1)
    payload = {
        "sub": "admin",
        "scopes": ["admin", "status:read", "metrics:read"],
        "exp": expire,
        "iat": now,
        "token_type": "access",
    }
    return jwt.encode(payload, SECRET_KEY, algorithm=ALGORITHM)


if __name__ == "__main__":
    print(generate_admin_token())
