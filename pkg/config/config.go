package config

import (
	_ "embed"
	"encoding/json"
)

// ConfigType represents Shorty's config
type ConfigType struct {
	// RootUrl, if non-empty, represents the link that the root URL will redirect to
	RootUrl string `json:"root_url"`
}

var Config ConfigType

//go:embed config.json
var rawConfig []byte

func init() {
	json.Unmarshal(rawConfig, &Config)
}
