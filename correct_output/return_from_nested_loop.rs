fn foo() {
    loop {
        let n = 1;
        let m = match fun_name() {
            Some(value) => value,
            None => return,
        };
        let h = 1 + m;
    }
}

fn fun_name() -> Option<i32> {
    let k = 1;
    loop {
        return None;
    }
    let m = k + 1;
    Some(m)
}