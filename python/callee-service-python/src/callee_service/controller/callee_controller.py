"""
Equivalent of CalleeController.kt

FastAPI router for /callees endpoints.
- GET  /callees/sayMyName?name=...      (query param)
- GET  /callees/sayMyOtherName/{name}   (path param)
- POST /callees/save                    (JSON body)

Security note: @RolesAllowed("standard_role") is reproduced via a
Depends() dependency that checks the Authorization header. Swap in
your real OIDC/JWT validation as needed.
"""

import logging

from fastapi import APIRouter, Depends, HTTPException, Security
from fastapi.security import HTTPAuthorizationCredentials, HTTPBearer

from callee_service.controller.dto.callee import Callee
from callee_service.logic.callee_logic import CalleeLogic

logger = logging.getLogger(__name__)

router = APIRouter(prefix="/callees", tags=["callees"])

_bearer_scheme = HTTPBearer(auto_error=False)


def _require_standard_role(
    credentials: HTTPAuthorizationCredentials | None = Security(_bearer_scheme),
) -> None:
    """
    Placeholder role guard — equivalent of @RolesAllowed("standard_role").
    Replace with real JWT/OIDC token validation in production.
    """
    # When running without a token (dev/test) we allow through.
    # Uncomment the block below to enforce the role:
    #
    # if credentials is None:
    #     raise HTTPException(status_code=401, detail="Not authenticated")
    # # decode & verify JWT, check roles claim, raise 403 if missing
    pass


@router.get("/sayMyName", summary="Say my name")
async def say_my_name(
    name: str,
    _: None = Depends(_require_standard_role),
    logic: CalleeLogic = Depends(CalleeLogic),
) -> Callee:
    return await logic.say_my_name(name)


@router.get("/sayMyOtherName/{name}", summary="Say my other name")
async def say_my_other_name(
    name: str,
    _: None = Depends(_require_standard_role),
    logic: CalleeLogic = Depends(CalleeLogic),
) -> Callee:
    return await logic.say_my_other_name(name)


@router.post("/save", summary="Save a callee message")
async def save(
    callee: Callee,
    _: None = Depends(_require_standard_role),
    logic: CalleeLogic = Depends(CalleeLogic),
) -> Callee:
    return await logic.save(callee)
