package service

import (
	"github.com/gin-gonic/gin"
	"net/http"
)

func Route(router *gin.Engine) {
	router.GET("/callees/sayMyName", sayMyName)
	router.GET("/callees/sayMyOtherName", sayMyOtherName)
}

func sayMyName(c *gin.Context) {
	var name = c.Request.URL.Query().Get("name")
	c.IndentedJSON(http.StatusOK,
		Callee{Message: name})
}

func sayMyOtherName(c *gin.Context) {
	var name = c.Request.URL.Query().Get("name")
	c.IndentedJSON(http.StatusOK,
		Callee{Message: name})
}
