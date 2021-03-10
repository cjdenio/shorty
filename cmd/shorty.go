package main

import (
	"log"
	"net/http"

	"github.com/cjdenio/shorty/pkg/config"
	"github.com/cjdenio/shorty/pkg/db"
	"github.com/cjdenio/shorty/pkg/middleware"
	"github.com/gin-gonic/gin"
	"github.com/go-redis/redis"
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
		if err != nil && err != redis.Nil {
			log.Println(err)
			c.String(http.StatusInternalServerError, "500 internal server error")
		} else if err != nil {
			c.String(http.StatusNotFound, "404 page not found")
			return
		}
		c.Redirect(http.StatusTemporaryRedirect, url)
	})

	admin := r.Group("/admin", middleware.AuthRequired)
	{
		admin.POST("/item", func(c *gin.Context) {
			var data struct {
				Name string `json:"name"`
				URL  string `json:"url"`
			}

			c.BindJSON(&data)

			if err := db.AddLink(data.Name, data.URL); err != nil {
				c.AbortWithError(500, err)
			}

			c.String(http.StatusOK, "Success!")
		})
	}

	r.Run(":3000")
}
