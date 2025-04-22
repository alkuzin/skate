package repository

import (
	"context"
	"gorm.io/gorm"
	"user-service/internal/models"
)

type ProfileRepository struct {
	db *gorm.DB
}

func NewProfileRepository(db *gorm.DB) *ProfileRepository {
	return &ProfileRepository{db: db}
}

func (r *ProfileRepository) CreateOrUpdate(ctx context.Context, profile *models.Profile) error {
	return r.db.WithContext(ctx).Save(profile).Error
}

func (r *ProfileRepository) GetByUserID(ctx context.Context, userID uint) (*models.Profile, error) {
	var profile models.Profile
	err := r.db.WithContext(ctx).Where("user_id = ?", userID).First(&profile).Error
	return &profile, err
}
