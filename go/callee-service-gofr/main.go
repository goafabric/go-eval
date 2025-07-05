package main

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

    app.GET("/actuator/health", func(ctx *gofr.Context) (interface{}, error) {
		return Health{Status: "OK"}, nil
	})

    app.GET("/callees/sayMyName", func(ctx *gofr.Context) (interface{}, error) {
		name := ctx.Param("name")
		return sayMyName(name), nil
	})

	app.GET("/callees/sayMyOtherName/{name}", func(ctx *gofr.Context) (interface{}, error) {
		name := ctx.PathParam("name")
		return sayMyOtherName(name), nil
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

    app.AddStaticFiles("/", "./static")
    app.Run()
}


func sayMyName(name string) Callee {
	return Callee{ID: "0", Message: "Your name is: " + name}
}

func sayMyOtherName(name string) Callee {
	return Callee{ID: "0", Message: "Your other name is: " + name}
}

func save(callee Callee) Callee {
	return Callee{ID: "0", Message: "Storing your message: " + callee.Message}
}