package repository

import (
	"backend/internal/models"
	"database/sql"
	"fmt"
	"log"
)

type UserRepository struct {
	db *sql.DB
}

func NewUserRepository(db *sql.DB) *UserRepository {
	return &UserRepository{db: db}
}

type User = models.User

func (r *UserRepository) CreateUser(user *models.User) (int64, error) {
	// Убедитесь, что пароль передаётся
	log.Printf("Создание пользователя: Email=%s, Password=%s", user.Email, user.Password)

	// Проверяем, есть ли уже пользователь с таким email
	var exists int
	err := r.db.QueryRow("SELECT COUNT(1) FROM users WHERE email = ?", user.Email).Scan(&exists)
	if err != nil {
		return 0, err
	}
	if exists != 0 {
		return 0, fmt.Errorf("poshel naxui")
	}

	// Вставляем нового пользователя
	res, err := r.db.Exec(
		`INSERT INTO users (name, email, password, address) VALUES (?, ?, ?, ?)`,
		user.Name, user.Email, user.Password, user.Address,
	)
	if err != nil {
		return 0, err
	}

	// Получаем ID нового пользователя
	id, err := res.LastInsertId()
	if err != nil {
		return 0, err
	}

	return id, nil
}

func (r *UserRepository) GetUserByEmail(email string) (*User, error) {
	user := &User{}
	err := r.db.QueryRow(
		`SELECT id, name, email, password, address FROM users WHERE email = ?`,
		email,
	).Scan(&user.ID, &user.Name, &user.Email, &user.Password, &user.Address)
	return user, err
}
