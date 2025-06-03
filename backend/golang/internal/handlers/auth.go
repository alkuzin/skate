package handlers

import (
	"backend/internal/models"
	"backend/internal/repository"
	"backend/pkg/jwt"
	"database/sql"
	"encoding/json"
	"net/http"
	"strings"
)

func writeJSONError(w http.ResponseWriter, status int, message string) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	json.NewEncoder(w).Encode(map[string]string{"error": message})
}

func Register(db *sql.DB) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {

		req1 := &models.User{}
		if err := json.NewDecoder(r.Body).Decode(&req1); err != nil {
			writeJSONError(w, http.StatusBadRequest, "Invalid JSON")
			return
		}

		userRepo := repository.NewUserRepository(db)
		userID, err := userRepo.CreateUser(req1)
		if err != nil {
			writeJSONError(w, http.StatusConflict, "Email already exists")
			return
		}

		token, err := jwt.GenerateToken(userID, req1.Email)
		if err != nil {
			writeJSONError(w, http.StatusInternalServerError, "Failed to generate token")
			return
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]interface{}{
			"token":   token,
			"user_id": userID,
		})
	}
}

func Login(db *sql.DB) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		var req struct {
			Email    string `json:"email"`
			Password string `json:"password"`
		}

		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			writeJSONError(w, http.StatusBadRequest, "Invalid JSON")
			return
		}

		req.Email = strings.ToLower(strings.TrimSpace(req.Email))
		req.Password = strings.TrimSpace(req.Password)

		user := &models.User{}
		userRepo := repository.NewUserRepository(db)
		user, err := userRepo.GetUserByEmail(req.Email)
		if err != nil || user.Password != req.Password {
			writeJSONError(w, http.StatusUnauthorized, "Email or password is incorrect")
			return
		}

		token, err := jwt.GenerateToken(int64(user.ID), user.Email)
		if err != nil {
			writeJSONError(w, http.StatusInternalServerError, "Failed to generate token")
			return
		}

		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(map[string]interface{}{
			"token":   token,
			"user_id": user.ID,
			"name":    user.Name,
		})
	}
}
