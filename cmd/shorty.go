package main

import (
	"net/http"

	"github.com/cjdenio/shorty/pkg/config"
	"github.com/cjdenio/shorty/pkg/db"
	"github.com/gin-gonic/gin"
)

func main() {
	db.Init()

	r := gin.Default()

	r.GET("/", func(c *gin.Context) {
		if config.Config.RootUrl != "" {
			c.Redirect(http.StatusTemporaryRedirect, config.Config.RootUrl)
		} else {
			c.String(http.StatusNotFound, "404 page not found")
		}
	})

	r.GET("/:name", func(c *gin.Context) {
		url, err := db.GetLink(c.Param("name"))
		if err != nil {
			c.String(http.StatusNotFound, "404 page not found")
			return
		}
		c.Redirect(http.StatusTemporaryRedirect, url)
	})

	r.Run(":3000")
}
