package main

import (
	"github.com/gin-gonic/gin"
	"web-service/service"
)

func main() {
	var router = gin.Default()
	service.Route(router)
	router.Run("localhost:50200")
}
