# ── Stage 1: Build ────────────────────────────────────────────────────────────
FROM mcr.microsoft.com/dotnet/sdk:10.0-alpine AS build
WORKDIR /src

# Restore dependencies (layer-cached)
COPY src/CalleeService/CalleeService.csproj src/CalleeService/
RUN dotnet restore src/CalleeService/CalleeService.csproj

# Copy everything and publish
COPY . .
RUN dotnet publish src/CalleeService/CalleeService.csproj \
        --configuration Release \
        --output /app/publish \
        --no-restore

# ── Stage 2: Runtime ──────────────────────────────────────────────────────────
FROM mcr.microsoft.com/dotnet/aspnet:10.0-alpine AS runtime
WORKDIR /app

# Non-root user for security
RUN addgroup -S appgroup && adduser -S appuser -G appgroup
USER appuser

COPY --from=build /app/publish .

ENV ASPNETCORE_URLS=http://+:50900
ENV ASPNETCORE_ENVIRONMENT=Production

EXPOSE 50900

ENTRYPOINT ["dotnet", "CalleeService.dll"]
