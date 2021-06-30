//!
//! # Serde Vistor Implementation
//!
//! Used to restore an existing database.
//!

use serde::{Deserialize, Serialize};

pub(crate) struct FunDBVisitor;

impl<'de> serde::de::Visitor<'de> for FunDBVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut ::core::fmt::Formatter) -> core::fmt::Result {
        formatter.write_str("The fucking world is over!")
    }

    fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v.to_owned())
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct FunDBMeta<'a> {
    pub in_mem_cnt: usize,
    pub data_path: &'a str,
}
