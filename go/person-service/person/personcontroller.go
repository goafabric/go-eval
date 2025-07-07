package person

import (
	"person-service/extensions"
	"gofr.dev/pkg/gofr"
)

func RoutePerson(app *gofr.App) {
	app.GET("/persons", func(ctx *gofr.Context) (interface{}, error) {
        println("UserName from UserContext:", ctx.Value("userContext").(extensions.UserContext).UserName)
		return Search(ctx, ctx.Param("firstName"), ctx.Param("lastName"))
	})
}
