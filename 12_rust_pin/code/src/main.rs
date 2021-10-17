use std::mem;

fn mutate_vec(mut v: Vec<i32>) -> Vec<i32> {
    v.push(42);
    v
}

fn main() {
    let xs = vec![1, 2, 3];
    let ys = mutate_vec(xs);
    let mut zs = ys;
    // обращаться к `xs` и `ys` теперь нельзя, объект был перемещён в `zs`
    // мы можем переместить в `zs` новый объект, а старый в `ws`
    let ws = mem::replace(&mut zs, vec![4, 5]);
    println!("zs = {:?}, ws = {:?}", zs, ws);
}
