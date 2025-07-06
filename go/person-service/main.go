package main

import (
	"gofr.dev/pkg/gofr"

	controller "person-service/controller"

	extensions "person-service/extensions"
)

func main() {
	app := gofr.New()

	controller.RoutePerson(app)

	app.UseMiddleware(extensions.PreHandle())

	app.AddStaticFiles("/", "./static")
	app.Run()
}
