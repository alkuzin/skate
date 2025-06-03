package models

type Customer struct {
	UserID  int    `json:"user_id" db:"user_id"`
	Name    string `json:"name" db:"name"`
	Address string `json:"address" db:"address"`
	Phone   string `json:"phone" db:"phone"`
}
