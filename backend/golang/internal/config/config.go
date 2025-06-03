package config

import "os"

type Config struct {
	DBPath     string
	JWTKey     string
	ServerPort string
}

func Load() *Config {
	return &Config{
		DBPath:     getEnv("DB_PATH", "./db/auth.db"),
		JWTKey:     getEnv("JWT_KEY", "your_secret_key"),
		ServerPort: getEnv("SERVER_PORT", "8080"),
	}
}

func getEnv(key, defaultValue string) string {
	if value, exists := os.LookupEnv(key); exists {
		return value
	}
	return defaultValue
}
