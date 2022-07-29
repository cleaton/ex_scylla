use rustler::types::atom;
use rustler::{Atom, Encoder, Env, Term};
pub trait ToElixir<T: Encoder> {
    const IS_UNWRAPPED: bool = false;
    fn ex(self) -> T;
}

pub enum ScyllaResult<R: Encoder, E: Encoder> {
    Unwrapped(R),
    Ok(R),
    Err(E),
}

impl<'a, R: Encoder, E: Encoder> Encoder for ScyllaResult<R, E> {
    fn encode<'b>(&self, env: Env<'b>) -> Term<'b> {
        match self {
            Self::Unwrapped(r) => r.encode(env),
            Self::Ok(r) => (atom::ok(), r).encode(env),
            Self::Err(e) => (atom::error(), e).encode(env),
        }
    }
}

pub trait ToRust<T> {
    fn r(self) -> T;
}

impl<A: Encoder, B: Encoder, S: ToElixir<A>, E: ToElixir<B>> ToElixir<ScyllaResult<A, B>>
    for Result<S, E>
{
    fn ex(self) -> ScyllaResult<A, B> {
        match self {
            Ok(s) if S::IS_UNWRAPPED => ScyllaResult::Unwrapped(s.ex()),
            Ok(s) => ScyllaResult::Ok(s.ex()),
            Err(e) => ScyllaResult::Err(e.ex()),
        }
    }
}
impl ToElixir<Atom> for () {
    const IS_UNWRAPPED: bool = true;
    fn ex(self) -> Atom {
        atom::ok()
    }
}

impl<A: Encoder, B: Into<A>> ToElixir<Vec<A>> for Vec<B> {
    fn ex(self) -> Vec<A> {
        self.into_iter().map(|v| v.into()).collect()
    }
}

macro_rules! async_elixir {
    ($env:ident, $opaque:expr, $e:expr) => {
        let pid = $env.pid();
        let mut owned_env = OwnedEnv::new();
        let opaque = owned_env
            .run(|env| -> NifResult<SavedTerm> { Ok(owned_env.save($opaque.in_env(env))) })?;
        runtime::rt().spawn(async move {
            let res = $e;
            owned_env.send_and_clear(&pid, |env| (opaque.load(env), res).encode(env));
        });
    };
}

macro_rules! to_elixir {
    ($from:ty, $to:ty, $e:expr) => {
        impl ToElixir<$to> for $from {
            fn ex(self) -> $to {
                $e(self)
            }
        }
    };
}

macro_rules! clone_enum {
    (@From $from:ident, $to:ident, $($fname:ident),*) => {
        impl From<$from> for $to {
            fn from(f: $from) -> Self {
                match f {
                    $(
                        $from::$fname => $to::$fname,
                    )*
                }
            }
        }
    };
    ($from:ident, $to:ident, { $($fname:ident),* }) => {
        #[derive(Debug, NifUnitEnum)]
        pub enum $to {
            $($fname),*
        }
        clone_enum!(@From $from, $to, $($fname),*);
        clone_enum!(@From $to, $from, $($fname),*);
    };
}

pub(crate) use async_elixir;
pub(crate) use clone_enum;
pub(crate) use to_elixir;
