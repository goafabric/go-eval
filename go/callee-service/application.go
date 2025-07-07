package main

import (
	"gofr.dev/pkg/gofr"

	"callee-service/callee"

	extensions "callee-service/extensions"
)

func main() {
	app := gofr.New()

	callee.RouteCallee(app)

	app.UseMiddleware(extensions.PreHandle())

	app.AddStaticFiles("/", "./static")
	app.Run()
}
