package callee

import (
	"callee-service/extensions"
	"gofr.dev/pkg/gofr"
)

func RouteCallee(app *gofr.App) {
	app.GET("/callees/sayMyName", func(ctx *gofr.Context) (interface{}, error) {
		println("UserName from UserContext:", ctx.Value("userContext").(extensions.UserContext).UserName)
		return SayMyName(ctx.Param("name")), nil
	})

	app.GET("/callees/sayMyOtherName/{name}", func(ctx *gofr.Context) (interface{}, error) {
		println("UserName from UserContext:", ctx.Value("userContext").(extensions.UserContext).UserName)
		return SayMyOtherName(ctx.PathParam("name")), nil
	})

}
