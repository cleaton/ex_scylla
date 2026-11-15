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

impl<R: Encoder, E: Encoder> Encoder for ScyllaResult<R, E> {
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

impl ToElixir<bool> for bool {
    const IS_UNWRAPPED: bool = true;
    fn ex(self) -> bool {
        self
    }
}

impl<A: Encoder, B: Into<A>> ToElixir<Vec<A>> for Vec<B> {
    fn ex(self) -> Vec<A> {
        self.into_iter().map(|v| v.into()).collect()
    }
}

macro_rules! async_elixir {
    ($env:ident, $opaque:expr, $e:expr) => {
        (|| -> NifResult<()> {
            let pid = $env.pid();
            let mut owned_env = OwnedEnv::new();
            let opaque = owned_env
                .run(|env| -> NifResult<SavedTerm> { Ok(owned_env.save($opaque.in_env(env))) })?;
            drop(runtime::rt().spawn(async move {
                let res = $e;
                owned_env.send_and_clear(&pid, |env| (opaque.load(env), res).encode(env));
            }));
            Ok(())
        })()
    };
    ($env:ident, $opaque:expr, $e:expr, |$env_ident:ident, $res_ident:ident| $enc:expr) => {
        (|| -> NifResult<()> {
            let pid = $env.pid();
            let mut owned_env = OwnedEnv::new();
            let opaque = owned_env
                .run(|env| -> NifResult<SavedTerm> { Ok(owned_env.save($opaque.in_env(env))) })?;
            drop(runtime::rt().spawn(async move {
                let $res_ident = $e;
                owned_env.send_and_clear(&pid, |$env_ident| {
                    (opaque.load($env_ident), $enc).encode($env_ident)
                });
            }));
            Ok(())
        })()
    };
}

// Like `async_elixir!`, but also copies `$args` into the owned env and decodes it
// with `$decode` on the Tokio worker instead of the BEAM scheduler. A decode error
// (anything that encodes, e.g. `{:error, {:parse_value, msg}}`) is sent as the reply.
macro_rules! async_elixir_decoded {
    ($env:ident, $opaque:expr, $args:expr, $decode:expr, |$decoded:ident| $e:expr, |$env_ident:ident, $res_ident:ident| $enc:expr) => {{
        let pid = $env.pid();
        let mut owned_env = OwnedEnv::new();
        let (opaque, args) = owned_env.run(|oenv| {
            (
                owned_env.save($opaque.in_env(oenv)),
                owned_env.save($args.in_env(oenv)),
            )
        });
        drop(runtime::rt().spawn(async move {
            let decoded = owned_env.run(|oenv| $decode(args.load(oenv)));
            let res = match decoded {
                Ok($decoded) => Ok($e),
                Err(err) => Err(err),
            };
            owned_env.send_and_clear(&pid, |$env_ident| {
                let reply = match res {
                    Ok($res_ident) => ($enc).encode($env_ident),
                    Err(err) => err.encode($env_ident),
                };
                (opaque.load($env_ident), reply).encode($env_ident)
            });
        }));
        NifResult::<()>::Ok(())
    }};
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
pub(crate) use async_elixir_decoded;
pub(crate) use clone_enum;
pub(crate) use to_elixir;
