package models

import "time"

type Profile struct {
	UserID      uint      `gorm:"primaryKey"`
	FirstName   string    `gorm:"size:100"`
	LastName    string    `gorm:"size:100"`
	Phone       string    `gorm:"size:20"`
	Address     string    `gorm:"type:text"`
	DateOfBirth time.Time `gorm:"type:date"`
	AvatarURL   string    `gorm:"size:255"`
	CreatedAt   time.Time
	UpdatedAt   time.Time
}
