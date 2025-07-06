package main

import (
	"gofr.dev/pkg/gofr"

	controller "callee-service/controller"

	extensions "callee-service/extensions"

	persistence "callee-service/persistence"
)

func main() {
	app := gofr.New()

	controller.RouteCallee(app)
	controller.RouteHealth(app)

	persistence.RoutePerson(app)

	app.UseMiddleware(extensions.PreHandle())

	app.AddStaticFiles("/", "./static")
	app.Run()
}
