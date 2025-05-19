document.addEventListener('DOMContentLoaded', function() {
    // Проверка авторизации
    const currentUser = JSON.parse(localStorage.getItem('currentUser'));
    if (!currentUser) {
        window.location.href = 'auth.html';
        return;
    }

    // Заполнение формы данными
    const form = document.getElementById('profile-form');
    if (form) {
        form.elements.name.value = currentUser.name || '';
        form.elements.email.value = currentUser.email || '';
        form.elements.phone.value = currentUser.phone || '';
        form.elements.address.value = currentUser.address || '';

        // Обработка сохранения
        form.addEventListener('submit', function(e) {
            e.preventDefault();
            
            // Обновление данных пользователя
            currentUser.name = form.elements.name.value.trim();
            currentUser.phone = form.elements.phone.value.trim();
            currentUser.address = form.elements.address.value.trim();
            
            // Обновляем в localStorage
            updateUserData(currentUser);
            showAlert('Данные успешно сохранены!', 'success');
        });
    }

    // Обработка выхода
    const logoutBtn = document.getElementById('logoutBtn');
    if (logoutBtn) {
        logoutBtn.addEventListener('click', function() {
            if (confirm('Вы действительно хотите выйти?')) {
                logoutUser();
            }
        });
    }
});

// Функция выхода пользователя
function logoutUser() {
    localStorage.removeItem('currentUser');
    window.location.href = 'auth.html'; 
}

// Функция обновления данных пользователя
function updateUserData(user) {
    const users = JSON.parse(localStorage.getItem('users')) || [];
    const index = users.findIndex(u => u.email === user.email);
    if (index !== -1) {
        users[index] = user;
        localStorage.setItem('users', JSON.stringify(users));
    }
    
    // Обновление текущего пользователя
    localStorage.setItem('currentUser', JSON.stringify(user));
}

// Показ уведомлений
function showAlert(message, type) {
    const alertDiv = document.createElement('div');
    alertDiv.className = `profile-alert ${type}`;
    alertDiv.textContent = message;
    
    const form = document.getElementById('profile-form');
    if (form) {
        // Удаление предыдущих уведомлений
        const oldAlert = form.querySelector('.profile-alert');
        if (oldAlert) oldAlert.remove();
        
        form.prepend(alertDiv);
        setTimeout(() => alertDiv.remove(), 3000);
    }
}