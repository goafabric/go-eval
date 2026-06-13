namespace GoaFabric.CalleeService.Extensions;

/// <summary>
/// Sets and clears the UserContext for each HTTP request.
/// Equivalent of the ContainerRequestFilter / ContainerResponseFilter parts of
/// the Quarkus HttpInterceptor.
/// </summary>
public class UserContextMiddleware(RequestDelegate next)
{
    public async Task InvokeAsync(HttpContext context)
    {
        UserContext.SetContext(context.Request.Headers);
        try
        {
            await next(context);
        }
        finally
        {
            UserContext.RemoveContext();
        }
    }
}
