use thiserror::Error;

#[derive(Error, Debug)]
pub enum PkgError {
    // 业务逻辑的错误
    #[error("Phone format error")]
    #[allow(dead_code)]
    PostgresError,

    #[allow(dead_code)]
    #[error("Redis create pool error")]
    RedisCreatePoolError(#[from] deadpool_redis::CreatePoolError),

    #[allow(dead_code)]
    #[error("Redis pool error")]
    RedisPoolError2(#[from] deadpool_redis::PoolError),
}

#[derive(Error, Debug)]
pub enum OsError {
    // 业务逻辑的错误
    #[error("Phone format error")]
    #[allow(dead_code)]
    PhoneFormat,
    #[error("password format error")]
    #[allow(dead_code)]
    PasswordFormat,
    #[error("Not sent sim code")]
    #[allow(dead_code)]
    NotSentCode,
    #[error("Redis connect error")]
    #[allow(dead_code)]
    RedisConnectError,
    #[error("{0}")]
    Pkg(PkgError),
    // #[allow(dead_code)]
    // #[error(transparent)]
    // Pkg2(#[from] PkgError),
    // #[allow(dead_code)]
    // #[error("Redis create pool error")]
    // RedisCreatePoolError(#[from] deadpool_redis::CreatePoolError),

    // #[allow(dead_code)]
    // #[error("Redis pool error")]
    // RedisPoolError2(#[from] deadpool_redis::PoolError),
}

impl From<PkgError> for OsError {
    fn from(s: PkgError) -> Self {
        Self::Pkg(s)
    }
}

#[allow(dead_code)]
type PkgResult<T> = Result<T, PkgError>;

#[allow(dead_code)]
type OsResult<T> = Result<T, OsError>;

#[cfg(test)]
mod tests {

    use deadpool_redis::{Config, Runtime};

    use super::*;
    #[test]
    fn test_error() {
        {
            let res = error_all(true, false);
            match res {
                Ok(_) => panic!("应该是 PkgError::PostgresError 1"),
                Err(e) => match e {
                    OsError::Pkg(v) => match v {
                        PkgError::PostgresError => {}
                        _ => panic!("应该是 PkgError::PostgresError 2"),
                    },
                    _ => panic!("应该是 PkgError::PostgresError 3"),
                },
            }
        }
        {
            let res = error_all(false, true);
            match res {
                Ok(_) => panic!("应该是 OsError::NotSentCode 1"),
                Err(e) => match e {
                    OsError::NotSentCode => {}
                    _ => panic!("应该是 OsError::NotSentCode 3"),
                },
            }
        }
    }

    fn error_pkg(is_pkg: bool) -> PkgResult<()> {
        if is_pkg {
            return Err(PkgError::PostgresError);
        }

        Ok(())
    }

    fn error_os(is_os: bool) -> OsResult<()> {
        if is_os {
            return Err(OsError::NotSentCode);
        }

        Ok(())
    }

    fn error_all(is_pkg: bool, is_os: bool) -> OsResult<()> {
        error_pkg(is_pkg)?;
        error_os(is_os)?;
        if 2 > 5 {
            return Err(OsError::PhoneFormat);
        }
        if 2 > 5 {
            return Err(PkgError::PostgresError.into());
        }
        if 2 > 5 {
            let cfg = Config::from_url("url".to_string());

            let pool = cfg
                .create_pool(Some(Runtime::Tokio1))
                .map_err(|_| OsError::RedisConnectError)?;

            return Err(PkgError::PostgresError.into());
        }

        Ok(())
    }
}
