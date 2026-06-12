from pydantic import BaseModel


class Callee(BaseModel):
    id: str | None = None
    message: str | None = None
