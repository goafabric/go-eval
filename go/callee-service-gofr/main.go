package main

import "callee-service/controller"
import "gofr.dev/pkg/gofr"

type Callee struct {
    ID      string `json:"id"`
	Message string `json:"message"`
}

type Health struct {
	Status string `json:"status"`
}

func main() {
    app := gofr.New()

    controller.Route(app)

    app.AddStaticFiles("/", "./static")
    app.Run()
}
