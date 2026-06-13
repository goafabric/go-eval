using System.Net;
using FluentAssertions;
using Microsoft.AspNetCore.Mvc.Testing;
using Xunit;

namespace GoaFabric.CalleeService.Tests.Controller;

/// <summary>
/// Controller integration tests — equivalent of the Quarkus @QuarkusTest CalleeControllerTest.
/// Uses WebApplicationFactory to spin up the full ASP.NET Core pipeline in-process.
/// </summary>
public class CalleeControllerTest(WebApplicationFactory<Program> factory)
    : IClassFixture<WebApplicationFactory<Program>>
{
    private readonly HttpClient _client = factory.CreateClient();

    [Fact]
    public async Task SayMyName()
    {
        var response = await _client.GetAsync("/callees/sayMyName?name=Heisenberg");
        response.StatusCode.Should().Be(HttpStatusCode.OK);
    }

    [Fact]
    public async Task SayMyOtherName()
    {
        var response = await _client.GetAsync("/callees/sayMyOtherName/Andreas");
        response.StatusCode.Should().Be(HttpStatusCode.OK);
    }

    [Fact]
    public async Task Save()
    {
        var callee = new StringContent(
            """{"id":null,"message":"Hello World"}""",
            System.Text.Encoding.UTF8,
            "application/json");

        var response = await _client.PostAsync("/callees/save", callee);
        response.StatusCode.Should().Be(HttpStatusCode.OK);
    }

    [Fact]
    public async Task HealthCheck()
    {
        var response = await _client.GetAsync("/actuator/health");
        response.StatusCode.Should().Be(HttpStatusCode.OK);
    }
}
