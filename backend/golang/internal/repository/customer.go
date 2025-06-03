package repository

import (
	"backend/internal/models"
	"database/sql"
)

type CustomerRepository struct {
	db *sql.DB
}

func NewCustomerRepository(db *sql.DB) *CustomerRepository {
	return &CustomerRepository{db: db}
}

// Создание профиля покупателя
func (r *CustomerRepository) CreateCustomer(customer *models.Customer) error {
	_, err := r.db.Exec(
		`INSERT INTO customers (user_id, name, address, phone) 
		 VALUES (?, ?, ?, ?)`,
		customer.UserID, customer.Name, customer.Address, customer.Phone,
	)
	return err
}

func (r *CustomerRepository) GetCustomerByUserID(userID int) (*models.Customer, error) {
	c := &models.Customer{}
	err := r.db.QueryRow(
		`SELECT user_id, name, address, phone FROM customers WHERE user_id = ?`,
		userID,
	).Scan(&c.UserID, &c.Name, &c.Address, &c.Phone)
	return c, err
}
