package main

import (
    "net/http"
    "github.com/gin-gonic/gin"
    "web-service/dto"
)


var albums = []dto.Album{
    {ID: "1", Title: "Blue Train", Artist: "John Coltrane", Price: 56.99},
    {ID: "2", Title: "Jeru", Artist: "Gerry Mulligan", Price: 17.99},
    {ID: "3", Title: "Sarah Vaughan and Clifford Brown", Artist: "Sarah Vaughan", Price: 39.99},
}

func getAlbums(c *gin.Context) {
    c.IndentedJSON(http.StatusOK, albums)
}

func main() {
    var router = gin.Default()
    router.GET("/albums", getAlbums)

    router.Run("localhost:50200")
}