### №5. Почему Раст так долго компилируется?

- Компилятор Rust делает больше, чем многие другие
- Семантический анализ, кодогенерация через LLVM
- _Мономорфизация_ это способ компилировать полиморфный код, создавая специализированный вариант для каждого типа:

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


### Способы уменьшения времени компиляции

1. Обычно важно время в _dev.cycle_
2. Учесть, что крейты компилируются параллельно
3. Применить новый компоновщик (linker☺️) [MOLD](https://github.com/rui314/mold)
4. Можно снизить накладные расходы на мономорфизацию
5. Можно снизить накладные расходы на макросы
6. Для экспериментирования с кодом можно выделить рабочую функцию или типаж в отдельный файл
7. Jupyter notebook https://github.com/evcxr/evcxr
8. Scripting: Rhai, Rune, Lua, Koto; иногда Python и JS
    - бенчмарки https://github.com/khvzak/script-bench-rs

