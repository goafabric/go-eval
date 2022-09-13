package main

import (
	"callee-service/service"
	"github.com/gin-gonic/gin"
)

func main() {
	var router = gin.Default()
	service.Route(router)

	router.Static("/welcome", "./static")
	router.Run("localhost:50200")
}
