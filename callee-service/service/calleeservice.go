package service

import (
	"github.com/gin-gonic/gin"
	"net/http"
)

func sayMyName(c *gin.Context) {
	c.IndentedJSON(http.StatusOK,
		Callee{Message: "Heisenberg"})
}

func sayMyOtherName(c *gin.Context) {
	c.IndentedJSON(http.StatusOK,
		Callee{Message: "Slim Shady"})
}

func Route(router *gin.Engine) {
	router.GET("/callees/sayMyName", sayMyName)
	router.GET("/callees/sayMyOtherName", sayMyOtherName)
}
