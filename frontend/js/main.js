let cart = {
    items: [],
    total: 0,
    count: 0
};

// Товары
const productsDB = {
    'dish-day-1': {
        id: 'dish-day-1',
        name: 'Ролл "Калифорния с крабом"',
        price: 350,
        category: 'cold-rolls',
        image: '../res/images/dish-of-day.jpg'
        
    },
    'set1': {
        id: 'set1',
        name: 'Сет "Домашний"',
        price: 1400,
        category: 'sets',
        image: '../res/images/set1.jpg',
        description: 'Сет на 3 персоны. Запечёные роллы с креветкой, с куриным филе, с крабом, с лососем. Подаётся с имбирём, васаби и соевым соусом 820г.'
    },
    'set2': {
        id: 'set2',
        name: 'Сет "Горячий"',
        price: 1600,
        category: 'sets',
        image: '../res/images/set2.jpg',
        description: 'Сет на 4 персоны. Темпура с крабом и курицей, джусто, кунашир, темпура с курицей и беконом с соусом спайси. Подаётся с соусом спайси 1000г.'
    },
    'set3': {
        id: 'set3',
        name: 'Сет "Мини гриль"',
        price: 1500,
        category: 'sets',
        image: '../res/images/set3.jpg',
        description: 'Сет на 4 персоны. Запечёные роллы с креветкой, с лососем, йоджи кани, брюс ли. Подаётся с имбирём, васаби и соевым соусом 930г.'
    },
    'set4': {
        id: 'set4',
        name: 'Сет "СамСам"',
        price: 1000,
        category: 'sets',
        image: '../res/images/set4.jpg',
        description: 'Сет на 2 персоны. Спайси суши с лососем, с креветкой, маки с лососем, калифорния с крабом. Подаётся с имбирём, васаби и соевым соусом 400г.'
    },
    'set5': {
        id: 'set5',
        name: 'Сет "Мега"',
        price: 2765,
        category: 'sets',
        image: '../res/images/set5.jpg',
        description: 'Сет на 6 персон. Йоджи кани, йоджи сяке, брюс ли, инь ян, филадельфия с огурцом, калифорния с лососем, маки с креветкой, с лососем. Подаётся с имбирём, васаби и соевым соусом 1650г.'
    },

    // Холодные роллы
    'roll1': {
        id: 'roll1',
        name: 'Ролл с Авокадо',
        price: 260,
        category: 'cold-rolls',
        image: '../res/images/roll1.jpg',
        description: '110г 8 кусочков: Классический ролл с авокадо. Подается с соевым соусом, васаби и имбирём.'
    },
    'roll2': {
        id: 'roll2',
        name: 'Ролл с Огурцом',
        price: 235,
        category: 'cold-rolls',
        image: '../res/images/roll2.jpg',
        description: '110г 8 кусочков: Классический ролл с хрустящим огурцом. Подается с соевым соусом, васаби и имбирём.'
    },
    'roll3': {
        id: 'roll3',
        name: 'Ролл с Лососем',
        price: 355,
        category: 'cold-rolls',
        image: '../res/images/roll3.jpg',
        description: '110г 8 кусочков: Классический ролл с охлаждённым лососем. Подается с соевым соусом, васаби и имбирём.'
    },
    'roll4': {
        id: 'roll4',
        name: 'Ролл с Тунцом',
        price: 323,
        category: 'cold-rolls',
        image: '../res/images/roll4.jpg',
        description: '110г 8 кусочков: Классический ролл с тунцом. Подается с соевым соусом, васаби и имбирём.'
    },
    'roll5': {
        id: 'roll5',
        name: 'Ролл с Креветкой',
        price: 307,
        category: 'cold-rolls',
        image: '../res/images/roll5.jpg',
        description: '110г 8 кусочков: Классический ролл с тигровыми креветками. Подается с соевым соусом, васаби и имбирём.'
    },

    // Горячие роллы
    'hot-roll1': {
        id: 'hot-roll1',
        name: 'Ролл "Ацуй"',
        price: 565,
        category: 'hot-rolls',
        image: '../res/images/hot-roll1.jpg',
        description: 'Темпура ролл с лососем, угрём, копчёным беконом и сыром моцарелла 200г. Подаётся с пикантным соусом спайс. Соевый соус, васаби или имбирь можно приобрести отдельно.'
    },
    'hot-roll2': {
        id: 'hot-roll2',
        name: 'Ролл "Темпура с лососем"',
        price: 535,
        category: 'hot-rolls',
        image: '../res/images/hot-roll2.jpg',
        description: 'Темпура ролл с лососем, сливочным сыром, огурцом, зелёным луком и икрой масаго под ореховым соусом и унаги 230г. Подаётся с пикантным соусом спайс. Соевый соус, васаби или имбирь можно приобрести отдельно.'
    },
    'hot-roll3': {
        id: 'hot-roll3',
        name: 'Ролл "Кунашир"',
        price: 497,
        category: 'hot-rolls',
        image: '../res/images/hot-roll3.jpg',
        description: 'Темпура ролл с лососем, тунцом, тигровыми креветками, кальмаром и икрой масаго 200г. Подаётся с пикантным соусом спайс. Соевый соус, васаби или имбирь можно приобрести отдельно.'
    },
    'hot-roll4': {
        id: 'hot-roll4',
        name: 'Ролл "Темпура краш"',
        price: 487,
        category: 'hot-rolls',
        image: '../res/images/hot-roll4.jpg',
        description: 'Темпура ролл с овощами и сливочным сыром под шапкой из крабового тартара, лука фри и зелёного лука с сырным соусом и унаги 300г. Подаётся с пикантным соусом спайс. Соевый соус, васаби или имбирь можно приобрести отдельно.'
    },
    'hot-roll5': {
        id: 'hot-roll5',
        name: 'Ролл "Горячий цезарь"',
        price: 425,
        category: 'hot-rolls',
        image: '../res/images/hot-roll5.jpg',
        description: 'Темпура ролл с куриным филе, томатом, листьями салата и соусом цезарь 200г. Подаётся с пикантным соусом спайс. Соевый соус, васаби или имбирь можно приобрести отдельно.'
    },

    // Пицца
    'pizza1': {
        id: 'pizza1',
        name: 'Пицца "Маргарита"',
        price: 740,
        category: 'pizza',
        image: '../res/images/pizza1.jpg',
        description: 'Увеличенная порция моцареллы, томаты, итальянские травы, фирменный томатный соус. 30 см, 590 г.'
    },
    'pizza2': {
        id: 'pizza2',
        name: 'Пицца "Пепперони"',
        price: 750,
        category: 'pizza',
        image: '../res/images/pizza2.jpg',
        description: 'Пикантная пепперони, увеличенная порция моцареллы, фирменный томатный соус. 30 см, 550 г.'
    },
    'pizza3': {
        id: 'pizza3',
        name: 'Пицца "Карбонара"',
        price: 919,
        category: 'pizza',
        image: '../res/images/pizza3.jpg',
        description: 'Бекон, сыры чеддер и пармезан, моцарелла, томаты, красный лук, чеснок, фирменный соус альфредо, итальянские травы. 30 см, 590 г.'
    },
    'pizza4': {
        id: 'pizza4',
        name: 'Пицца "Песто"',
        price: 819,
        category: 'pizza',
        image: '../res/images/pizza4.jpg',
        description: 'Цыпленок, соус песто, кубики брынзы, томаты, моцарелла, фирменный соус альфредо. 30 см, 610 г.'
    },
    'pizza5': {
        id: 'pizza5',
        name: 'Пицца "Сырный цыпленок"',
        price: 839,
        category: 'pizza',
        image: '../res/images/pizza5.jpg',
        description: 'Цыпленок, моцарелла, сыры чеддер и пармезан, сырный соус, томаты, фирменный соус альфредо, чеснок. 30 см, 620 г.'
    },


    // Поке
    'poke1': {
        id: 'poke1',
        name: 'Поке с тигровыми креветками и битыми огурцами',
        price: 490,
        category: 'poke',
        image: '../res/images/poke1.jpg',
        description: 'Поке с натуральным соусом, сочными тигровыми креветками и полезным салатом чука, пряными битыми огурцами, душистым корнем имбиря и свежими помидорами черри.'
    },
    'poke2': {
        id: 'poke2',
        name: 'Поке с тигровыми креветками и командорским кальмаром',
        price: 540,
        category: 'poke',
        image: '../res/images/poke2.jpg',
        description: 'Поке с натуральным соусом, тигровыми креветками и командорским кальмаром, пряными битыми огурцами, душистым корнем имбиря, свежими помидорами черри.'
    },

    // Горячее
    'hot1': {
        id: 'hot1',
        name: 'Торидон',
        price: 453,
        category: 'hot',
        image: '../res/images/hot1.jpg',
        description: '295г Маринованный цыплёнок в хрустящей панировке, подаётся на рисе с салатом айсберг, эдамамэ, нори и соусом терияки, соусом чиммай и соусом сладкий чили.'
    },
    'hot2': {
        id: 'hot2',
        name: 'Гёдза с креветкой 6 шт.',
        price: 513,
        category: 'hot',
        image: '../res/images/hot2.jpg',
        description: '100/30г Японские пельмени с тигровыми креветками, капустой, кунжутным маслом и свежей зеленью, под фирменным соусом никкей с долькой лимона.'
    },
    'hot3': {
        id: 'hot3',
        name: 'Томагояки',
        price: 453,
        category: 'hot',
        image: '../res/images/hot3.jpg',
        description: '290г Блюдо на каждый день, в котором поджаренная куриная грудка с рисом, болтуньей, салатом айсберг и хрустящим луком фри, заправлены соусом унаги и ореховым соусом, а зелёный лук, томаты черри, кунжут и фирменный соус гармонично дополняют вкус блюда.'
    },
    
    // Десерт
    'dessert1': {
        id: 'dessert1',
        name: 'Чизкейк Нью-Йорк',
        price: 295,
        category: 'dessert',
        image: '../res/images/dessert1.jpg',
        description: 'Чизкейк нью-йорк с топпингом на выбор 125г.'
    },
    'dessert2': {
        id: 'dessert2',
        name: 'Карамельный чизкейк',
        price: 350,
        category: 'dessert',
        image: '../res/images/dessert2.jpg',
        description: 'Сливочный десерт с очень карамельной начинкой, шоколадным печеньем и орехами, 100 г.'
    },


    // Напитки
    'drink1': {
        id: 'drink1',
        name: 'Сок Rich Вишня',
        price: 200,
        category: 'drinks',
        image: '../res/images/drink1.jpg',
        description: 'Сок Рич 200мл на выбор.'
    },
    'drink2': {
        id: 'drink2',
        name: 'Сок Rich Персик',
        price: 200,
        category: 'drinks',
        image: '../res/images/drink2.jpg',
        description: 'Сок Рич 200мл на выбор.'
    },
    'drink3': {
        id: 'drink3',
        name: 'Сок Rich Яблоко',
        price: 200,
        category: 'drinks',
        image: '../res/images/drink3.jpg',
        description: 'Сок Рич 200мл на выбор.'
    },
    'drink4': {
        id: 'drink4',
        name: 'Сок Rich Апельсин',
        price: 200,
        category: 'drinks',
        image: '../res/images/drink4.jpg',
        description: 'Сок Рич 200мл на выбор.'
    },
    
};

function initCart() {
    const savedCart = localStorage.getItem('cart');
    if (savedCart) {
        cart = JSON.parse(savedCart);
        updateCartUI();
    }
}

// Проверка авторизации при загрузке страницы
document.addEventListener('DOMContentLoaded', function () {
    updateUserNavigation();
    const currentUser = JSON.parse(localStorage.getItem('currentUser'));
    
    if (currentUser) {
        const userProfileElements = document.querySelectorAll('.user-profile');
        userProfileElements.forEach(el => {
            el.innerHTML = `
                <i class="far fa-user"></i>
                <span>${currentUser.name}</span>
            `;
            el.onclick = function() {
                window.location.href = 'profile.html';
            };
        });
    }
    
    initCart();
    initFavorites();
    initFixedCategories();
    initScrollButtons();
    window.addEventListener('scroll', handleScroll);
});

// Добавление товара в корзину
function addToCart(productId, event) {
    if (event) {
        event.preventDefault();
        event.stopPropagation();
    }
    
    const product = productsDB[productId];
    if (!product) return;

    const existingItem = cart.items.find(item => item.id === productId);
    
    if (existingItem) {
        existingItem.quantity += 1;
    } else {
        cart.items.push({
            ...product,
            quantity: 1
        });
    }

    updateCart();
    showNotification(`"${product.name}" добавлен в корзину`);
}

// Обновление корзины
function updateCart() {
    cart.total = cart.items.reduce((sum, item) => sum + (item.price * item.quantity), 0);
    cart.count = cart.items.reduce((count, item) => count + item.quantity, 0);
    
    localStorage.setItem('cart', JSON.stringify(cart));
    updateCartUI();
}

function updateCartUI() {
    const cartTotalElement = document.querySelector('.cart-total');
    const cartCountElement = document.querySelector('.cart-count');
    
    if (cartTotalElement) {
        cartTotalElement.textContent = `${cart.total} руб`;
    }
    
    if (cartCountElement) {
        cartCountElement.textContent = cart.count;
        cartCountElement.style.display = cart.count > 0 ? 'flex' : 'none';
    }
}

// Уведомление
function showNotification(message) {
    const notification = document.createElement('div');
    notification.className = 'notification show';
    notification.innerHTML = `
        <span>${message}</span>
        <a href="cart.html">Перейти в корзину</a>
    `;
    document.body.appendChild(notification);
    
    setTimeout(() => {
        notification.classList.remove('show');
        setTimeout(() => notification.remove(), 300);
    }, 3000);
}

// Фильтрация по категориям
function filterByCategory(categoryId) {
    const section = document.getElementById(categoryId);
    if (section) {
        window.scrollTo({
            top: section.offsetTop - 120,
            behavior: 'smooth'
        });
    }
}

// Фиксированная панель категорий
function initFixedCategories() {
    const originalCategories = document.querySelector('.categories-section .categories-container');
    const fixedCategories = document.querySelector('.fixed-categories .categories-container');
    
    if (originalCategories && fixedCategories) {
        fixedCategories.innerHTML = originalCategories.innerHTML;
        
        // Добавление обработчики клика для фиксированной панели
        document.querySelectorAll('.fixed-categories .category').forEach(category => {
            category.addEventListener('click', function() {
                const categoryType = this.getAttribute('onclick').match(/'([^']+)'/)[1];
                filterByCategory(categoryType);
            });
        });
    }
}

// Обработчик прокрутки для фиксированной панели
function handleScroll() {
    const categoriesSection = document.querySelector('.categories-section');
    const fixedCategories = document.querySelector('.fixed-categories');
    const scrollPosition = window.scrollY;
    
    if (categoriesSection && fixedCategories) {
        const categoriesTop = categoriesSection.offsetTop;
        const scrollThreshold = categoriesTop - 150;
        
        if (scrollPosition > scrollThreshold) {
            fixedCategories.classList.add('visible');
            fixedCategories.classList.remove('hidden');
        } else {
            fixedCategories.classList.remove('visible');
            fixedCategories.classList.add('hidden');
        }
    }
}

function initScrollButtons() {
    document.querySelectorAll('.categories-container').forEach(container => {
        const leftBtn = container.parentElement.querySelector('.left-scroll');
        const rightBtn = container.parentElement.querySelector('.right-scroll');
        
        if (leftBtn) {
            leftBtn.addEventListener('click', () => {
                container.scrollBy({ left: -200, behavior: 'smooth' });
            });
        }
        
        if (rightBtn) {
            rightBtn.addEventListener('click', () => {
                container.scrollBy({ left: 200, behavior: 'smooth' });
            });
        }
    });
}

document.addEventListener('DOMContentLoaded', () => {
    initCart();
    initFixedCategories();
    initScrollButtons();
    window.addEventListener('scroll', handleScroll);
    
    // Добавление счетчика товаров в корзину
    const cartBtn = document.querySelector('.cart-btn');
    if (cartBtn && !document.querySelector('.cart-count')) {
        const countElement = document.createElement('div');
        countElement.className = 'cart-count';
        countElement.style.display = 'none';
        cartBtn.appendChild(countElement);
        updateCartUI();
    }
});


// Открытие модального окна товара
function openProductModal(productId) {
    const product = productsDB[productId];
    if (!product) return;

    const modal = document.getElementById('productModal');
    const modalBody = document.getElementById('productModalBody');
    
    const isFavorite = favorites.items.includes(productId);
    
    modalBody.innerHTML = `
        <div class="product-modal-image">
            <img src="${product.image}" alt="${product.name}" style="max-height: 300px; object-fit: contain;">
        </div>
        <div class="product-modal-info">
            <h2 class="product-modal-title">${product.name}</h2>
            <p class="product-modal-description">${product.description || 'Описание отсутствует'}</p>
            <div class="product-modal-price">${product.price} руб</div>
            <div class="product-modal-actions">
                <button class="btn" onclick="addToCart('${productId}', event); event.stopPropagation()">
                    Добавить в корзину
                </button>
                <button class="favorite-btn-modal" onclick="toggleFavorite('${productId}', event); event.stopPropagation()">
                    <i class="${isFavorite ? 'fas' : 'far'} fa-heart"></i>
                </button>
            </div>
        </div>
    `;
    
    modal.style.display = 'block';
    document.body.style.overflow = 'hidden';
}

// Закрытие модального окна
function closeProductModal() {
    const modal = document.getElementById('productModal');
    modal.style.display = 'none';
    document.body.style.overflow = 'auto';
}

// Закрытие при клике вне модального окна
window.onclick = function(event) {
    const modal = document.getElementById('productModal');
    if (event.target === modal) {
        closeProductModal();
    }
}

// Избранное
let favorites = {
    items: []
};

function initFavorites() {
    const savedFavorites = localStorage.getItem('favorites');
    if (savedFavorites) {
        favorites = JSON.parse(savedFavorites);
    } else {
        favorites = { items: [] };
    }
    updateFavoritesUI();
}

// Функция добавления/удаления товара из избранного
function toggleFavorite(productId, event) {
    if (event) {
        event.preventDefault();
        event.stopPropagation();
    }
    
    const index = favorites.items.findIndex(item => item === productId);
    
    if (index === -1) {
        favorites.items.push(productId);
    } else {
        favorites.items.splice(index, 1);
    }
    
    localStorage.setItem('favorites', JSON.stringify(favorites));
    updateFavoritesUI();
    
    // Обновление иконки на главной странице
    const favoriteBtns = document.querySelectorAll(`.favorite-btn[data-product-id="${productId}"]`);
    favoriteBtns.forEach(btn => {
        btn.innerHTML = favorites.items.includes(productId) ? 
            '<i class="fas fa-heart"></i>' : '<i class="far fa-heart"></i>';
    });
    
    // Обновление иконки в модальном окне
    const modalFavoriteBtn = document.querySelector('.favorite-btn-modal');
    if (modalFavoriteBtn && modalFavoriteBtn.dataset.productId === productId) {
        modalFavoriteBtn.innerHTML = favorites.items.includes(productId) ? 
            '<i class="fas fa-heart"></i>' : '<i class="far fa-heart"></i>';
    }
    
    // Уведомление
    const product = productsDB[productId];
    if (product) {
        showNotification(favorites.items.includes(productId) ? 
            `"${product.name}" добавлен в избранное` : 
            `"${product.name}" удален из избранного`);
    }
}

function updateFavoritesUI() {
    // Обновление счетчика в шапке
    const favoritesCount = document.querySelector('.favorites-count');
    if (favoritesCount) {
        favoritesCount.textContent = favorites.items.length;
        favoritesCount.style.display = favorites.items.length > 0 ? 'flex' : 'none';
    }
    
    // Обновление иконки на карточках товаров
    document.querySelectorAll('.favorite-btn').forEach(btn => {
        const productId = btn.dataset.productId;
        btn.innerHTML = favorites.items.includes(productId) ? 
            '<i class="fas fa-heart"></i>' : '<i class="far fa-heart"></i>';
    });
}

document.addEventListener('DOMContentLoaded', () => {
    initCart();
    initFavorites();
});

document.querySelector('.user-profile').addEventListener('click', function() {
    const user = getCurrentUser();
    if (user) {
        window.location.href = 'profile.html';
    } else {
        window.location.href = 'auth.html';
    }
});

function loadCatalog() {
    fetch('/api/products')
        .then(response => response.json())
        .then(products => {
            const catalog = document.getElementById('catalog');
            products.forEach(product => {
                catalog.innerHTML += `
                    <div class="product-card" data-id="${product.id}">
                        <img src="${product.image}" alt="${product.name}">
                        <h3>${product.name}</h3>
                        <p>${product.price} руб.</p>
                        <button class="add-to-cart" data-id="${product.id}">
                            В корзину
                        </button>
                    </div>
                `;
            });
        });
}

// Хранение данных пользователя
let currentUser = null;

document.addEventListener('DOMContentLoaded', () => {
    checkAuth();
    loadCart();
    loadCatalog();
});

// Проверка авторизации
function checkAuth() {
    const user = localStorage.getItem('currentUser');
    if (user) {
        currentUser = JSON.parse(user);
        updateAuthUI();
    }
}

function updateAuthUI() {
    if (currentUser) {
        document.querySelectorAll('.auth-only').forEach(el => {
            el.style.display = 'block';
        });
        document.querySelectorAll('.guest-only').forEach(el => {
            el.style.display = 'none';
        });
    }
}
