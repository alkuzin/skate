package jwt

import (
	"errors"
	"github.com/dgrijalva/jwt-go"
	"time"
)

var secretKey = []byte("skate")

func GenerateToken(userID int64, email string) (string, error) {
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, jwt.MapClaims{
		"user_id": userID,
		"email":   email,
		"exp":     time.Now().Add(24 * time.Hour).Unix(),
	})

	return token.SignedString(secretKey)
}

var (
	ErrInvalidToken = errors.New("invalid token")
)

// ParseToken проверяет и распаковывает JWT токен
func ParseToken(tokenString string) (int64, string, error) {
	token, err := jwt.Parse(tokenString, func(token *jwt.Token) (interface{}, error) {
		// Проверяем метод подписи
		if _, ok := token.Method.(*jwt.SigningMethodHMAC); !ok {
			return nil, ErrInvalidToken
		}
		return secretKey, nil
	})

	if err != nil {
		return 0, "", err
	}

	if !token.Valid {
		return 0, "", ErrInvalidToken
	}

	// Извлекаем claims
	claims, ok := token.Claims.(jwt.MapClaims)
	if !ok {
		return 0, "", ErrInvalidToken
	}

	// Получаем userID (преобразуем к float64, а затем к int64, так как jwt.MapClaims хранит числа как float64)
	userID, ok := claims["user_id"].(float64)
	if !ok {
		return 0, "", ErrInvalidToken
	}

	// Получаем email
	email, ok := claims["email"].(string)
	if !ok {
		return 0, "", ErrInvalidToken
	}

	return int64(userID), email, nil
}
