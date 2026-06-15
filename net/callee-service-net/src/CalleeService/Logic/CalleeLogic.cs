using GoaFabric.CalleeService.Models;

namespace GoaFabric.CalleeService.Logic;

/// <summary>
/// Business logic — equivalent of the Quarkus @ApplicationScoped CalleeLogic bean.
/// Registered as a singleton in Program.cs (services.AddSingleton).
/// </summary>
public class CalleeLogic
{
    public Callee SayMyName(string name)
        => new("0", $"Your name is: {name}");

    public Callee SayMyOtherName(string name)
        => new("0", $"Your other name is: {name}");

    public Callee Save(Callee callee)
        => new("0", $"Storing your message: {callee.Message}");
}
