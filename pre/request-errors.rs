#[derive(Error, Debug, Serialize, Deserialize)]
pub enum GetPool {
    #[error(transparent)]
    PoolKeyError(#[from] PoolKeyError),
    #[error(transparent)]
    NetworkError(#[from] NetworkError),
    #[error(transparent)]
    BadRequest(#[from] BadRequest),
}
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum GetPool {
    #[error(transparent)]
    PoolKeyError(#[from] PoolKeyError),
    #[error(transparent)]
    NetworkError(#[from] NetworkError),
    #[error(transparent)]
    BadRequest(#[from] BadRequest),
}
