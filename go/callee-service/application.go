package main

import (
	"callee-service/controller"

	"github.com/gin-contrib/static"
	"github.com/gin-gonic/gin"
)

func main() {
	var router = gin.Default()
	controller.Route(router)

	router.Use(static.Serve("/", static.LocalFile("./static", false)))
	router.Run("0.0.0.0:50900")
}
