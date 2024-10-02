//- minicore: try
fn foo() {
    'bar: loop {
        loop {
            continue 'bar;
        }
    }
}