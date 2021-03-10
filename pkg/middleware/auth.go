package middleware

import (
	"fmt"
	"net/http"

	"github.com/cjdenio/shorty/pkg/config"
	"github.com/gin-gonic/gin"
)

func AuthRequired(c *gin.Context) {
	if c.GetHeader("Authorization") != fmt.Sprintf("Bearer %s", config.Config.Token) {
		c.AbortWithStatus(http.StatusUnauthorized)
		return
	}

	c.Next()
}
