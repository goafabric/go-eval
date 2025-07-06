package main

import "gofr.dev/pkg/gofr"

import controller "callee-service/controller"
import extensions "callee-service/extensions"

import persistence "callee-service/persistence"


func main() {
    app := gofr.New()

    controller.RouteCallee(app)
    controller.RouteHealth(app)

    persistence.RoutePerson(app)

    app.UseMiddleware(extensions.PreHandle())

    app.AddStaticFiles("/", "./static")
    app.Run()
}

