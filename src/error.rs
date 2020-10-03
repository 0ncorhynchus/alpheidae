use actix_web::client::SendRequestError;
use actix_web::error::PayloadError;
use serde_qs::Error as SerdeQsError;
use std::convert::From;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

macro_rules! impl_error {
    ($($Err:ident),*$(,)?) => {
        #[derive(Debug)]
        pub enum Error {
            $($Err($Err)),*
        }

        impl fmt::Display for Error {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    $(Self::$Err(error) => write!(f, "{}", error)),*
                }
            }
        }

        impl std::error::Error for Error {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                match self {
                    $(Self::$Err(error) => Some(error)),*
                }
            }
        }

        $(impl From<$Err> for Error {
            fn from(error: $Err) -> Self {
                Self::$Err(error)
            }
        })*
    }
}

impl_error! {
    SendRequestError,
    PayloadError,
    SerdeQsError,
}
