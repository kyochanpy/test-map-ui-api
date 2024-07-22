pub mod point;
use derive_new::new;
use std::{marker::PhantomData, sync::Arc};

use crate::clients::mysql::MySqlConnection;

#[derive(new)]
pub struct ServiceImpl<T> {
    pub(crate) mysql_connection: Arc<MySqlConnection>,
    _marker: PhantomData<T>,
}
