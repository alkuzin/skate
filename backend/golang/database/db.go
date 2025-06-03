package database

import (
	"database/sql"
	"fmt"
	_ "github.com/mattn/go-sqlite3"
	"path/filepath"

	"os"
)

var DB *sql.DB

func InitDB() (*sql.DB, error) {
	// Проверяем, существует ли папка db
	if _, err := os.Stat("db"); os.IsNotExist(err) {
		if err := os.Mkdir("db", 0755); err != nil {
			return nil, fmt.Errorf("не удалось создать папку db: %v", err)
		}
	}
	absPath, _ := filepath.Abs("./db/auth.db")
	fmt.Println("Database path:", absPath)
	// Подключаемся к БД (файл создаётся автоматически)
	db, err := sql.Open("sqlite3", "./db/auth.db")
	if err != nil {
		return nil, fmt.Errorf("ошибка подключения к БД: %v", err)
	}
	DB = db

	createTableSQL := `
    CREATE TABLE IF NOT EXISTS users (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL,
        email TEXT NOT NULL UNIQUE,
        number INTEGER NOT NULL,
        password TEXT NOT NULL,
        address TEXT NOT NULL
    );`

	_, err = db.Exec(createTableSQL)
	if err != nil {
		db.Close() // Закрываем соединение, если создание таблицы не удалось
		return nil, fmt.Errorf("ошибка при создании таблицы users: %v", err)
	}
	// Проверяем соединение
	if err := DB.Ping(); err != nil {
		return nil, fmt.Errorf("ошибка ping БД: %v", err)
	}

	return db, nil
}
