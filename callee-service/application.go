package main

import (
	"callee-service/service"

	"github.com/gin-contrib/static"
	"github.com/gin-gonic/gin"
)

func main() {
	var router = gin.Default()
	service.Route(router)

	router.Use(static.Serve("/", static.LocalFile("./static", false)))
	router.Run("localhost:50200")
}
