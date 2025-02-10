use std::sync::Arc;

use auto_impl::auto_impl;

use crate::{
    config::{Config, ConfigSvc},
    di::{di, life, service, Life, Test},
};

service!(Hello, HelloSvc, HelloImpl);

#[auto_impl(Arc)]
pub(crate) trait HelloSvc {
    async fn hello(&self) -> String;
}

impl HelloSvc for () {
    async fn hello(&self) -> String {
        "Hello".to_string()
    }
}

pub(crate) struct HelloImpl<M: Config> {
    config: di!(M, Config),
}

impl<Mode: Config> HelloSvc for HelloImpl<Mode> {
    async fn hello(&self) -> String {
        self.config.nonsense_wait().await;
        "Bonjour".to_string()
    }
}
pub(crate) fn hello_life(cfg: life!(Config)) -> HelloImpl<Life> {
    HelloImpl::<Life> { config: cfg }
}

#[cfg(test)]
fn hello_test() -> impl HelloSvc {
    HelloImpl::<Test> { config: () }
}

#[tokio::test]
async fn test_hello() {
    assert_eq!(hello_test().hello().await, "Bonjour");
}
