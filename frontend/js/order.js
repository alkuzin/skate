document.getElementById("order-form").addEventListener("submit", function (e) {
    e.preventDefault();
    const address = document.getElementById("address").value;
    const payment = document.getElementById("payment").value;

    alert(`Заказ оформлен! Адрес: ${address}, Способ оплаты: ${payment}`);
    location.href = "index.html";
});