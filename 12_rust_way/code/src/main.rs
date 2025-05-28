use std::fmt::{Display, Formatter};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;
use Color::{Blue, Green, Red, Yellow};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Color {
    Red, Yellow, Green, Blue
}
impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

type Fire = ();

struct Car {
    color: Color,
    thread: Option<thread::JoinHandle<u8>>,
}
impl Car {
    fn new(color: Color, receiver: Arc<Mutex<Receiver<Fire>>>, place: Arc<AtomicU8>) -> Car {
        let thread = thread::spawn(move || {
            // ждём, пока не прозвучит сигнал к началу гонки,
            // сигнал состоит в том, что для receiver будет дропнут единственный sender
            if let Err(_) = receiver.lock().unwrap().recv() {
                println!("Go {color}!")
            }
            let mut distance = 0;
            for _ in 0..10_000_000 {
                distance += 1;
            }
            std::hint::black_box(distance);

            // поток возвращает номер места, которое заняла машинка
            // 1 - самый быстрый
            place.fetch_add(1, Ordering::SeqCst)
        });
        Car { color, thread: Some(thread) }
    }
}

fn fire(s: Sender<Fire>) {
    drop(s)
}

fn main() {
    let (s, r) = channel::<Fire>();
    let r = Arc::new(Mutex::new(r));
    let place = Arc::new(AtomicU8::new(1));

    let cars = [Red, Yellow, Green, Blue]
        .iter()
        .map(|c| Car::new(*c, r.clone(), place.clone())).collect::<Vec<Car>>();

    thread::sleep(Duration::from_millis(1000));
    fire(s); // поiхали!

    let mut pairs = Vec::with_capacity(cars.len());
    for car in cars {
        let p = car.thread.unwrap().join().unwrap();
        pairs.push((car.color, p));
    }
    pairs.sort_by(|a, b| a.1.cmp(&b.1));
    for pair in pairs {
        println!("Car {} takes place {}", pair.0, pair.1);
    }
}



// struct Point<T> {
//     x: T,
//     y: T
// }

// impl<T: Clone + Copy + std::ops::Add<T, Output=T + std::ops::Mul<T, Output=T>> Point<T> {
// pub fn new(x: T, y: T) -> Point<T> { ... }
//
// pub fn dot(&self, other: &Point<T>) -> T {
// self.x * other.x + self.y * other.y
// }
// }

// enum Op {
//     Incr(usize),
//     Decr(usize),
// }

// fn main() {
//     // let mut x = None;
//     // x = Some(3);
//     let mut arr = vec![1, 2, 3];
//     let mut changes = Vec::new();
//     // gather
//     for (i, a) in arr.iter().enumerate() {
//         if *a % 2 == 0 {
//             changes.push(Op::Incr(i));
//         } else {
//             changes.push(Op::Decr(i));
//         }
//     }
//     // apply
//     for ch in changes {
//         match ch {
//             Op::Incr(i) => arr[i] += 1,
//             Op::Decr(i) => arr[i] -= 1,
//         }
//     }
//
//     println!("{arr:?}");
// }



// fn mutate_vec(mut v: Vec<i32>) -> Vec<i32> {
//     v.push(42);
//     v
// }
//
// fn main() {
//     let xs = vec![1, 2, 3];
//     let ys = mutate_vec(xs);
//     let mut zs = ys;
//     // обращаться к `xs` и `ys` теперь нельзя, объект был перемещён в `zs`
//     // мы можем переместить в `zs` новый объект, а старый в `ws`
//     let ws = mem::replace(&mut zs, vec![4, 5]);
//     println!("zs = {:?}, ws = {:?}", zs, ws);
// }
