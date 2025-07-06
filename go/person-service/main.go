package main

import (
	"gofr.dev/pkg/gofr"

	"person-service/extensions"
	"person-service/person"
)

func main() {
	app := gofr.New()

	person.RoutePerson(app)

	app.UseMiddleware(extensions.PreHandle())

	app.AddStaticFiles("/", "./static")
	app.Run()
}
