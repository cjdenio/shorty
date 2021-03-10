package config

import "os"

// ConfigType represents Shorty's config
type ConfigType struct {
	// RootUrl, if non-empty, represents the link that the root URL will redirect to
	RootUrl string `json:"root_url"`
	DbUrl   string `json:"db_url"`
	Token   string `json:"token"`
}

var Config ConfigType

func init() {
	Config.DbUrl = os.Getenv("DB_URL")
	Config.RootUrl = os.Getenv("ROOT_URL")
	Config.Token = os.Getenv("TOKEN")
}
