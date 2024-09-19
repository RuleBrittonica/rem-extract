//- minicore: result
fn foo() -> Result<(), i64> {
    let n = 1;
    let k = foo()?;
    fun_name();

    Ok(())
}

fn fun_name() {
    Ok(())
}
