using GoaFabric.CalleeService.Extensions;
using GoaFabric.CalleeService.Logic;
using Microsoft.AspNetCore.Diagnostics.HealthChecks;
using Microsoft.Extensions.Diagnostics.HealthChecks;
using OpenTelemetry.Resources;
using OpenTelemetry.Trace;
using Prometheus;
using Serilog;
using Serilog.Enrichers.Span;

var builder = WebApplication.CreateBuilder(args);

// ── Serilog ──────────────────────────────────────────────────────────────────
builder.Host.UseSerilog((ctx, cfg) =>
    cfg.ReadFrom.Configuration(ctx.Configuration)
       .Enrich.FromLogContext()
       .Enrich.WithThreadId()
       .Enrich.WithSpan()
       .WriteTo.Console(
           outputTemplate: "{Timestamp:HH:mm:ss} {Level:u5} tenantId={TenantId} [{TraceId}] [{SourceContext}] ({ThreadId}) {Message:lj}{NewLine}{Exception}"
       ));

// ── Application services ──────────────────────────────────────────────────────
builder.Services.AddSingleton<CalleeLogic>();

// ── Exception handling ────────────────────────────────────────────────────────
builder.Services.AddExceptionHandler<GlobalExceptionHandler>();
builder.Services.AddProblemDetails();

// ── HTTP / Controllers ────────────────────────────────────────────────────────
builder.Services.AddControllers();
builder.Services.AddHttpContextAccessor();

// ── OpenAPI / Swagger ─────────────────────────────────────────────────────────
builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen(c =>
{
    c.SwaggerDoc("v1", new() { Title = "Callee Service", Version = "v1" });
});

// ── Health checks ─────────────────────────────────────────────────────────────
builder.Services.AddHealthChecks();

// ── MCP Server (Model Context Protocol) ──────────────────────────────────────
builder.Services.AddMcpServer()
    .WithHttpTransport()
    .WithToolsFromAssembly();

// ── Prometheus metrics ────────────────────────────────────────────────────────
builder.Services.AddSingleton<IMetricFactory>(_ => Metrics.DefaultFactory);

// ── OpenTelemetry tracing ─────────────────────────────────────────────────────
var appName = builder.Configuration["ApplicationName"] ?? builder.Environment.ApplicationName;
var otlpEndpoint = builder.Configuration["Otel:Exporter:Otlp:Traces:Endpoint"] ?? "http://localhost:4317";

builder.Services.AddOpenTelemetry()
    .ConfigureResource(r => r.AddService(appName))
    .WithTracing(tracing => tracing
        .AddAspNetCoreInstrumentation()
        .AddOtlpExporter(o => o.Endpoint = new Uri(otlpEndpoint)));

// ── Build ─────────────────────────────────────────────────────────────────────
var app = builder.Build();

// ── Middleware pipeline ───────────────────────────────────────────────────────
app.UseExceptionHandler();
app.UseMiddleware<UserContextMiddleware>();
app.UseMiddleware<RequestLoggingMiddleware>();

app.UseRouting();

// Health checks at Spring Boot-compatible paths
app.MapHealthChecks("/actuator/health", new HealthCheckOptions
{
    ResultStatusCodes =
    {
        [HealthStatus.Healthy]   = StatusCodes.Status200OK,
        [HealthStatus.Degraded]  = StatusCodes.Status200OK,
        [HealthStatus.Unhealthy] = StatusCodes.Status503ServiceUnavailable
    }
});
app.MapHealthChecks("/actuator/health/liveness",  new HealthCheckOptions { Predicate = _ => false });
app.MapHealthChecks("/actuator/health/readiness", new HealthCheckOptions { Predicate = _ => false });

// Prometheus metrics
app.UseHttpMetrics();
app.MapMetrics("/actuator/prometheus");

// Swagger — controlled by config (default: on; set Swagger:AlwaysInclude=false to disable)
app.UseSwagger();
app.UseSwaggerUI(c => c.SwaggerEndpoint("/swagger/v1/swagger.json", "Callee Service v1"));

// MCP endpoint
app.MapMcp("/mcp");

app.MapControllers();

// Static files (index.html landing page — served only at root to avoid
// intercepting Swagger UI's embedded static resources)
app.UseStaticFiles();
app.MapGet("/", () => Results.File("index.html", "text/html"));

app.Run();

// Make Program accessible in integration tests
public partial class Program { }
