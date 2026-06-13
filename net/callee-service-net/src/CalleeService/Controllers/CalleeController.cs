using GoaFabric.CalleeService.Logic;
using GoaFabric.CalleeService.Models;
using ModelContextProtocol.Server;
using Microsoft.AspNetCore.Mvc;
using System.ComponentModel;

namespace GoaFabric.CalleeService.Controllers;

/// <summary>
/// REST controller — equivalent of the Quarkus JAX-RS CalleeController.
///
/// Endpoints:
///   GET  /callees/sayMyName?name=X
///   GET  /callees/sayMyOtherName/{name}
///   POST /callees/save  (body: Callee JSON)
///
/// MCP tools expose the GET endpoints to AI clients via the Model Context Protocol.
/// </summary>
[ApiController]
[Route("callees")]
[Produces("application/json")]
public class CalleeController(CalleeLogic calleeLogic) : ControllerBase
{
    [HttpGet("sayMyName")]
    public Callee SayMyName([FromQuery] string name)
        => calleeLogic.SayMyName(name);

    [HttpGet("sayMyOtherName/{name}")]
    public Callee SayMyOtherName([FromRoute] string name)
        => calleeLogic.SayMyOtherName(name);

    [HttpPost("save")]
    [Consumes("application/json")]
    public Callee Save([FromBody] Callee callee)
        => calleeLogic.Save(callee);
}

/// <summary>
/// MCP Tool definitions — equivalent of the @Tool annotations on CalleeController.
/// Registered via .WithToolsFromAssembly() in Program.cs.
/// </summary>
[McpServerToolType]
public class CalleeTools(CalleeLogic calleeLogic)
{
    [McpServerTool, Description("say my name")]
    public Callee SayMyName([Description("The name to say")] string name)
        => calleeLogic.SayMyName(name);

    [McpServerTool, Description("say my other name")]
    public Callee SayMyOtherName([Description("The other name to say")] string name)
        => calleeLogic.SayMyOtherName(name);
}
