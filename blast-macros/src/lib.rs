extern crate blast_interface;
extern crate blast_proc_macros;
extern crate rocket;

#[allow(unused_imports)]
use blast_interface::error::Respondable;

#[allow(unused_imports)]
use rocket::catch;

#[macro_export]
macro_rules! maperr {
(on $enum:ident ... $($variant:ident => $code:expr),* $(,)?) => {
    // Generated implementation of From<$enum> for rocket::http::Status
    //
    // Made by blast-macros <3
    impl From<$enum> for rocket::http::Status {
        fn from(error: $enum) -> rocket::http::Status {
            match error {
                $($enum::$variant => rocket::http::Status::from_code($code).unwrap(),)*
            }
        }
    }

    impl Into<$enum> for rocket::http::Status {
        fn into(self) -> $enum {
            let code = self.code;
            match code {
                $($code => $enum::$variant,)*
                _ => panic!("Invalid status code"),
            }
        }
    }

    impl From<(rocket::http::Status, &rocket::Request<'_>)> for $enum {
        fn from((status, req): (rocket::http::Status, &rocket::Request<'_>)) -> $enum {
            let code = status.code;
            match code {
                $($code => $enum::$variant,)*
                _ => panic!("Invalid status code"),
            }
        }
    }

    // Generated error interface extension
    //
    // Made by blast-macros <3
    impl Respondable<$enum> for $enum {
        type Payload = rocket::http::Status;
    }
};
}

#[macro_export]
macro_rules! catchers {
    (using $enum:ident ... $($func_name:ident, $variant:ident => (($code:expr, $req:ident))),* $(,)?) => {
        /// Catches HTTP errors and coerces them into custom AppErrors.
        #[catch(default)]
        pub async fn default_catcher(status: rocket::http::Status, req: &rocket::Request<'_>) -> $enum {
            return $enum::from((status, req));
        }

        $(
            /// Catches specific HTTP errors.
            #[catch($code)]
            pub async fn $func_name(req: &rocket::Request<'_>) -> $enum {
                return $enum::from((rocket::http::Status::from_code($code).unwrap(), req));
            }
        )*
    };
}
