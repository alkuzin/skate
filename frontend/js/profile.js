document.getElementById("profile-form").addEventListener("submit", function (e) {
    e.preventDefault();
    const name = document.getElementById("name").value;
    const phone = document.getElementById("phone").value;

    alert(`Данные сохранены! Имя: ${name}, Телефон: ${phone}`);
});