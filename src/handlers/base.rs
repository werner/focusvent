#[derive(FromForm)]
pub struct GetTransactionParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
