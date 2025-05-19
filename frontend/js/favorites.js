function renderFavoritesPage() {
    const favoritesContainer = document.querySelector('.favorites-items');
    const emptyMessage = document.querySelector('.empty-favorites');
    
    favoritesContainer.innerHTML = '';
    
    if (!favorites.items || favorites.items.length === 0) {
        emptyMessage.style.display = 'flex';
        document.querySelector('.clear-favorites-btn').style.display = 'none';
        return;
    }
    
    emptyMessage.style.display = 'none';
    document.querySelector('.clear-favorites-btn').style.display = 'block';
    
    favorites.items.forEach(productId => {
        const product = productsDB[productId];
        if (!product) return;
        
        const favoriteItem = document.createElement('div');
        favoriteItem.className = 'product-card favorite-item';
        
        favoriteItem.innerHTML = `
            <div class="favorite-item-img-container">
                <img src="${product.image}" alt="${product.name}" class="favorite-item-img">
            </div>
            <div class="product-info">
                <h3>${product.name}</h3>
                <p class="description">${product.description?.substring(0, 60)}...</p>
                <div class="price-add">
                    <span class="price">${product.price} руб</span>
                    <button class="add-btn" onclick="addToCart('${productId}', event); event.stopPropagation()">+</button>
                </div>
            </div>
            <button class="favorite-remove" onclick="removeFromFavorites('${productId}', event)">
                <i class="fas fa-times"></i>
            </button>
        `;
        
        favoriteItem.addEventListener('click', function(e) {
            if (!e.target.closest('.favorite-remove') && !e.target.closest('.add-btn')) {
                openProductModal(productId);
            }
        });
        
        favoritesContainer.appendChild(favoriteItem);
    });
}

// Функция для поиска товара по ID
function findProductById(productId) {
    // Поиск в основной базе товаров
    if (productsDB[productId]) {
        return productsDB[productId];
    }
    
    return null;
}

// Функция удаления из избранного
function removeFromFavorites(productId, event) {
    if (event) event.stopPropagation();
    
    const index = favorites.items.findIndex(item => item === productId);
    if (index !== -1) {
        favorites.items.splice(index, 1);
        localStorage.setItem('favorites', JSON.stringify(favorites));
        updateFavoritesUI();
        
        renderFavoritesPage();
    }
}

document.addEventListener('DOMContentLoaded', () => {
    initFavorites();
    renderFavoritesPage();
});

function clearFavorites() {
    if (confirm('Вы действительно хотите очистить все избранные товары?')) {
        favorites.items = [];
        localStorage.setItem('favorites', JSON.stringify(favorites));
        updateFavoritesUI();
        renderFavoritesPage();
    }
}