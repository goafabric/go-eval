package main

import controller "callee-service/controller"
import "gofr.dev/pkg/gofr"

func main() {
    app := gofr.New()

    controller.RouteCallee(app)
    controller.RouteHealth(app)

    app.AddStaticFiles("/", "./static")
    app.Run()
}
