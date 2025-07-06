package main

import "gofr.dev/pkg/gofr"

import controller "callee-service/controller"
import extensions "callee-service/extensions"



func main() {
    app := gofr.New()

    controller.RouteCallee(app)
    controller.RouteHealth(app)

    app.UseMiddleware(extensions.PreHandle())

    app.AddStaticFiles("/", "./static")
    app.Run()
}

