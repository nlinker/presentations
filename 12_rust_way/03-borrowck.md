### №3. Почему боров чекер такой душный...

- ... даже когда у нас всего 1 поток?
- (это хороший вопрос для собеседования)
- понятно, когда много потоков
```rust
fn main() {
    let mut counter = 0;
    let mut handles = vec![];
    for _ in 0..10 {
        let counter_ref = &mut counter;
        let handle = std::thread::spawn(move || {
            *counter_ref += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", counter);
}
```
... Rust ожидаемо выдаёт ошибку компиляции


#### Способы нахождения ошибок

1. Развить интуицию у программиста выявлять плохие паттерны в коде
2. Снабдить программиста мощным отладчиком
3. Пресекать такие ситуации на уровне системы типов

```rust
fn main() {
    let mut a = 42;

    let b = &a;
    let c = &mut a;

    println!("{a}, {b}, {c}");
}
```
```textmate
   Compiling playground v0.0.1 (/playground)
error[E0502]: cannot borrow `a` as mutable because
              it is also borrowed as immutable
 --> src/main.rs:5:13
```
_Что здесь не так-то?_


#### Немножко подправим
Было
```rust
fn main() {
    let mut a = 42;

    let b = &a;
    let c = &mut a;

    println!("{a}, {b}, {c}");
}
```
Стало
```rust
fn main() {
    let mut a = vec![1, 2, 3];

    let b = &a;
    let c = &mut a;
    let _a0 = &b[0]; // вешаем указатель на элемент 0
    c.clear();       // удаляем все элементы в векторе

    println!("{a:?}, {b:?}, {c:?}");
}
```


#### Картина Репина

![priplyli.jpeg](slides/03/priplyli.jpeg)


#### Способы обхода ограничения

1. Interior Mutablity, доступ через индексы
2. Структурирование кода на стадии
3. Создание специализированной структуры данных

<pre data-id="code-animation"><code class="hljs rust" data-trim data-line-numbers="|9-16|17-23"><script type="text/template">
enum Op {
    Incr(usize),
    Decr(usize),
}

fn main() {
    let mut arr = vec![1, 2, 3];
    let mut changes = Vec::new();
    // eval
    for (i, a) in arr.iter().enumerate() {
        if *a % 2 == 0 {
            changes.push(Op::Incr(i));
        } else {
            changes.push(Op::Decr(i));
        }
    }
    // apply
    for ch in changes {
        match ch {
            Op::Incr(i) => arr[i] += 1,
            Op::Decr(i) => arr[i] -= 1,
        }
    }
    println!("{arr:?}"); // выведет [0, 3, 2]
}
</script></code></pre>


#### Заключительный штрих

- [SICP](https://en.wikipedia.org/wiki/Structure_and_Interpretation_of_Computer_Programs)
- [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)

<img src="slides/03/qr_sicp.png" alt="slides/03/qr_sicp.png" width="300" height="300" />
<img src="slides/03/qr_tmll.png" alt="slides/03/qr_tmll.png" width="300" height="300" />
