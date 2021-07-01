//!
//! # Common Types and Macros
//!
//! 
use lazy_static::lazy_static;
use ruc::*;
use serde::{de::DeserializeOwned, Serialize};
use std::{borrow::Cow, cmp::Ordering, 
env, 
fmt, 
fs, 
ops::Deref};


lazy_static! {
    /// the directory of cache
    pub static ref CACHE_DIR: String = env::var("FUNDB_DIR").unwrap_or_else(|_| "/tmp".to_owned());
}

/// try_twice
#[macro_export]
macro_rules! try_twice {
    ($ops: expr) => {
        ruc::pnk!($ops.c(d!()).or_else(|e| {
            e.print();
            $ops.c(d!())
        }))
    };
}

/// unique_path
#[macro_export]
macro_rules! unique_path {
    () => {
        format!(
            "{}/.fundb/{}/{}_{}_{}_{}",
            *$crate::helper::CACHE_DIR,
            ts!(),
            file!(),
            line!(),
            column!(),
            rand::random::<u32>()
        )
    };
}

/// new_vecx
#[macro_export]
macro_rules! new_vecx {
    ($ty: ty, $in_mem_cnt: expr) => {
        $crate::new_vecx_custom!($ty, $in_mem_cnt, false)
    };
    ($ty: ty) => {
        $crate::new_vecx_custom!($ty, None, false)
    };
    ($in_mem_cnt: expr) => {
        $crate::new_vecx_custom!($in_mem_cnt, false)
    };
    () => {
        $crate::new_vecx_custom!(false)
    };
}

/// new_vecx_custom
#[macro_export]
macro_rules! new_vecx_custom {
    ($ty: ty, $in_mem_cnt: expr, $is_tmp: expr) => {{
        let obj: $crate::Vecx<$ty> = $crate::try_twice!($crate::Vecx::new(
            $crate::unique_path!(),
            Some($in_mem_cnt)
            $is_tmp,
        ));
        obj
    }};
    ($ty: ty, $is_tmp: expr) => {{
        let obj: $crate::Vecx<$ty> =
            $crate::try_twice!($crate::Vecx::new($crate::unique_path!(), None, $is_tmp));
        obj
    }};
    ($in_mem_cnt: expr, $is_tmp: expr) => {
        $crate::try_twice!($crate::Vecx::new($crate::unique_path!(), Some($in_mem_cnt), $is_tmp))
    };
    ($is_tmp: expr) => {
        $crate::try_twice!($crate::Vecx::new($crate::unique_path!(), None, $is_tmp))
    };
}


/// new_mapx
#[macro_export]
macro_rules! new_mapx {
    ($ty: ty, $in_mem_cnt: expr) => {
        $crate::new_mapx_custom!($ty, $in_mem_cnt, false)
    };
    ($ty: ty) => {
        $crate::new_mapx_custom!($ty, None, false)
    };
    ($in_mem_cnt: expr) => {
        $crate::new_mapx_custom!($in_mem_cnt, false)
    };
    () => {
        $crate::new_mapx_custom!(false)
    };
}

/// new_mapx_custom
#[macro_export]
macro_rules! new_mapx_custom {
    ($ty: ty, $in_mem_cnt: expr, $is_tmp: expr) => {{
        let obj: $crate::Mapx<$ty> = $crate::try_twice!($crate::Mapx::new(
            $crate::unique_path!(),
            $in_mem_cnt,
            $is_tmp,
        ));
        obj
    }};
    ($ty: ty, $is_tmp: expr) => {{
        let obj: $crate::Mapx<$ty> =
            $crate::try_twice!($crate::Mapx::new($crate::unique_path!(), None, $is_tmp,));
        obj
    }};
    ($in_mem_cnt: expr, $is_tmp: expr) => {
        $crate::try_twice!($crate::Mapx::new(
            $crate::unique_path!(),
            $in_mem_cnt,
            $is_tmp
        ))
    };
    ($is_tmp: expr) => {
        $crate::try_twice!($crate::Mapx::new($crate::unique_path!(), None, $is_tmp,))
    };
}

////////////////////////////////////////////////////////////////////////////////
// Begin of the implementation of Value(returned by `self.get`) for Vecx/Mapx //
/******************************************************************************/

/// Returned by `.get(...)`
#[derive(Eq, Debug, Clone)]
pub struct Value<'a, V>
where
    V: Clone + Eq + PartialEq + Serialize + DeserializeOwned + fmt::Debug,
{
    value: Cow<'a, V>,
}

impl<'a, V> Value<'a, V>
where
    V: Clone + Eq + PartialEq + Serialize + DeserializeOwned + fmt::Debug,
{
    pub(crate) fn new(value: Cow<'a, V>) -> Self {
        Value { value }
    }

    /// Comsume the ownship and get the inner value.
    pub fn into_inner(self) -> Cow<'a, V> {
        self.value
    }
}

impl<'a, V> Deref for Value<'a, V>
where
    V: Clone + Eq + PartialEq + Serialize + DeserializeOwned + fmt::Debug,
{
    type Target = V;

    fn deref(&self) -> &Self::Target {
        // todo!()
        &self.value
    }
}

impl<'a, V> PartialEq for Value<'a, V>
where
    V: Clone + Eq + PartialEq + Serialize + DeserializeOwned + fmt::Debug,
{
    fn eq(&self, other: &Value<'a, V>) -> bool {
        // todo!()
        // self.value.eq(&other.value)
        self.value == other.value
    }
}

impl<'a, V> PartialEq<V> for Value<'a, V>
where
    V: Clone + Eq + PartialEq + Serialize + DeserializeOwned + fmt::Debug,
{
    fn eq(&self, other: &V) -> bool {
        // todo!()
        self.value.deref() == other
    }
}

impl<'a, V> PartialOrd<V> for Value<'a, V>
where
    V: fmt::Debug + Clone + Eq + PartialEq + Ord + PartialOrd + Serialize + DeserializeOwned,
{
    fn partial_cmp(&self, other: &V) -> Option<Ordering> {
        // todo!()
        self.value.deref().partial_cmp(other)
    }
}

impl<'a, V> From<V> for Value<'a, V>
where
    V: Clone + Eq + PartialEq + Serialize + DeserializeOwned + fmt::Debug,
{
    fn from(v: V) -> Self {
        // todo!()
        Value::new(Cow::Owned(v))
    }
}

impl<'a, V> From<Cow<'a, V>> for Value<'a, V>
where
    V: Clone + Eq + PartialEq + Serialize + DeserializeOwned + fmt::Debug,
{
    fn from(v: Cow<'a, V>) -> Self {
        // todo!()
        Value::new(v)
    }
}

impl<'a, V> From<Value<'a, V>> for Cow<'a, V>
where
    V: Clone + Eq + PartialEq + Serialize + DeserializeOwned + fmt::Debug,
{
    fn from(v: Value<'a, V>) -> Self {
        // todo!()
        // Value::new(Cow::Owned(v.clone()))
        v.into_inner()
    }
}

impl<'a, V> From<&V> for Value<'a, V>
where
    V: Clone + Eq + PartialEq + Serialize + DeserializeOwned + fmt::Debug,
{
    fn from(v: &V) -> Self {
        // todo!()
        Value::new(Cow::Owned(v.clone()))
    }
}

/****************************************************************************/
// End of the implementation of Value(returned by `self.get`) for Vecx/Mapx //
//////////////////////////////////////////////////////////////////////////////

#[inline(always)]
pub(crate) fn sled_open(path: &str, is_tmp: bool) -> Result<sled::Db> {
    // todo!()
    let mut cf = sled::Config::default().path(path).mode(sled::Mode::HighThroughput)
        .use_compression(false);

    if is_tmp {
        cf = cf.temporary(true);
    }
    cf.open().c(d!())
}

#[inline(always)]
pub(crate) fn read_db_len(path: &str) -> Result<usize> {
    // todo!()
    let bytes = fs::read(path).unwrap();
    let mut buffer = [0u8; 8];
    buffer.copy_from_slice(&bytes[0..8]);
    Ok(usize::from_le_bytes(buffer))
}

#[inline(always)]
pub(crate) fn write_db_len(path: &str, len: usize) -> Result<()> {
    // todo!()
    fs::write(path, usize::to_le_bytes(len)).c(d!())
}
