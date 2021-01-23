use anyhow::Result;
use bevy::{
    ecs::QueryError,
    prelude::{error, In},
};

pub fn error_handler(In(result): In<Result<()>>) {
    match result {
        Ok(_) => {}
        Err(err) => {
            error!("{}", err);
        }
    }
}
pub fn querr_error_handler(In(result): In<Result<(), QueryError>>) {
    match result {
        Ok(_) => {}
        Err(err) => {
            error!("query error!");
        }
    }
}
