package profile

import (
	"github.com/go-chi/chi/v5"
	"github.com/go-chi/render"
	"net/http"
	"user-service/internal/auth"
)

type Handler struct {
	service *Service
}

func NewProfileHandler(service *Service) *Handler {
	return &Handler{service: service}
}

func (h *Handler) Routes() chi.Router {
	r := chi.NewRouter()
	r.Use(auth.AuthMiddleware) // Защищаем роуты авторизацией

	r.Get("/", h.getProfile)
	r.Put("/", h.updateProfile)

	return r
}

func (h *Handler) getProfile(w http.ResponseWriter, r *http.Request) {
	userID := r.Context().Value("userID").(uint)

	profile, err := h.service.GetProfile(r.Context(), userID)
	if err != nil {
		render.Status(r, http.StatusNotFound)
		render.JSON(w, r, map[string]string{"error": "profile not found"})
		return
	}

	render.JSON(w, r, profile)
}

func (h *Handler) updateProfile(w http.ResponseWriter, r *http.Request) {
	userID := r.Context().Value("userID").(uint)

	var req UpdateProfileRequest
	if err := render.DecodeJSON(r.Body, &req); err != nil {
		render.Status(r, http.StatusBadRequest)
		render.JSON(w, r, map[string]string{"error": "invalid request"})
		return
	}

	profile, err := h.service.UpdateProfile(r.Context(), userID, req)
	if err != nil {
		render.Status(r, http.StatusInternalServerError)
		render.JSON(w, r, map[string]string{"error": err.Error()})
		return
	}

	render.JSON(w, r, profile)
}
