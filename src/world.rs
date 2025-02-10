use std::sync::Arc;

use auto_impl::auto_impl;

use crate::{
    config::{Config, ConfigSvc},
    di::{di, life, Life, Test},
};

#[auto_impl(Arc)]
pub(crate) trait WorldSvc {
    async fn world(&self) -> String;
}

pub(crate) trait World {
    type Dep: WorldSvc;
}

impl WorldSvc for () {
    async fn world(&self) -> String {
        "World!".to_string()
    }
}

impl World for Test {
    type Dep = ();
}

pub(crate) struct WorldImpl<M: Config> {
    cfg: di!(M, Config),
}

impl<Mode: Config> WorldSvc for WorldImpl<Mode> {
    async fn world(&self) -> String {
        self.cfg.nonsense_wait().await;
        "Ebat".to_string()
    }
}

impl World for Life {
    type Dep = Arc<WorldImpl<Life>>;
}

pub(crate) fn world_life(cfg: life!(Config)) -> WorldImpl<Life> {
    WorldImpl::<Life> { cfg }
}

#[cfg(test)]
fn world_test() -> impl WorldSvc {
    WorldImpl::<Test> { cfg: () }
}

#[tokio::test]
async fn test_world() {
    assert_eq!(world_test().world().await, "Ebat");
}
