document.addEventListener('DOMContentLoaded', function() {
    // Проверка авторизации
    const currentUser = JSON.parse(localStorage.getItem('currentUser'));
    if (!currentUser) {
        alert('Для оформления заказа необходимо войти в систему');
        window.location.href = 'login.html?redirect=checkout.html';
        return;
    }

    // Заполнение формы данными пользователя
    const form = document.getElementById('deliveryForm');
    form.elements.name.value = currentUser.name || '';
    form.elements.phone.value = currentUser.phone || '';
    form.elements.address.value = currentUser.address || '';

    // Загрузка товаров из корзины
    const cart = JSON.parse(localStorage.getItem('cart')) || { items: [], total: 0, count: 0 };
    const orderItemsContainer = document.getElementById('orderItems');
    const totalPriceElement = document.getElementById('totalPrice');
    
    // Отображение товаров
    function renderOrderItems() {
        orderItemsContainer.innerHTML = '';
        let total = 0;
        
        cart.items.forEach(item => {
            const itemElement = document.createElement('div');
            itemElement.className = 'order-item';
            itemElement.innerHTML = `
                <span>${item.name} × ${item.quantity}</span>
                <span>${item.price * item.quantity} руб.</span>
            `;
            orderItemsContainer.appendChild(itemElement);
            total += item.price * item.quantity;
        });
        
        totalPriceElement.textContent = total + ' руб.';
    }
    
    // Обработка подтверждения заказа
    document.getElementById('confirmOrder').addEventListener('click', function() {
        const name = document.getElementById('name').value;
        const phone = document.getElementById('phone').value;
        const address = document.getElementById('address').value;
        
        if (!name || !phone || !address) {
            alert('Пожалуйста, заполните все поля');
            return;
        }
        
        // Обновление данных пользователя
        currentUser.name = name;
        currentUser.phone = phone;
        currentUser.address = address;
        localStorage.setItem('currentUser', JSON.stringify(currentUser));
        localStorage.setItem('user_' + currentUser.email, JSON.stringify(currentUser));
        
        const order = {
            id: Date.now(),
            userId: currentUser.email,
            date: new Date().toLocaleString(),
            items: cart.items,
            customer: { name, phone, address },
            total: cart.total,
            status: 'В обработке'
        };
        
        const orders = JSON.parse(localStorage.getItem('orders')) || [];
        orders.push(order);
        localStorage.setItem('orders', JSON.stringify(orders));
        
        localStorage.removeItem('cart');
        
        window.location.href = 'order-success.html';
    });
    
    renderOrderItems();
});