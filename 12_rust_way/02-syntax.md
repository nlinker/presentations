### Почему у Раста такой шумный синтаксис?

<style>
.twocolumn_03 {
   display: grid;
   grid-template-columns: 4fr 1fr;
   grid-gap: 10px;
   text-align: left;
}
</style>

Обычно приводятся фрагменты
```rust
pub fn read<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
  fn inner(path: &Path) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(bytes)
  }
  inner(path.as_ref())
}
```
(этот код взят из [статьи тов. Matklad](https://matklad.github.io/2023/01/26/rusts-ugly-syntax.html))
<img src="slides/03/qr_matklad.png" height=150 alt="qr_matklad.png"></img>


#### Низкоуровневый язык!

1. В функции `read` параметр `path` обобщённый
2. Внутренняя функция `inner` для ускорения rustc 
3. Сложный возвращаемый тип для описания типа ошибки
4. `file.read_to_end(&mut bytes)`

```rust
pub fn read<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
  fn inner(path: &Path) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(bytes)
  }
  inner(path.as_ref())
}
```


#### Без этих деталей код упрощается!

```rust
pub fn read(path: Path) -> Bytes {
  let file = File::open(path);
  let bytes = Bytes::new();
  file.read_to_end(bytes);
  bytes
}
```
Можно ещё:
1. Попробовать избавиться от `;` на концах строк
2. Сделать так, чтобы `File::open(path)` выглядел как `File.open(path)`
3. Сделать так, чтобы не были нужны `let` 


#### Точки-с-запятой `;`

- Разработчики смотрели на Js, Python, Scala, Haskell
- Случайный перевод строки может изменить логику
- В expression-based языках выражение ниже строчкой может удлинить время жизни объектов
```js
return
  a + b
```
```rust
fn test1() {
    a
    [b] // a[b] или a; [b], сколько будет жить a?
}
fn test2() {
    a
    ::b // a::b или a; ::b?
}
fn test3() {
    a
    - b // a - b или a; -b ?
}
fn test4() {
    break
    a // break a или break; a?
}
```


#### Универсальное правило

* `expr` вычисляет выражение и возвращает вычисленное значение
* `expr;` вычисляет выражение, отбрасывает вычисленное значение и возвращает `()`
* `;` это и разделитель, и приблизительный аналог функции `void`:
```rust
fn void(t: T) {
   // время жизни t подошло к концу
}
fn main() {
   let mut x = X::new();
   let mut y = { Y::new() };
   let mut z = {
      y.func();
      Z::new()
   };
}
```


#### Двоедвоеточия `::` vs `.`

- Может ли синтаксис обойтись только `.` в `File.open(path)`?
- Увы:
```rust
struct T {}
impl T {
    fn is_some() {}
}
fn main() {
    let T = Some(1);
    T::is_some(); 
    T.is_some(); // вызов Option::is_some(&T)
}
```
Следовательно, должны быть разные разделители:
- для доступа по модулям и пространствам имён
- для доступа к полям и методам

<!--
#### Альтернативы `::`?

Чисто эстетически можете оценить, насколько красивее альтернативы:
```rust
fn test() -> io::Result<()> {
    let _ = std::env::current_dir().map_err(|_| Error::File("..."))?;

    std:env:current_dir().map_err(|_| Error:File("..."))?; // Erlang
    std/env/current_dir().map_err(|_| Error/File("..."))?; // Nim
    std$env$current_dir().map_err(|_| Error$File("..."))?; // java bc
    std-env-current_dir().map_err(|_| Error-File("..."))?; // :-)
    std^env^current_dir().map_err(|_| Error^File("..."))?; // ^_^
}
```
-->


#### Избавиться от `let`?

<pre data-id="code-animation"><code class="hljs cplusplus" data-trim data-line-numbers="|2,4"><script type="text/template">
Framed<FramedWrite<WriteHalf<TcpStream>, 
   LengthDelimitedCodec>, Request, SomeType1, SomeType2> requests = ...;
Framed<FramedRead<ReadHalf<TcpStream>, 
   LengthDelimitedCodec>, Response, SomeType1, SomeType2> responses = ...;
</script></code></pre>
<pre data-id="code-animation"><code class="hljs rust" data-trim data-line-numbers="|4|5|6|1,7|10"><script type="text/template">
type Requests = Framed<FramedWrite<WriteHalf<TcpStream>, LengthDelimitedCodec>, Request, SomeType1, SomeType2>;

fn main() {
    let requests = ...; // если вывод типов полностью справляется
    let requests: Framed<_, _, _, _> = ...; // если надо немного помочь
    let requests: Framed<_, Request, SomeType1, SomeType2> = ...; // если надо чуть больше помочь
    let requests: Requests = ...; // если хочется вообще замести тип под ковёр 

    // часто можно помочь с помощью т.н. turbofish
    let vec = a_collection.iter().map(|x| ...).collect::<Vec<_>>();
}
</script></code></pre>
(Кстати, `|_|` пришёл из Ruby)


#### Камень в огород C++

Неоднозначность парсинга:
```сpp
template<typename T> class X {
    void foo() {
        T::A* pa;
        // typename T::A* pa; 
    }
} 
```
Трудность восприятия сигнатур типов:
```cpp
int (*(*foo)(void))[3]
void (*signal(int sig, void (*func)(int)))(int)
char * const (*(* const bar)[5])(int )
```
(https://cdecl.org/ в помощь)
Разработчики языка Rust грамотно обошли эти грабли