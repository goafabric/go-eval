"""
Equivalent of UserContext.kt

Provides per-request multi-tenant context propagation using Python's
contextvars.ContextVar, which is the async-safe equivalent of Java's ThreadLocal.
Context is populated from HTTP headers set by an upstream proxy/OIDC provider.
"""

import base64
import json
import logging
from contextvars import ContextVar
from dataclasses import dataclass

from fastapi import Request

logger = logging.getLogger(__name__)


@dataclass(frozen=True)
class UserContextRecord:
    tenant_id: str
    organization_id: str
    user_name: str

    def to_adapter_header_map(self) -> dict[str, str]:
        return {
            "X-TenantId": self.tenant_id,
            "X-OrganizationId": self.organization_id,
            "X-Auth-Request-Preferred-Username": self.user_name,
        }


_DEFAULT_CONTEXT = UserContextRecord(
    tenant_id="0",
    organization_id="0",
    user_name="anonymous",
)

_CONTEXT: ContextVar[UserContextRecord] = ContextVar("user_context", default=_DEFAULT_CONTEXT)


def set_context_from_request(request: Request) -> None:
    headers = request.headers
    set_context(
        tenant_id=headers.get("X-TenantId"),
        organization_id=headers.get("X-OrganizationId"),
        user_name=headers.get("X-Auth-Request-Preferred-Username"),
        user_info=headers.get("X-UserInfo"),
    )


def set_context(
    tenant_id: str | None,
    organization_id: str | None,
    user_name: str | None,
    user_info: str | None = None,
) -> None:
    resolved_user_name = _get_username_from_user_info(user_info) or user_name or "anonymous"
    _CONTEXT.set(
        UserContextRecord(
            tenant_id=tenant_id or "0",
            organization_id=organization_id or "0",
            user_name=resolved_user_name,
        )
    )


def reset_context() -> None:
    _CONTEXT.set(_DEFAULT_CONTEXT)


def get_tenant_id() -> str:
    return _CONTEXT.get().tenant_id


def get_organization_id() -> str:
    return _CONTEXT.get().organization_id


def get_user_name() -> str:
    return _CONTEXT.get().user_name


def get_adapter_header_map() -> dict[str, str]:
    return _CONTEXT.get().to_adapter_header_map()


def _get_username_from_user_info(user_info: str | None) -> str | None:
    if not user_info:
        return None
    try:
        decoded = base64.urlsafe_b64decode(user_info + "==")
        data: dict = json.loads(decoded)
        return data.get("preferred_username")
    except Exception:
        logger.debug("Failed to decode X-UserInfo header", exc_info=True)
        return None
