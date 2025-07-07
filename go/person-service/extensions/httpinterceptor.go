package extensions

import (
    "net/http"
    "context"
    "fmt"
    gofrHTTP "gofr.dev/pkg/gofr/http"
)

func PreHandle() gofrHTTP.Middleware {
	return func(inner http.Handler) http.Handler {
		return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
            userContext := UserContext{
                TenantId:       r.Header.Get("X-TenantId"),
                OrganizationId: r.Header.Get("OrganizationId"),
                UserName:       r.Header.Get("X-Auth-Request-Preferred-Username"),
            }

            fmt.Printf("Intercepted request: %s | TenantId: %s | UserName: %s\n",
                r.URL.Path, userContext.TenantId, userContext.UserName)

            // Attach UserContext to request context and pass it down the request chain
            ctx := context.WithValue(r.Context(), "userContext", userContext)
            inner.ServeHTTP(w, r.WithContext(ctx))
        })
	}
}