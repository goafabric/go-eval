package extensions

import "net/http"
import gofrHTTP "gofr.dev/pkg/gofr/http"

func PreHandle() gofrHTTP.Middleware {
	return func(inner http.Handler) http.Handler {
		return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		    println("Intercepted request:", r.URL.Path)
			inner.ServeHTTP(w, r)
		})
	}
}