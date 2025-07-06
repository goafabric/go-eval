package controller

import (
	"person-service/logic"

	"gofr.dev/pkg/gofr"
)

func RoutePerson(app *gofr.App) {
	app.GET("/persons", func(ctx *gofr.Context) (interface{}, error) {
		return logic.Search(ctx, ctx.Param("firstName"), ctx.Param("lastName"))
	})
}
