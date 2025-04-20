function renderCartPage() {
    const cartItemsContainer = document.querySelector('.cart-items');
    const itemsCountElement = document.querySelector('.items-count');
    const totalPriceElement = document.querySelector('.total-price');
    
    cartItemsContainer.innerHTML = '';
    
    if (cart.items.length === 0) {
        cartItemsContainer.innerHTML = '<p class="empty-cart">Ваша корзина пуста</p>';
        itemsCountElement.textContent = '0';
        totalPriceElement.textContent = '0 руб';
        return;
    }
    
    // Добавление товаров
    cart.items.forEach(item => {
        const cartItem = document.createElement('div');
        cartItem.className = 'cart-item';
        cartItem.innerHTML = `
            <img src="${item.image}" alt="${item.name}">
            <div class="item-info">
                <h3>${item.name}</h3>
                <p class="item-category">${getCategoryName(item.category)}</p>
            </div>
            <div class="item-controls">
                <button class="quantity-btn minus" onclick="changeQuantity('${item.id}', -1)">-</button>
                <span class="quantity">${item.quantity}</span>
                <button class="quantity-btn plus" onclick="changeQuantity('${item.id}', 1)">+</button>
                <span class="item-price">${item.price * item.quantity} руб</span>
                <button class="remove-btn" onclick="removeFromCart('${item.id}')">
                    <i class="fas fa-trash"></i>
                </button>
            </div>
        `;
        cartItemsContainer.appendChild(cartItem);
    });
    
    // Обновление итоговой суммы
    itemsCountElement.textContent = cart.count;
    totalPriceElement.textContent = `${cart.total} руб`;
}

// Получение названия категории
function getCategoryName(categoryId) {
    const categories = {
        'sets': 'Сеты',
        'cold-rolls': 'Холодные роллы',
        'hot-rolls': 'Горячие роллы',
        'pizza': 'Пицца',
        'poke': 'Поке',
        'hot': 'Горячее',
        'dessert': 'Десерт',
        'drinks': 'Напитки'
    };
    return categories[categoryId] || '';
}

// Изменение количества товара
function changeQuantity(productId, change) {
    const item = cart.items.find(item => item.id === productId);
    if (!item) return;
    
    item.quantity += change;
    
    // Если количество стало 0 или меньше, товар удаляется
    if (item.quantity <= 0) {
        removeFromCart(productId);
        return;
    }
    
    // Обновление общей суммы и количества
    cart.total = cart.items.reduce((sum, item) => sum + (item.price * item.quantity), 0);
    cart.count = cart.items.reduce((count, item) => count + item.quantity, 0);

    // Сохранение
    localStorage.setItem('cart', JSON.stringify(cart));
    updateCartUI();
    renderCartPage();
}

// Удаление товара из корзины
function removeFromCart(productId) {
    cart.items = cart.items.filter(item => item.id !== productId);
    
    // Обновление общей суммы и количества
    cart.total = cart.items.reduce((sum, item) => sum + (item.price * item.quantity), 0);
    cart.count = cart.items.reduce((count, item) => count + item.quantity, 0);
    
    // Сохранение
    localStorage.setItem('cart', JSON.stringify(cart));
    updateCartUI();
    renderCartPage();
}

// Инициализация страницы корзины
document.addEventListener('DOMContentLoaded', () => {
    initCart();
    renderCartPage();
});