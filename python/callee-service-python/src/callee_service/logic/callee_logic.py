from callee_service.controller.dto.callee import Callee


class CalleeLogic:
    async def say_my_name(self, name: str | None) -> Callee:
        return Callee(id="0", message=f"Your name is: {name}")

    async def say_my_other_name(self, name: str | None) -> Callee:
        return Callee(id="0", message=f"Your other name is: {name}")

    async def save(self, callee: Callee) -> Callee:
        return Callee(id="0", message=f"Storing your message: {callee.message}")
