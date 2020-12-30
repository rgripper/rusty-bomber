use anyhow::Result;
use bevy::prelude::{error, In};

pub fn error_handler(In(result): In<Result<()>>) {
    match result {
        Ok(_) => {}
        Err(err) => {
            error!("{}", err);
        }
    }
}
