namespace GoaFabric.CalleeService.Models;

/// <summary>
/// Callee DTO — equivalent of the Kotlin data class Callee(id, message).
/// C# records provide the same immutability and value semantics.
/// </summary>
public record Callee(string? Id, string? Message);
