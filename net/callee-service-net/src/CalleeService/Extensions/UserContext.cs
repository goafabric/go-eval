using System.Text;
using System.Text.Json;

namespace GoaFabric.CalleeService.Extensions;

/// <summary>
/// Per-request user context — equivalent of the Kotlin UserContext object (ThreadLocal).
///
/// In ASP.NET Core, AsyncLocal&lt;T&gt; is used instead of ThreadLocal because async
/// code may resume on a different thread. AsyncLocal flows the value through the
/// async call chain (like InheritableThreadLocal in Java).
/// </summary>
public static class UserContext
{
    public record UserContextRecord(string TenantId, string OrganizationId, string UserName)
    {
        public Dictionary<string, string> ToAdapterHeaderMap() => new()
        {
            ["X-TenantId"]                          = TenantId,
            ["X-OrganizationId"]                    = OrganizationId,
            ["X-Auth-Request-Preferred-Username"]   = UserName
        };
    }

    private static readonly AsyncLocal<UserContextRecord> Context = new();

    private static UserContextRecord DefaultContext => new("0", "0", "anonymous");

    public static void SetContext(IHeaderDictionary headers)
    {
        SetContext(
            headers["X-TenantId"],
            headers["X-OrganizationId"],
            headers["X-Auth-Request-Preferred-Username"],
            headers["X-UserInfo"]
        );
    }

    public static void SetContext(
        string? tenantId,
        string? organizationId,
        string? userName,
        string? userInfo)
    {
        Context.Value = new UserContextRecord(
            GetValue(tenantId, "0"),
            GetValue(organizationId, "0"),
            GetValue(GetUserNameFromUserInfo(userInfo), GetValue(userName, "anonymous"))
        );
    }

    public static void RemoveContext() => Context.Value = DefaultContext;

    public static string TenantId
    {
        get => (Context.Value ?? DefaultContext).TenantId;
        set
        {
            var current = Context.Value ?? DefaultContext;
            Context.Value = current with { TenantId = value };
        }
    }

    public static string OrganizationId => (Context.Value ?? DefaultContext).OrganizationId;
    public static string UserName       => (Context.Value ?? DefaultContext).UserName;

    public static Dictionary<string, string> AdapterHeaderMap
        => (Context.Value ?? DefaultContext).ToAdapterHeaderMap();

    private static string GetValue(string? value, string defaultValue) => value ?? defaultValue;

    private static string? GetUserNameFromUserInfo(string? userInfo)
    {
        if (userInfo is null) return null;

        try
        {
            var decoded = Encoding.UTF8.GetString(Convert.FromBase64String(
                // Pad the base64url string if needed
                userInfo.PadRight(userInfo.Length + (4 - userInfo.Length % 4) % 4, '=')));

            using var doc = JsonDocument.Parse(decoded);
            if (doc.RootElement.TryGetProperty("preferred_username", out var prop))
                return prop.GetString();
        }
        catch
        {
            // Malformed userInfo — fall through to null
        }

        return null;
    }
}
