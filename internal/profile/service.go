package profile

import (
	"context"
	"time"
	"user-service/internal/models"
	"user-service/internal/repository"
)

type Service struct {
	profileRepo *repository.ProfileRepository
	userRepo    *repository.UserRepository
}

func NewProfileService(profileRepo *repository.ProfileRepository, userRepo *repository.UserRepository) *Service {
	return &Service{
		profileRepo: profileRepo,
		userRepo:    userRepo,
	}
}

type UpdateProfileRequest struct {
	FirstName   string
	LastName    string
	Phone       string
	Address     string
	DateOfBirth string // ISO format "2006-01-02"
	AvatarURL   string
}

func (s *Service) UpdateProfile(ctx context.Context, userID uint, req UpdateProfileRequest) (*models.Profile, error) {
	profile := &models.Profile{
		UserID:    userID,
		FirstName: req.FirstName,
		LastName:  req.LastName,
		Phone:     req.Phone,
		Address:   req.Address,
		AvatarURL: req.AvatarURL,
	}

	if req.DateOfBirth != "" {
		dob, err := time.Parse("2006-01-02", req.DateOfBirth)
		if err != nil {
			return nil, err
		}
		profile.DateOfBirth = dob
	}

	if err := s.profileRepo.CreateOrUpdate(ctx, profile); err != nil {
		return nil, err
	}

	return profile, nil
}

func (s *Service) GetProfile(ctx context.Context, userID uint) (*models.Profile, error) {
	return s.profileRepo.GetByUserID(ctx, userID)
}
