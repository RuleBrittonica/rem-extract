//- minicore: option
fn foo() -> Option<()> {
    let n = 1;
    let k = foo()?;
    fun_name();

    Some(())
}

fn fun_name() {
    Some(())
}
