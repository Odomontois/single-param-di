pub(crate) struct Life;

pub(crate) struct Test;

macro_rules! life {
    ($t: ident) => {
        <Life as $t>::Dep
    };
}
pub(crate) use life;

macro_rules! di {
    ($m: ty, $t: ident) => {
        <$m as $t>::Dep
    };
}

pub(crate) use di;

macro_rules! service {
    ( $t: ident, $svc: ident, $prod: ident) => {
        pub(crate) trait $t {
            type Dep: $svc;
        }

        impl $t for Life {
            type Dep = Arc<$prod<Life>>;
        }

        impl $t for Test {
            type Dep = ();
        }
    };
}

pub(crate) use service;
