package db

import (
	"fmt"
	"log"

	"github.com/cjdenio/shorty/pkg/config"
	"github.com/go-redis/redis"
)

var DB *redis.Client

func Init() {
	opt, err := redis.ParseURL(config.Config.DbUrl)
	if err != nil {
		log.Fatal(err)
	}

	DB = redis.NewClient(opt)
}

func AddLink(name, url string) error {
	result := DB.Set(fmt.Sprintf("link:%s", name), url, 0)
	if err := result.Err(); err != nil {
		return err
	}

	return nil
}

func GetLink(name string) (string, error) {
	result := DB.Get(fmt.Sprintf("link:%s", name))
	if err := result.Err(); err != nil {
		return "", err
	}

	return result.Val(), nil
}
