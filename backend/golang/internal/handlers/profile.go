package handlers

import (
	"database/sql"
	"encoding/json"
	"github.com/dgrijalva/jwt-go"
	"net/http"
	"strconv"
)

func GetProfile(db *sql.DB) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		// Извлекаем userID из JWT
		claims, _ := r.Context().Value("claims").(*jwt.StandardClaims)
		userID, _ := strconv.Atoi(claims.Subject)

		var user struct {
			Name    string `json:"name"`
			Email   string `json:"email"`
			Address string `json:"address"`
		}

		err := db.QueryRow(
			"SELECT name, email, address FROM users WHERE id = ?",
			userID,
		).Scan(&user.Name, &user.Email)

		if err != nil {
			w.WriteHeader(http.StatusNotFound)
			return
		}

		json.NewEncoder(w).Encode(user)
	}
}
