document.addEventListener('DOMContentLoaded', function() {
    // Проверка авторизации
    const currentUser = JSON.parse(localStorage.getItem('currentUser'));
    if (!currentUser) {
        window.location.href = 'login.html';
        return;
    }

    // Заполнение формы данными пользователя
    const form = document.getElementById('profile-form');
    form.elements.name.value = currentUser.name || '';
    form.elements.email.value = currentUser.email || '';
    form.elements.phone.value = currentUser.phone || '';
    form.elements.address.value = currentUser.address || '';

    form.addEventListener('submit', function(e) {
        e.preventDefault();
        
        // Обновление данных пользователя
        currentUser.name = form.elements.name.value;
        currentUser.phone = form.elements.phone.value;
        currentUser.address = form.elements.address.value;
        
        // Сохранение
        localStorage.setItem('currentUser', JSON.stringify(currentUser));
        localStorage.setItem('user_' + currentUser.email, JSON.stringify(currentUser));
        
        alert('Данные успешно сохранены!');
    });
});

function logout() {
    localStorage.removeItem('currentUser');
    window.location.href = 'login.html';
}