package person

import (
	"gofr.dev/pkg/gofr"
)

func RoutePerson(app *gofr.App) {
	app.GET("/persons", func(ctx *gofr.Context) (interface{}, error) {
		return Search(ctx, ctx.Param("firstName"), ctx.Param("lastName"))
	})
}
