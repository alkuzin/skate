document.addEventListener('DOMContentLoaded', function() {
    checkAuthState();

    const urlParams = new URLSearchParams(window.location.search);
    const mode = urlParams.get('mode') || 'login';
    
    if (mode === 'register') {
        renderRegisterForm();
    } else {
        renderLoginForm();
    }
});

function checkAuthState() {
    const currentUser = getCurrentUser();
    if (currentUser && window.location.pathname.includes('auth.html')) {
        window.location.href = 'index.html';
    }
}

function renderLoginForm() {
    document.getElementById('authContainer').innerHTML = `
        <div class="auth-header">
            <h1>Вход в аккаунт</h1>
            <p>Введите свои данные</p>
        </div>
        
        <form id="loginForm">
            <div class="form-group">
                <label for="email">Email</label>
                <input type="email" id="email" required placeholder="example@mail.com">
            </div>
            
            <div class="form-group">
                <label for="password">Пароль</label>
                <div class="input-with-icon">
                    <input type="password" id="password" required placeholder="••••••••">
                    <i class="fas fa-eye toggle-password"></i>
                </div>
            </div>
            
            <button type="submit" class="btn-auth">Войти</button>
            
            <div class="auth-switch">
                Нет аккаунта? <a href="auth.html?mode=register">Создать аккаунт</a>
            </div>
        </form>
    `;

    initFormHandlers('loginForm', handleLogin);
    initPasswordToggle();
}

function renderRegisterForm() {
    document.getElementById('authContainer').innerHTML = `
        <div class="auth-header">
            <h1>Регистрация</h1>
            <p>Заполните форму</p>
        </div>
        
        <form id="registerForm">
            <div class="form-group">
                <label for="name">Имя</label>
                <input type="text" id="name" required placeholder="Ваше имя">
            </div>
            
            <div class="form-group">
                <label for="email">Email</label>
                <input type="email" id="email" required placeholder="example@mail.com">
            </div>
            
            <div class="form-group">
                <label for="password">Пароль</label>
                <div class="input-with-icon">
                    <input type="password" id="password" required placeholder="••••••••">
                    <i class="fas fa-eye toggle-password"></i>
                </div>
                <small class="password-hint">Минимум 6 символов</small>
            </div>
            
            <div class="form-group">
                <label for="address">Адрес</label>
                <input type="text" id="address" required placeholder="Ваш адрес">
            </div>
            
            <button type="submit" class="btn-auth">Зарегистрироваться</button>
            
            <div class="auth-switch">
                Уже есть аккаунт? <a href="auth.html">Войти</a>
            </div>
        </form>
    `;

    initFormHandlers('registerForm', handleRegister);
    initPasswordToggle();
}

function handleLogin() {
    const email = document.getElementById('email').value.trim();
    const password = document.getElementById('password').value;
    
    if (!email || !password) {
        showAlert('Заполните все поля', 'error');
        return;
    }
    
    const user = getUserByEmail(email);
    
    if (!user) {
        showAlert('Пользователь не найден', 'error');
        return;
    }
    
    if (user.password !== password) {
        showAlert('Неверный пароль', 'error');
        return;
    }
    
    setCurrentUser(user);
    showAlert('Вход выполнен!', 'success');
    setTimeout(() => window.location.href = 'index.html', 1500);
}

function handleRegister() {
    const user = {
        name: document.getElementById('name').value.trim(),
        email: document.getElementById('email').value.trim(),
        password: document.getElementById('password').value,
        address: document.getElementById('address').value.trim(),
        phone: '',
        orders: [],
        favorites: []
    };
    
    if (!validateUserData(user)) return;
    
    registerUser(user);
    showAlert('Регистрация успешна!', 'success');
    setTimeout(() => window.location.href = 'profile.html', 1500);
}

function validateUserData(user) {
    if (!user.name || !user.email || !user.password || !user.address) {
        showAlert('Заполните все поля', 'error');
        return false;
    }
    
    if (!validateEmail(user.email)) {
        showAlert('Введите корректный email', 'error');
        return false;
    }
    
    if (user.password.length < 6) {
        showAlert('Пароль должен быть не менее 6 символов', 'error');
        return false;
    }
    
    if (getUserByEmail(user.email)) {
        showAlert('Пользователь с таким email уже существует', 'error');
        return false;
    }
    
    return true;
}

function getCurrentUser() {
    return JSON.parse(localStorage.getItem('currentUser'));
}

function setCurrentUser(user) {
    localStorage.setItem('currentUser', JSON.stringify(user));
    updateUserNavigation();
}

function getUserByEmail(email) {
    return JSON.parse(localStorage.getItem('user_' + email));
}

function registerUser(user) {
    localStorage.setItem('user_' + user.email, JSON.stringify(user));
    setCurrentUser(user);
}

function initFormHandlers(formId, handler) {
    const form = document.getElementById(formId);
    if (form) {
        form.addEventListener('submit', function(e) {
            e.preventDefault();
            handler();
        });
    }
}

function initPasswordToggle() {
    document.querySelectorAll('.toggle-password').forEach(icon => {
        icon.addEventListener('click', function() {
            const input = this.previousElementSibling;
            if (input.type === 'password') {
                input.type = 'text';
                this.classList.replace('fa-eye', 'fa-eye-slash');
            } else {
                input.type = 'password';
                this.classList.replace('fa-eye-slash', 'fa-eye');
            }
        });
    });
}

function validateEmail(email) {
    return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email);
}

function showAlert(message, type) {
    const alertDiv = document.createElement('div');
    alertDiv.className = `auth-alert ${type}`;
    alertDiv.textContent = message;
    
    const form = document.querySelector('form');
    if (form) {
        const oldAlert = form.querySelector('.auth-alert');
        if (oldAlert) oldAlert.remove();
        
        form.prepend(alertDiv);
        setTimeout(() => alertDiv.remove(), 5000);
    }
}

// После успешной авторизации/регистрации
function afterAuthSuccess(user) {
    localStorage.setItem('currentUser', JSON.stringify(user));
    
    // Обновление статуса на всех страницах
    if (window.opener) {
        window.opener.updateUserStatus();
    }
    
    window.location.href = 'profile.html';
}