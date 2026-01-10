"""
Axiom Data Models

Pydantic models for request/response objects.
"""

from datetime import datetime
from typing import Optional, List, Dict, Any
from pydantic import BaseModel, Field, ConfigDict


# ============================================================================
# Authentication Models
# ============================================================================


class User(BaseModel):
    """User model."""

    user_id: str
    username: str
    email: Optional[str] = None
    full_name: Optional[str] = None
    role: str
    scopes: List[str] = []
    disabled: bool = False
    created_at: Optional[datetime] = None

    model_config = ConfigDict(from_attributes=True)


class LoginResponse(BaseModel):
    """Login response with JWT tokens."""

    access_token: str
    refresh_token: str
    token_type: str = "bearer"
    expires_in: int
    user: User


# ============================================================================
# API Key Models
# ============================================================================


class APIKey(BaseModel):
    """API Key model."""

    key_id: str
    name: str
    key_prefix: str
    scopes: List[str] = []
    created_at: datetime
    expires_at: Optional[datetime] = None
    last_used_at: Optional[datetime] = None
    disabled: bool = False
    api_key: Optional[str] = None  # Only populated on creation

    model_config = ConfigDict(from_attributes=True)


class APIKeyCreate(BaseModel):
    """Request to create API key."""

    name: str = Field(..., description="Descriptive name for the key")
    scopes: Optional[List[str]] = Field(None, description="List of scopes")
    expires_in_days: Optional[int] = Field(None, description="Expiration in days")


class APIKeyCreated(BaseModel):
    """Response when API key is created (includes full key)."""

    key_id: str
    name: str
    api_key: str  # Full key, only shown once!
    key_prefix: str
    scopes: List[str]
    created_at: datetime
    expires_at: Optional[datetime] = None
    disabled: bool = False
    last_used_at: Optional[datetime] = None


# ============================================================================
# Token Models
# ============================================================================


class Coordinates(BaseModel):
    """Coordinates for a single dimension."""

    x: Optional[float] = None
    y: Optional[float] = None
    z: Optional[float] = None


class Token(BaseModel):
    """Token model."""

    id: int = Field(..., description="Token ID (32-bit)")
    id_hex: str = Field(..., description="Token ID in hex format")
    local_id: int = Field(..., description="Local ID (24 bits)")
    entity_type: int = Field(..., description="Entity type (4 bits)")
    domain: int = Field(..., description="Domain (4 bits)")
    weight: float = Field(..., description="Token weight")
    field_radius: float = Field(..., description="Field radius")
    field_strength: float = Field(..., description="Field strength")
    timestamp: int = Field(..., description="Creation timestamp")
    age_seconds: int = Field(..., description="Age in seconds")
    flags: Dict[str, bool] = Field(..., description="Token flags")
    coordinates: Dict[str, Optional[List[float]]] = Field(
        ..., description="Coordinates in 8 spaces"
    )

    model_config = ConfigDict(from_attributes=True)


class TokenCreate(BaseModel):
    """Request to create a token."""

    entity_type: int = Field(default=0, ge=0, le=15, description="Entity type (0-15)")
    domain: int = Field(default=0, ge=0, le=15, description="Domain (0-15)")
    weight: float = Field(default=0.5, ge=0.0, le=1.0, description="Token weight")
    field_radius: float = Field(default=1.0, ge=0.0, le=2.55, description="Field radius")
    field_strength: float = Field(default=1.0, ge=0.0, le=1.0, description="Field strength")
    persistent: bool = Field(default=False, description="Should token persist")

    # 8 levels of coordinates
    l1_physical: Optional[Coordinates] = None
    l2_sensory: Optional[Coordinates] = None
    l3_motor: Optional[Coordinates] = None
    l4_emotional: Optional[Coordinates] = None
    l5_cognitive: Optional[Coordinates] = None
    l6_social: Optional[Coordinates] = None
    l7_temporal: Optional[Coordinates] = None
    l8_abstract: Optional[Coordinates] = None


class TokenUpdate(BaseModel):
    """Request to update a token."""

    weight: Optional[float] = Field(None, ge=0.0, le=1.0, description="Token weight")
    field_radius: Optional[float] = Field(None, ge=0.0, le=2.55, description="Field radius")
    field_strength: Optional[float] = Field(None, ge=0.0, le=1.0, description="Field strength")

    # 8 levels of coordinates
    l1_physical: Optional[Coordinates] = None
    l2_sensory: Optional[Coordinates] = None
    l3_motor: Optional[Coordinates] = None
    l4_emotional: Optional[Coordinates] = None
    l5_cognitive: Optional[Coordinates] = None
    l6_social: Optional[Coordinates] = None
    l7_temporal: Optional[Coordinates] = None
    l8_abstract: Optional[Coordinates] = None


class TokenQuery(BaseModel):
    """Query for similar tokens."""

    query_vector: List[float] = Field(..., description="Query embedding vector")
    top_k: int = Field(10, ge=1, le=1000, description="Number of results to return")
    threshold: Optional[float] = Field(None, description="Similarity threshold")


class TokenQueryResult(BaseModel):
    """Token query result with similarity."""

    token_id: Optional[int] = None
    label: str
    score: float
    entity_type: str = "Concept"
    coordinates: Optional[Dict[str, Optional[List[float]]]] = None


# ============================================================================
# Grid Models
# ============================================================================


class GridCell(BaseModel):
    """Grid cell model."""

    x: float
    y: float
    z: float
    token_ids: List[int] = []
    density: float = 0.0

    model_config = ConfigDict(from_attributes=True)


class GridQuery(BaseModel):
    """Grid spatial query."""

    center: List[float] = Field(..., description="Center point [x, y, z]")
    radius: float = Field(..., description="Search radius")
    max_results: int = Field(100, ge=1, le=10000)


class GridQueryResult(BaseModel):
    """Grid query result."""

    cells: List[GridCell]
    total_count: int
    query_time_ms: float


# ============================================================================
# CDNA Models
# ============================================================================


class CDNAConfig(BaseModel):
    """CDNA configuration model."""

    profile: str = "explorer"
    curiosity_scale: float = 1.0
    novelty_scale: float = 1.0
    affinity_scale: float = 1.0

    model_config = ConfigDict(from_attributes=True)


class CDNAUpdate(BaseModel):
    """CDNA configuration update."""

    curiosity_scale: Optional[float] = None
    novelty_scale: Optional[float] = None
    affinity_scale: Optional[float] = None


# ============================================================================
# Response Models
# ============================================================================


class SuccessResponse(BaseModel):
    """Generic success response."""

    success: bool = True
    message: str
    data: Optional[Dict[str, Any]] = None


class ErrorResponse(BaseModel):
    """Generic error response."""

    success: bool = False
    error: Dict[str, Any]


class PaginatedResponse(BaseModel):
    """Paginated response wrapper."""

    items: List[Any]
    total: int
    page: int
    page_size: int
    total_pages: int


# ============================================================================
# Health & Status Models
# ============================================================================


class HealthStatus(BaseModel):
    """Health check response."""

    status: str
    version: str
    uptime_seconds: Optional[float] = None
    timestamp: Optional[datetime] = None


class SystemStatus(BaseModel):
    """System status response."""

    state: str
    uptime_seconds: float
    tokens: Dict[str, int]
    connections: Dict[str, int]
    memory_usage_mb: float
    cpu_usage_percent: float
    components: Dict[str, str]
    version: Optional[str] = None
    storage_backend: Optional[str] = None
    cdna_profile: Optional[str] = None
