use std::{sync::Arc, time::Duration};

use auto_impl::auto_impl;
use tokio::time::sleep;

use crate::di::{Life, Test};

#[auto_impl(Arc)]
pub(crate) trait ConfigSvc {
    async fn nonsense_wait(&self);
}

pub(crate) trait Config {
    type Dep: ConfigSvc;
}

impl ConfigSvc for () {
    async fn nonsense_wait(&self) {}
}

impl Config for Test {
    type Dep = ();
}

pub(crate) struct ConfigImpl;

impl ConfigSvc for ConfigImpl {
    async fn nonsense_wait(&self) {
        sleep(Duration::from_secs(1)).await;
    }
}

impl Config for Life {
    type Dep = Arc<ConfigImpl>;
}
