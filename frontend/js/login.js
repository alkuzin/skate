document.addEventListener('DOMContentLoaded', () => {
    const authContainer = document.getElementById('authContainer');
    const urlParams = new URLSearchParams(window.location.search);
    const mode = urlParams.get('mode') || 'login';
    
    if (mode === 'register') {
        renderRegisterForm();
    } else {
        renderLoginForm();
    }
});

function renderLoginForm() {
    authContainer.innerHTML = `
        <h1>Вход в аккаунт</h1>
        <form id="loginForm">
            <div class="form-group">
                <label for="email">Email</label>
                <input type="email" id="email" required>
            </div>
            <div class="form-group">
                <label for="password">Пароль</label>
                <input type="password" id="password" required>
            </div>
            <div class="auth-actions">
                <button type="submit" class="btn">Войти</button>
                <div class="auth-switch">
                    Нет аккаунта? <a href="auth.html?mode=register">Зарегистрироваться</a>
                </div>
            </div>
        </form>
    `;
    
    document.getElementById('loginForm').addEventListener('submit', handleLogin);
}

function renderRegisterForm() {
    authContainer.innerHTML = `
        <h1>Регистрация</h1>
        <form id="registerForm">
            <div class="form-group">
                <label for="name">Имя</label>
                <input type="text" id="name" required>
            </div>
            <div class="form-group">
                <label for="email">Email</label>
                <input type="email" id="email" required>
            </div>
            <div class="form-group">
                <label for="password">Пароль</label>
                <input type="password" id="password" required>
            </div>
            <div class="form-group">
                <label for="address">Адрес доставки</label>
                <input type="text" id="address" required>
            </div>
            <div class="auth-actions">
                <button type="submit" class="btn">Зарегистрироваться</button>
                <div class="auth-switch">
                    Уже есть аккаунт? <a href="auth.html?mode=login">Войти</a>
                </div>
            </div>
        </form>
    `;
    
    document.getElementById('registerForm').addEventListener('submit', handleRegister);
}