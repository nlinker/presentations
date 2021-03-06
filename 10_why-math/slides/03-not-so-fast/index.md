# Не так быстро!

- Основной упор при решении задач это борьба со *сложностью*
- Сложность растёт экспоненциально с ростом размера проекта
- Если представить, что вы строитель
    - первый кирпич кладёте за минуту
    - на тысячный требуется минимум месяц <br /> 
      // _Е. Тюменцев_

----

## Перевёрнутая пирамида

- В программировании пирамиды перевёрнутые!

![Pyramid](slides/03-not-so-fast/pyramid.jpg) <!-- .element: style="max-height: 400px;" class="plain" -->

----

## Пирамида отражает как софт пишется

- Приложения, пользовательские программы
- Библиотеки для создания программ
- Библиотеки для создания библиотек
- Системные библиотеки и ОС
- Железо (тоже может быть программируемым!)

На софт более низкого уровня накладываются более жёсткие требования
и он призван быть строительным блоком для того, что выше. 

----

## Анализ и синтез

- **Модульность и композиция** - ключевые концепции для борьбы со сложностью
- Но композицию легко сломать!
    - Если во время футбола каждый встанет, ему будет видно лучше.
        Следовательно, если все встанут, то ...
    - Клетки невидимы, я состою из клеток, следовательно...
    - Если я игнорирую проблемы на 1 час, то они исчезают на час для меня. 
        Если я игнорирую на 1 день, следовательно... ☺

----

## Композицию легко сломать

```clang
bool flag = true;
int f(int n) { 
    int r = flag? 2 * n: n;
    flag != flag;
    return r;
}

printf("f(1) + f(2) = %d\n", f(1) + f(2)); // напечатает 4
printf("f(2) + f(1) = %d\n", f(2) + f(1)); // напечатает 5
```

Всё ещё хуже, когда такое в МП/async <!-- .element class="fragment" -->

----

## Композицию трудно гарантировать

Как только вы задумаетесь о композиции частей, вы сталкиваетесь с математикой!

- Чистые функции, (_ФП, лямбда исчисление, алгебра_)
- Типизация, (_алгебра, ТК, конструктивная математика, логика_)

ФП + Типизация = ♥

Например, ФП для обработки потоков событий, необходимы точные типы (FRP).

----

## Соответствие Карри-Ховарда

![Curry-Howard](slides/03-not-so-fast/curry-howard.png) <!-- .element height="500" -->
Построить значение типа == доказать теорему! 

----

## Wake up, Neo, Math has you... 

- необходимость искать модульность и композицию нужны, чтобы ...
- справиться со сложностью, которая порождена ...
- необходимостью в поиске всё более сложных решений ...
- мы столкнулись с математикой ещё даже не окунувшись в предметную область

![Matrix has you](slides/03-not-so-fast/matrix.jpg) <!-- .element height="300" -->

----

## Предметные области

- Машинное обучение
- Криптовалюты и ZKP
- AR/VR, игры, 3d-моделирование
- Физические, химические, социологические и другие модели
- Алгоритмы обработки данных, поиска и оптимизации
- Инструменты для создания программ
