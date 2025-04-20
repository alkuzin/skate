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
        image: '../res/images/set1.jpg'
    },
    'set2': {
        id: 'set2',
        name: 'Сет "Горячий"',
        price: 1600,
        category: 'sets',
        image: '../res/images/set2.jpg'
    },
    'set3': {
        id: 'set3',
        name: 'Сет "Мини гриль"',
        price: 1500,
        category: 'sets',
        image: '../res/images/set3.jpg'
    },
    'set4': {
        id: 'set4',
        name: 'Сет "СамСам"',
        price: 1000,
        category: 'sets',
        image: '../res/images/set4.jpg'
    },
    'set5': {
        id: 'set5',
        name: 'Сет "Мега"',
        price: 2765,
        category: 'sets',
        image: '../res/images/set5.jpg'
    },

    // Холодные роллы
    'roll1': {
        id: 'roll1',
        name: 'Ролл с Авокадо',
        price: 260,
        category: 'cold-rolls',
        image: '../res/images/roll1.jpg'
    },
    'roll2': {
        id: 'roll2',
        name: 'Ролл с Огурцом',
        price: 235,
        category: 'cold-rolls',
        image: '../res/images/roll2.jpg'
    },
    'roll3': {
        id: 'roll3',
        name: 'Ролл с Лососем',
        price: 355,
        category: 'cold-rolls',
        image: '../res/images/roll3.jpg'
    },
    'roll4': {
        id: 'roll4',
        name: 'Ролл с Тунцом',
        price: 323,
        category: 'cold-rolls',
        image: '../res/images/roll4.jpg'
    },
    'roll5': {
        id: 'roll5',
        name: 'Ролл с Креветкой',
        price: 307,
        category: 'cold-rolls',
        image: '../res/images/roll5.jpg'
    },

    // Горячие роллы
    'hot-roll1': {
        id: 'hot-roll1',
        name: 'Ролл "Ацуй"',
        price: 565,
        category: 'hot-rolls',
        image: '../res/images/hot-roll1.jpg'
    },
    'hot-roll2': {
        id: 'hot-roll2',
        name: 'Ролл "Темпура с лососем"',
        price: 535,
        category: 'hot-rolls',
        image: '../res/images/hot-roll2.jpg'
    },
    'hot-roll3': {
        id: 'hot-roll3',
        name: 'Ролл "Кунашир"',
        price: 497,
        category: 'hot-rolls',
        image: '../res/images/hot-roll3.jpg'
    },
    'hot-roll4': {
        id: 'hot-roll4',
        name: 'Ролл "Темпура краш"',
        price: 487,
        category: 'hot-rolls',
        image: '../res/images/hot-roll4.jpg'
    },
    'hot-roll5': {
        id: 'hot-roll5',
        name: 'Ролл "Горячий цезарь"',
        price: 425,
        category: 'hot-rolls',
        image: '../res/images/hot-roll5.jpg'
    },

    // Пицца
    'pizza1': {
        id: 'pizza1',
        name: 'Пицца "Маргарита"',
        price: 740,
        category: 'pizza',
        image: '../res/images/pizza1.jpg'
    },
    'pizza2': {
        id: 'pizza2',
        name: 'Пицца "Пепперони"',
        price: 750,
        category: 'pizza',
        image: '../res/images/pizza2.jpg'
    },
    'pizza3': {
        id: 'pizza3',
        name: 'Пицца "Карбонара"',
        price: 919,
        category: 'pizza',
        image: '../res/images/pizza3.jpg'
    },
    'pizza4': {
        id: 'pizza4',
        name: 'Пицца "Песто"',
        price: 819,
        category: 'pizza',
        image: '../res/images/pizza4.jpg'
    },
    'pizza5': {
        id: 'pizza5',
        name: 'Пицца "Сырный цыпленок"',
        price: 839,
        category: 'pizza',
        image: '../res/images/pizza5.jpg'
    },


    // Поке
    'poke1': {
        id: 'poke1',
        name: 'Поке с тигровыми креветками и битыми огурцами',
        price: 490,
        category: 'poke',
        image: '../res/images/poke1.jpg'
    },
    'poke2': {
        id: 'poke2',
        name: 'Поке с тигровыми креветками и командорским кальмаром',
        price: 540,
        category: 'poke',
        image: '../res/images/poke2.jpg'
    },

    // Горячее
    'hot1': {
        id: 'hot1',
        name: 'Торидон',
        price: 453,
        category: 'hot',
        image: '../res/images/hot1.jpg'
    },
    'hot2': {
        id: 'hot2',
        name: 'Гёдза с креветкой 6 шт.',
        price: 513,
        category: 'hot',
        image: '../res/images/hot2.jpg'
    },
    'hot3': {
        id: 'hot3',
        name: 'Томагояки',
        price: 453,
        category: 'hot',
        image: '../res/images/hot3.jpg'
    },
    
    // Десерт
    'dessert1': {
        id: 'dessert1',
        name: 'Чизкейк Нью-Йорк',
        price: 295,
        category: 'dessert',
        image: '../res/images/dessert1.jpg'
    },
    'dessert2': {
        id: 'dessert2',
        name: 'Карамельный чизкейк',
        price: 350,
        category: 'dessert',
        image: '../res/images/dessert2.jpg'
    },


    // Напитки
    'drink1': {
        id: 'drink1',
        name: 'Сок Rich Вишня',
        price: 200,
        category: 'drinks',
        image: '../res/images/drink1.jpg'
    },
    'drink2': {
        id: 'drink2',
        name: 'Сок Rich Персик',
        price: 200,
        category: 'drinks',
        image: '../res/images/drink2.jpg'
    },
    'drink3': {
        id: 'drink3',
        name: 'Сок Rich Яблоко',
        price: 200,
        category: 'drinks',
        image: '../res/images/drink3.jpg'
    },
    'drink4': {
        id: 'drink4',
        name: 'Сок Rich Апельсин',
        price: 200,
        category: 'drinks',
        image: '../res/images/drink4.jpg'
    },
    
};

// Инициализация корзины
function initCart() {
    const savedCart = localStorage.getItem('cart');
    if (savedCart) {
        cart = JSON.parse(savedCart);
        updateCartUI();
    }
}

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

// Инициализация фиксированной панели категорий
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

// Инициализация скролла категорий
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

// Инициализация при загрузке страницы
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