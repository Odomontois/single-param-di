use std::{marker::PhantomData, sync::Arc, time::Duration};

use auto_impl::auto_impl;
use tokio::time::sleep;

use crate::di::{service, Life, Test};

#[auto_impl(Arc)]
pub(crate) trait ConfigSvc {
    async fn nonsense_wait(&self);
}

service!(Config, ConfigSvc, ConfigImpl);

impl ConfigSvc for () {
    async fn nonsense_wait(&self) {}
}

pub(crate) struct ConfigImpl<Mode>(pub(crate) PhantomData<Mode>);

impl ConfigSvc for ConfigImpl<Life> {
    async fn nonsense_wait(&self) {
        sleep(Duration::from_secs(1)).await;
    }
}
