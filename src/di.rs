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
