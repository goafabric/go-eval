using Microsoft.AspNetCore.Diagnostics;
using Microsoft.AspNetCore.Mvc;

namespace GoaFabric.CalleeService.Extensions;

/// <summary>
/// Global exception handler — equivalent of the Quarkus ExceptionMapper&lt;Exception&gt;.
///
/// Mapping:
///   ArgumentException / InvalidOperationException → 412 Precondition Failed
///   Everything else                               → 500 Internal Server Error
/// </summary>
public class GlobalExceptionHandler(ILogger<GlobalExceptionHandler> logger) : IExceptionHandler
{
    public async ValueTask<bool> TryHandleAsync(
        HttpContext httpContext,
        Exception exception,
        CancellationToken cancellationToken)
    {
        logger.LogError(exception, "{Message}", exception.Message);

        var statusCode = exception switch
        {
            ArgumentException        => StatusCodes.Status412PreconditionFailed,
            InvalidOperationException => StatusCodes.Status412PreconditionFailed,
            _                        => StatusCodes.Status500InternalServerError
        };

        var problem = new ProblemDetails
        {
            Status = statusCode,
            Title  = "An error occurred",
            Detail = $"An error occured: {exception.Message}"
        };

        httpContext.Response.StatusCode  = statusCode;
        httpContext.Response.ContentType = "application/problem+json";

        await httpContext.Response.WriteAsJsonAsync(problem, cancellationToken);
        return true;
    }
}
