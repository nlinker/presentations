use std::ops::{Add, Mul};

struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> where T: Add<T, Output=T> + Mul<T, Output=T> + Copy {
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
    pub fn dot(&self, other: &Point<T>) -> T {
        self.x * other.x + self.y * other.y
    }
}

enum Op {
    Incr(usize),
    Decr(usize),
}

fn main() {
    // let mut x = None;
    // x = Some(3);
    let mut arr = vec![1, 2, 3];
    let mut changes = Vec::new();
    // gather
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

    println!("{arr:?}");
}



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
