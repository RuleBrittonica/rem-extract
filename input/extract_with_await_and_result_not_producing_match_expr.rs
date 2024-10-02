//- minicore: future, result
async fn foo() -> Result<(), ()> {
    async {}.await;
    Err(())?
}