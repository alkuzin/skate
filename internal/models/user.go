package models

import "time"

type Role string

const (
	RoleCustomer Role = "CUSTOMER"
	RoleCourier  Role = "COURIER"
	RoleManager  Role = "MANAGER"
)

type User struct {
	ID           uint   `gorm:"primaryKey"`
	Email        string `gorm:"uniqueIndex;not null"`
	PasswordHash string `gorm:"not null"`
	CreatedAt    time.Time
	UpdatedAt    time.Time
	Role         Role `gorm:"type:role_enum;not null"`

	// Profile associations
	Customer *Customer
	Employee *Employee
}

type Customer struct {
	ID        uint `gorm:"primaryKey"`
	UserID    uint `gorm:"uniqueIndex"`
	FirstName string
	LastName  string
	Phone     string
	Address   string
}

type Employee struct {
	ID       uint `gorm:"primaryKey"`
	UserID   uint `gorm:"uniqueIndex"`
	Position string
	BranchID uint
}
