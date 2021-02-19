package main

import (
	"fmt"
	"net/http"

	"github.com/cjdenio/shorty/pkg/config"
	"github.com/gin-gonic/gin"
)

func main() {
	fmt.Println("hi")
	r := gin.Default()

	r.GET("/", func(c *gin.Context) {
		if config.Config.RootUrl != "" {
			c.Redirect(http.StatusTemporaryRedirect, config.Config.RootUrl)
		} else {
			c.String(http.StatusNotFound, "404 Not Found")
		}
	})

	r.GET("/:name", func(c *gin.Context) {
		c.Redirect(http.StatusTemporaryRedirect, fmt.Sprintf("https://github.com/cjdenio/%s", c.Param("name")))
	})

	r.Run(":3000")
}
