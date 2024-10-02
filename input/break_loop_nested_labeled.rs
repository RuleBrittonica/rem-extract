//- minicore: try
fn foo() {
    'bar: loop {
        loop {
            break 'bar;
        }
    }
}