package main

import calleeController "callee-service/controller"
import "gofr.dev/pkg/gofr"

func main() {
    app := gofr.New()

    calleeController.Route(app)

    app.AddStaticFiles("/", "./static")
    app.Run()
}
