package callee

import (
	"gofr.dev/pkg/gofr"
)

func RouteCallee(app *gofr.App) {
	app.GET("/callees/sayMyName", func(ctx *gofr.Context) (interface{}, error) {
		name := ctx.Param("name")
		return SayMyName(name), nil
	})

	app.GET("/callees/sayMyOtherName/{name}", func(ctx *gofr.Context) (interface{}, error) {
		name := ctx.PathParam("name")
		return SayMyOtherName(name), nil
	})

	/*
			app.POST("/callees/save", func(ctx *gofr.Context) (interface{}, error) {
				var callee Callee
		        if err := ctx.Bind(&callee); err != nil {
		            return nil, gofr.NewError(http.StatusBadRequest, "invalid request body")
		        }
				return Callee{ID: "0", Message: "Storing your message: " + callee.Message}
			})
	*/

}
