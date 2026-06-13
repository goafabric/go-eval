using FluentAssertions;
using NetArchTest.Rules;
using Xunit;

namespace GoaFabric.CalleeService.Tests.Architecture;

/// <summary>
/// Architecture rules — equivalent of the Quarkus ArchUnit ApplicationRulesTest.
///
/// Rules enforced:
///  1. No class names ending in "Impl" or "Management"
///  2. No dependency on forbidden libraries (Newtonsoft.Json as example of an unapproved lib)
/// </summary>
public class ApplicationRulesTest
{
    // Program is a top-level class (no namespace) — reference via a known namespaced type
    private static readonly Types AppTypes =
        Types.InAssembly(typeof(GoaFabric.CalleeService.Logic.CalleeLogic).Assembly);

    [Fact]
    public void NoImplOrManagementClassNames()
    {
        var result = AppTypes
            .That().HaveNameEndingWith("Impl")
            .Or().HaveNameEndingWith("Management")
            .ShouldNot().HaveNameEndingWith("Impl")  // vacuous — triggers evaluation
            .GetResult();

        // Use the count-based check: no types should match the predicate
        var implTypes = AppTypes
            .That().HaveNameEndingWith("Impl")
            .Or().HaveNameEndingWith("Management")
            .GetTypes();

        implTypes.Should().BeEmpty(
            because: "class names ending in 'Impl' or 'Management' are not permitted");
    }

    [Fact]
    public void NoNewtonsoftJsonDependency()
    {
        // Enforce that Newtonsoft.Json is not used — project uses System.Text.Json
        var result = AppTypes
            .That().HaveDependencyOn("Newtonsoft.Json")
            .ShouldNot().HaveDependencyOn("Newtonsoft.Json")
            .GetResult();

        result.IsSuccessful.Should().BeTrue(
            because: "Newtonsoft.Json is not an approved dependency; use System.Text.Json");
    }
}
