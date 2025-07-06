package controller

import "gofr.dev/pkg/gofr"
import "callee-service/dto"

func RouteHealth(app *gofr.App) {
    app.GET("/actuator/health", func(ctx *gofr.Context) (interface{}, error) {
		return dto.Health{Status: "OK"}, nil
	})
}