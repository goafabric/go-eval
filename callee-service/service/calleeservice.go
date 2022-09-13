package service

import (
	"callee-service/service/dto"
	"github.com/gin-gonic/gin"
	"net/http"
)

func sayMyName(c *gin.Context) {
	c.IndentedJSON(http.StatusOK,
		dto.Callee{Message: "Heisenberg"})
}

func sayMyOtherName(c *gin.Context) {
	c.IndentedJSON(http.StatusOK,
		dto.Callee{Message: "Slim Shady"})
}

func Route(router *gin.Engine) {
	router.GET("/sayMyName", sayMyName)
	router.GET("/sayMyOtherName", sayMyOtherName)
}
