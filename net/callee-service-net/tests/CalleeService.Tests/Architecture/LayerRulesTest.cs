using FluentAssertions;
using NetArchTest.Rules;
using Xunit;

namespace GoaFabric.CalleeService.Tests.Architecture;

/// <summary>
/// Layered architecture rules — equivalent of the Quarkus ArchUnit ControllerRulesTest
/// and PersistenceRulesTest combined.
///
/// Rules enforced:
///  1. Logic layer must NOT depend on Controllers layer
///  2. Classes in Controllers namespace must be named *Controller (or *Tools for MCP)
/// </summary>
public class LayerRulesTest
{
    // Program is a top-level class (no namespace) — reference via a known namespaced type
    private static readonly Types AppTypes =
        Types.InAssembly(typeof(GoaFabric.CalleeService.Logic.CalleeLogic).Assembly);

    [Fact]
    public void LogicMustNotDependOnControllers()
    {
        var result = AppTypes
            .That().ResideInNamespace("GoaFabric.CalleeService.Logic")
            .ShouldNot().HaveDependencyOn("GoaFabric.CalleeService.Controllers")
            .GetResult();

        result.IsSuccessful.Should().BeTrue(
            because: "Logic layer must not depend on Controllers layer");
    }

    [Fact]
    public void ControllerClassesMustBeNamedCorrectly()
    {
        var result = AppTypes
            .That().ResideInNamespace("GoaFabric.CalleeService.Controllers")
            .And().AreClasses()
            .Should().HaveNameEndingWith("Controller").Or().HaveNameEndingWith("Tools")
            .GetResult();

        result.IsSuccessful.Should().BeTrue(
            because: "classes in the Controllers namespace must end with 'Controller' or 'Tools'");
    }
}
