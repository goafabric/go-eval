package extensions

import "net/http"
import gofrHTTP "gofr.dev/pkg/gofr/http"

func PreHandle() gofrHTTP.Middleware {
	return func(inner http.Handler) http.Handler {
		return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		    context := UserContext {TenantId: r.Header.Get("X-TenantId"),
		                            OrganizationId: r.Header.Get("OrganizationId"),
		                            UserName: r.Header.Get("X-Auth-Request-Preferred-Username")}

            println("Intercepted request:", r.URL.Path)
            println("TenantId:", context.TenantId)

			inner.ServeHTTP(w, r)
		})
	}
}