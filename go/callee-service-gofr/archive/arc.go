package main

import (
	"net/http"

	"github.com/toddproject/gofr/pkg/gofr"
)

type Callee struct {
	ID      string `json:"id"`
	Message string `json:"message"`
}

type CalleeLogic struct{}

func (c *CalleeLogic) SayMyName(name string) Callee {
	return Callee{ID: "0", Message: "Your name is: " + name}
}

func (c *CalleeLogic) SayMyOtherName(name string) Callee {
	return Callee{ID: "0", Message: "Your other name is: " + name}
}

func (c *CalleeLogic) Save(callee Callee) Callee {
	return Callee{ID: "0", Message: "Storing your message: " + callee.Message}
}

func main() {
	app := gofr.New()
	logic := &CalleeLogic{}

	app.GET("/callees/sayMyName", func(ctx *gofr.Context) (interface{}, error) {
		name := ctx.Param("name")
		if name == "" {
			name = ctx.QueryParam("name") // fallback to query param
		}
		return logic.SayMyName(name), nil
	})

	app.GET("/callees/sayMyOtherName/{name}", func(ctx *gofr.Context) (interface{}, error) {
		name := ctx.PathParam("name")
		return logic.SayMyOtherName(name), nil
	})

	app.POST("/callees/save", func(ctx *gofr.Context) (interface{}, error) {
		var callee Callee
		if err := ctx.Bind(&callee); err != nil {
			return nil, gofr.NewError(http.StatusBadRequest, "invalid request body")
		}
		return logic.Save(callee), nil
	})

	app.Start()
}
