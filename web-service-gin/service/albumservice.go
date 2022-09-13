package service

import (
	"github.com/gin-gonic/gin"
	"net/http"
	"web-service/service/dto"
)

var albums = []dto.Album{
	{ID: "1", Title: "Blue Train", Artist: "John Coltrane", Price: 56.99},
	{ID: "2", Title: "Jeru", Artist: "Gerry Mulligan", Price: 17.99},
	{ID: "3", Title: "Sarah Vaughan and Clifford Brown", Artist: "Sarah Vaughan", Price: 39.99},
}

func getAlbums(c *gin.Context) {
	c.IndentedJSON(http.StatusOK,
		dto.Album{ID: "1", Title: "Blue Train", Artist: "Mickey Mouse"})
}

func Route(router *gin.Engine) {
	router.GET("/albums", getAlbums)
}
