package main

import (
	"callee-service/service"
	"github.com/gin-gonic/gin"
)

func main() {
	var router = gin.Default()
	service.Route(router)
	router.Run("localhost:50200")
}
