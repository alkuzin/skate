document.addEventListener('DOMContentLoaded', function() {
    // Проверка авторизации
    const currentUser = JSON.parse(localStorage.getItem('currentUser'));
    if (!currentUser) {
        window.location.href = 'login.html';
        return;
    }

    // Загрузка заказов пользователя
    const orders = JSON.parse(localStorage.getItem('orders')) || [];
    const userOrders = orders.filter(order => order.userId === currentUser.email);
    
    const ordersList = document.getElementById('ordersList');
    
    if (userOrders.length === 0) {
        ordersList.innerHTML = '<p>У вас пока нет заказов</p>';
        return;
    }
    
    // Сортиртировка заказов по дате (сначала новые)
    userOrders.sort((a, b) => new Date(b.date) - new Date(a.date));
    
    // Отображение заказов
    userOrders.forEach(order => {
        const orderElement = document.createElement('div');
        orderElement.className = 'order-card';
        
        // Форматирование даты
        const orderDate = new Date(order.date);
        const formattedDate = orderDate.toLocaleString('ru-RU', {
            day: '2-digit',
            month: '2-digit',
            year: 'numeric',
            hour: '2-digit',
            minute: '2-digit'
        });
        
        // Общая сумма
        const total = order.items.reduce((sum, item) => sum + (item.price * item.quantity), 0);
        
        orderElement.innerHTML = `
            <div class="order-header">
                <span>Заказ #${order.id}</span>
                <span>${formattedDate}</span>
                <span class="status ${order.status.toLowerCase()}">${order.status}</span>
            </div>
            <div class="order-items">
                ${order.items.map(item => `
                    <div class="order-item">
                        <span>${item.name} × ${item.quantity}</span>
                        <span>${item.price * item.quantity} руб.</span>
                    </div>
                `).join('')}
            </div>
            <div class="order-total">
                Итого: ${total} руб.
            </div>
        `;
        
        ordersList.appendChild(orderElement);
    });
});