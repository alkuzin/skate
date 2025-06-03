package main

import (
	"backend/database"
	"backend/internal/config"
	"backend/internal/handlers"
	"github.com/gorilla/mux"
	"log"
	"net/http"
)

func main() {
	// Загрузка конфигурации
	cfg := config.Load()

	// Инициализация БД (теперь возвращает *sql.DB)
	db, err := database.InitDB()
	if err != nil {
		log.Fatalf("Ошибка инициализации БД: %v", err)
	}
	defer db.Close()

	// Создание роутера
	r := mux.NewRouter()
	r.Handle("/api/profile", handlers.MiddlewareCORS(
		handlers.GetProfile(db),
	)).Methods("GET")
	// Auth routes
	r.Use(handlers.MiddlewareCORS)
	r.HandleFunc("/api/auth/register", handlers.Register(db)).Methods("POST", "OPTIONS")
	r.HandleFunc("/api/auth/login", handlers.Login(db)).Methods("POST", "OPTIONS")

	// Настройка сервера
	/*
		srv := &http.Server{
			Handler:      r,
			Addr:         ":" + cfg.ServerPort,
			WriteTimeout: 15 * time.Second,
			ReadTimeout:  15 * time.Second,
		}*/

	log.Printf("Сервер запущен на порту %s", cfg.ServerPort)
	http.ListenAndServe(":8080", r)

}
