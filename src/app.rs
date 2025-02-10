use std::{marker::PhantomData, sync::Arc};

use crate::{
    config::{Config, ConfigImpl, ConfigSvc},
    di::{di, Life},
    hello::{hello_life, Hello, HelloSvc},
    world::{world_life, World, WorldSvc},
};

#[allow(async_fn_in_trait)]
pub trait AppSvc {
    async fn run(&self);
}

pub(crate) struct AppImpl<M: Config + Hello + World> {
    config: di!(M, Config),
    hello: di!(M, Hello),
    world: di!(M, World),
}

impl<M: Config + Hello + World> AppSvc for AppImpl<M> {
    async fn run(&self) {
        self.config.nonsense_wait().await;
        let hello = self.hello.hello().await;
        let world = self.world.world().await;
        println!("{hello}, {world}!\n");
    }
}

pub fn app_life() -> impl AppSvc {
    let config: Arc<_> = ConfigImpl(PhantomData).into();
    let hello = hello_life(config.clone()).into();
    let world = world_life(config.clone()).into();

    AppImpl::<Life> {
        config,
        hello,
        world,
    }
}

#[cfg(test)]
fn app_test() -> impl AppSvc {
    use crate::di::Test;

    AppImpl::<Test> {
        config: (),
        hello: (),
        world: (),
    }
}

#[tokio::test]
async fn test_app() {
    app_test().run().await;
}
