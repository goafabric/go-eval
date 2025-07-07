package main

import (
	"github.com/gin-contrib/static"
	"github.com/gin-gonic/gin"
	"net/http"
)

func main() {
	var router = gin.Default()
	Route(router)

	router.Use(static.Serve("/", static.LocalFile("./static", false)))
	router.Run("0.0.0.0:50900")
}

type Callee struct {
	Message string `json:"message"`
}

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

