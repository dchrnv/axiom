# API Key Rotation Guide

## Overview

API key rotation is a security best practice that involves regularly replacing API keys with new ones. This guide explains how to rotate API keys in NeuroGraph.

## Why Rotate API Keys?

1. **Minimize exposure window** - Limit the time a compromised key can be used
2. **Compliance** - Many security standards require regular key rotation
3. **Proactive security** - Reduce risk before a breach occurs
4. **Audit trail** - Track which keys were used when

## Recommended Rotation Schedule

- **High-security environments:** Every 30 days
- **Production systems:** Every 90 days (recommended)
- **Development/testing:** Every 180 days
- **After suspected compromise:** Immediately

## How to Rotate an API Key

### Via REST API

```bash
# 1. Get your current API key ID
curl -X GET http://localhost:8000/api/v1/api-keys \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"

# 2. Rotate the key
curl -X POST http://localhost:8000/api/v1/api-keys/{KEY_ID}/rotate \
  -H "Authorization: Bearer YOUR_JWT_TOKEN" \
  -H "Content-Type: application/json"

# Response:
{
  "success": true,
  "data": {
    "key": "ngapi_new_key_here_shown_only_once",
    "key_id": "new_key_id",
    "name": "My API Key (rotated)",
    "created_at": "2026-01-12T...",
    "scopes": ["read:tokens", "write:tokens"],
    "old_key_id": "old_key_id",
    "old_key_status": "revoked",
    "message": "API key rotated successfully. Store the new key securely!"
  }
}
```

### Via Python SDK

```python
import requests

# Rotate API key
response = requests.post(
    f"http://localhost:8000/api/v1/api-keys/{key_id}/rotate",
    headers={"Authorization": f"Bearer {jwt_token}"}
)

new_key = response.json()["data"]["key"]
print(f"New API key: {new_key}")
print("⚠️  Store this key securely - it won't be shown again!")
```

## What Happens During Rotation?

1. **New key generation**
   - Creates a new API key with the same permissions
   - Inherits scopes and rate limits from old key
   - Named as "{original_name} (rotated)"

2. **Old key revocation**
   - Marks the old key as revoked
   - Old key can no longer be used for authentication
   - Remains in storage for audit purposes

3. **Response**
   - Returns the new full API key (shown only once!)
   - Provides metadata about both old and new keys
   - Logs the rotation event with user info

## Migration Process

To ensure zero-downtime key rotation:

### 1. Pre-rotation
```bash
# Record your current key ID
export OLD_KEY_ID="abc123"
export OLD_KEY="ngapi_old_key_here"
```

### 2. Rotate
```bash
# Rotate the key
NEW_KEY_RESPONSE=$(curl -X POST \
  http://localhost:8000/api/v1/api-keys/${OLD_KEY_ID}/rotate \
  -H "Authorization: Bearer ${JWT_TOKEN}")

# Extract new key
NEW_KEY=$(echo $NEW_KEY_RESPONSE | jq -r '.data.key')
echo "New key: $NEW_KEY"
```

### 3. Update applications
```bash
# Update environment variables
export API_KEY="$NEW_KEY"

# Update .env files
echo "API_KEY=$NEW_KEY" >> .env.production

# Restart services that use the key
systemctl restart my-app
```

### 4. Verify
```bash
# Test new key works
curl -X GET http://localhost:8000/api/v1/tokens \
  -H "X-API-Key: $NEW_KEY"

# Verify old key is revoked
curl -X GET http://localhost:8000/api/v1/tokens \
  -H "X-API-Key: $OLD_KEY"
# Should return 401 Unauthorized
```

## Automated Rotation

For production environments, consider automating key rotation:

### Cron Job Example

```bash
#!/bin/bash
# rotate-api-keys.sh

# Load current credentials
source /etc/neurograph/.env

# Rotate key
NEW_KEY=$(curl -s -X POST \
  http://localhost:8000/api/v1/api-keys/${KEY_ID}/rotate \
  -H "Authorization: Bearer ${JWT_TOKEN}" | jq -r '.data.key')

# Update environment file
sed -i "s/API_KEY=.*/API_KEY=${NEW_KEY}/" /etc/neurograph/.env

# Restart services
systemctl reload my-app

# Log rotation
echo "$(date): API key rotated successfully" >> /var/log/neurograph/key-rotation.log
```

Add to crontab for quarterly rotation:
```bash
# Rotate API keys every 90 days at 2 AM
0 2 1 */3 * /usr/local/bin/rotate-api-keys.sh
```

## Security Considerations

### Do's ✅

- **Rotate regularly** - Set up automated rotation schedule
- **Store securely** - Use secrets manager (Vault, AWS Secrets Manager, etc.)
- **Monitor usage** - Check for unexpected API key usage
- **Log rotations** - Maintain audit trail of all rotations
- **Test new keys** - Verify new key works before revoking old one

### Don'ts ❌

- **Don't hardcode keys** - Always use environment variables
- **Don't share keys** - Each service should have its own key
- **Don't skip verification** - Always test the new key works
- **Don't lose keys** - New key is shown only once!
- **Don't delay rotation** - Rotate immediately if compromise suspected

## Troubleshooting

### "Key not found"
- Verify the key ID exists: `GET /api/v1/api-keys`
- Check you have admin permissions

### "Permission denied"
- Requires `admin:config` permission
- Check your JWT token has correct scopes

### "Failed to rotate"
- Check API server logs for details
- Verify API key storage is accessible
- Ensure old key is not already revoked

## Audit Trail

All API key rotations are logged with:
- Event type: `api_key_rotated`
- Old key ID
- New key ID
- User who performed rotation
- Timestamp

Query rotation history:
```bash
# View recent rotations
grep "api_key_rotated" /var/log/neurograph/app.log | tail -10
```

## Best Practices

1. **Schedule regular rotations** - Don't wait for a breach
2. **Automate the process** - Reduce human error
3. **Test rotation process** - Practice in staging first
4. **Monitor for issues** - Set up alerts for rotation failures
5. **Document your keys** - Keep inventory of which keys are used where
6. **Use different keys** - One key per service/environment
7. **Rotate after team changes** - When people leave or change roles

## Examples

### Development Environment
```bash
# Rotate dev API key (every 180 days)
curl -X POST http://localhost:8000/api/v1/api-keys/dev_key/rotate \
  -H "Authorization: Bearer ${DEV_TOKEN}"
```

### Production Environment
```bash
# Rotate production API key (every 90 days)
curl -X POST https://api.neurograph.prod/api/v1/api-keys/prod_key/rotate \
  -H "Authorization: Bearer ${PROD_TOKEN}"
```

### Emergency Rotation (Suspected Compromise)
```bash
# Immediate rotation - no delay!
for KEY_ID in $(cat /etc/neurograph/key_ids.txt); do
  echo "Rotating $KEY_ID..."
  curl -X POST https://api.neurograph.prod/api/v1/api-keys/${KEY_ID}/rotate \
    -H "Authorization: Bearer ${ADMIN_TOKEN}"
done
```

## See Also

- [API Keys Management](../src/api/routers/api_keys.py)
- [Security Policy](../SECURITY.md)
- [Security Audit](SECURITY_AUDIT.md)

---

**Last Updated:** 2026-01-12  
**Version:** v0.67.3
