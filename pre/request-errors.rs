
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum GetPool{
    #[error(transparent)]
    PoolKeyError(#[from] PoolKeyError),
    #[error(transparent)]
    MelnetError(#[from] MelnetError),
    #[error(transparent)]
    BadRequest(#[from] BadRequest),
    
}
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum GetPool{
    #[error(transparent)]
    PoolKeyError(#[from] PoolKeyError),
    #[error(transparent)]
    MelnetError(#[from] MelnetError),
    #[error(transparent)]
    BadRequest(#[from] BadRequest),
    
}