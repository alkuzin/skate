function handleRegister(e) {
    e.preventDefault();
    
    const name = document.getElementById('name').value.trim();
    const email = document.getElementById('email').value.trim();
    const password = document.getElementById('password').value;
    const address = document.getElementById('address').value.trim();
    
    // Валидация
    if (!validateEmail(email)) {
        alert('Введите корректный email');
        return;
    }
    
    if (password.length < 6) {
        alert('Пароль должен содержать минимум 6 символов');
        return;
    }
    
    // Проверка существующего пользователя
    if (localStorage.getItem(`user_${email}`)) {
        alert('Пользователь с таким email уже зарегистрирован');
        return;
    }
    
    // Создание пользователя
    const user = {
        name,
        email,
        password,
        address,
        phone: '',
        orders: []
    };
    
    localStorage.setItem(`user_${email}`, JSON.stringify(user));
    localStorage.setItem('currentUser', JSON.stringify(user));

    window.location.href = 'profile.html';
}

function validateEmail(email) {
    const re = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return re.test(email);
}