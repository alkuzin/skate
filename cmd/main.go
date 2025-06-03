package main

import (
	"log"
	"net/http"
	"user-service/internal/auth"
	"user-service/internal/profile"
	"user-service/internal/repository"
	"user-service/pkg/config"
	"user-service/pkg/database"

	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	"gorm.io/gorm"
)

func main() {
	cfg := config.Load()

	// Initialize database
	db, err := database.NewPostgresConnection(cfg.DBURL)
	if err != nil {
		log.Fatalf("Failed to connect to database: %v", err)
	}

	// Run migrations
	if err := database.Migrate(db); err != nil {
		log.Fatalf("Migration failed: %v", err)
	}

	// Initialize repositories
	userRepo := repository.NewUserRepository(db)

	// Initialize services
	authService := auth.NewAuthService(userRepo, cfg.JWTSecret)

	// Добавьте в main.go после создания репозиториев
	profileRepo := repository.NewProfileRepository(db)
	profileService := profile.NewProfileService(profileRepo, userRepo)
	profileHandler := profile.NewProfileHandler(profileService)

	// Setup router
	r := chi.NewRouter()
	r.Use(middleware.Logger)

	// Auth routes
	r.Mount("/auth", auth.NewAuthHandler(authService).Routes())

	// Profile routes
	r.Mount("/profile", profile.NewProfileHandler(profileService).Routes())
	r.Mount("/api/profile", profileHandler.Routes())

	log.Printf("Starting server on :%s", cfg.Port)
	if err := http.ListenAndServe(":"+cfg.Port, r); err != nil {
		log.Fatalf("Server failed: %v", err)
	}
}
