using System.Diagnostics;
using Serilog.Context;

namespace GoaFabric.CalleeService.Extensions;

/// <summary>
/// Logs each incoming HTTP request with the user from UserContext.
/// Pushes TenantId into the Serilog LogContext (equivalent of MDC in the Quarkus interceptor).
/// Enriches the active OTel span with tenant.id (equivalent of
/// Span.fromContext(Context.current()).setAttribute("tenant.id", ...) in HttpInterceptor).
///
/// Equivalent of the logging/tracing parts of the Quarkus HttpInterceptor.
/// </summary>
public class RequestLoggingMiddleware(RequestDelegate next, ILogger<RequestLoggingMiddleware> logger)
{
    public async Task InvokeAsync(HttpContext context)
    {
        // Push tenantId into Serilog log context (equivalent of MDC.put("tenantId", ...))
        using (LogContext.PushProperty("TenantId", UserContext.TenantId))
        {
            // Enrich the active OTel span with tenant.id
            // (equivalent of Span.fromContext(Context.current()).setAttribute("tenant.id", ...) in Quarkus)
            Activity.Current?.SetTag("tenant.id", UserContext.TenantId);

            var endpoint = context.Request.Path;
            logger.LogInformation("{Endpoint} http call for user {UserName}", endpoint, UserContext.UserName);
            await next(context);
        }
    }
}
