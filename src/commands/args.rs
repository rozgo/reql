use {Client, Result, Connection, IntoArg, Arg};
use types::FromJson;
use serde_json::value::Value;
use ql2::proto::{Term, Term_AssocPair as TermPair};
#[cfg(feature = "with-io")]
use tokio_core::reactor::{Handle, Remote};

impl IntoArg for Arg {
    fn into_arg(self) -> Arg {
        self
    }
}

impl IntoArg for Client {
    fn into_arg(self) -> Arg {
        Arg {
            string: self.query,
            term: self.term,
            pool: None,
            remote: None,
        }
    }
}

impl IntoArg for Term {
    fn into_arg(self) -> Arg {
        Arg {
            string: String::new(),
            term: Ok(self),
            pool: None,
            remote: None,
        }
    }
}

impl IntoArg for String {
    fn into_arg(self) -> Arg {
        Arg {
            string: format!(r#""{}""#, self),
            term: Term::from_json(self),
            pool: None,
            remote: None,
        }
    }
}

impl IntoArg for char {
    fn into_arg(self) -> Arg {
        Arg {
            string: format!("'{}'", self),
            term: Term::from_json(self),
            pool: None,
            remote: None,
        }
    }
}

impl<'a> IntoArg for &'a String {
    fn into_arg(self) -> Arg {
        Arg {
            string: format!(r#""{}""#, self),
            term: Term::from_json(self),
            pool: None,
            remote: None,
        }
    }
}

impl<'a> IntoArg for &'a str {
    fn into_arg(self) -> Arg {
        Arg {
            string: format!(r#""{}""#, self),
            term: Term::from_json(self),
            pool: None,
            remote: None,
        }
    }
}

impl IntoArg for f32 {
    fn into_arg(self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
            pool: None,
            remote: None,
        }
    }
}

impl IntoArg for i32 {
    fn into_arg(self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
            pool: None,
            remote: None,
        }
    }
}

impl IntoArg for u32 {
    fn into_arg(self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
            pool: None,
            remote: None,
        }
    }
}

impl IntoArg for f64 {
    fn into_arg(self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
            pool: None,
            remote: None,
        }
    }
}

impl IntoArg for i64 {
    fn into_arg(self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
            pool: None,
            remote: None,
        }
    }
}

impl IntoArg for u64 {
    fn into_arg(self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
            pool: None,
            remote: None,
        }
    }
}

impl IntoArg for bool {
    fn into_arg(self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
            pool: None,
            remote: None,
        }
    }
}

impl IntoArg for Value {
    fn into_arg(self) -> Arg {
        Arg {
            string: self.to_string(),
            term: Term::from_json(self),
            pool: None,
            remote: None,
        }
    }
}

#[cfg(feature = "with-io")]
impl IntoArg for Connection {
    fn into_arg(self) -> Arg {
        Arg {
            string: String::from("conn"),
            term: Ok(Term::new()),
            pool: Some(self),
            remote: None,
        }
    }
}

#[cfg(feature = "with-io")]
impl<'a> IntoArg for &'a Handle {
    fn into_arg(self) -> Arg {
        Arg {
            string: String::from("&handle"),
            term: Ok(Term::new()),
            pool: None,
            remote: Some(self.remote().clone()),
        }
    }
}

#[cfg(feature = "with-io")]
impl IntoArg for Remote {
    fn into_arg(self) -> Arg {
        Arg {
            string: String::from("remote"),
            term: Ok(Term::new()),
            pool: None,
            remote: Some(self),
        }
    }
}

impl Arg {
    /// Create a new command argument
    ///
    /// This is the return type of the `IntoArg` trait. You need to
    /// use `Arg::new` to create an argument when implementing that
    /// trait for any additional types that you want to pass to ReQL
    /// commands.
    ///
    /// ReQL commands are represented as `term`s so you must first
    /// convert your argument to a term and pass it as `term` to this method.
    /// For debugging and logging purposes, this method also requires that you
    /// pass the string representation of your argument i.e. `as_str`.
    pub fn new() -> Arg {
        Arg {
            string: String::new(),
            term: Ok(Term::new()),
            pool: None,
            remote: None,
        }
    }

    #[doc(hidden)]
    pub fn set_string(&mut self, string: &str) {
        self.string = string.into();
    }

    #[doc(hidden)]
    pub fn set_term(&mut self, term: Result<Term>) {
        self.term = term;
    }

    #[doc(hidden)]
    pub fn add_arg(&mut self, arg: Arg) {
        if let Some(pool) = arg.pool {
            self.pool = Some(pool);
        }
        if let Some(remote) = arg.remote {
            self.remote = Some(remote);
        }
        let mut error = None;
        if let Ok(ref mut term) = self.term {
            match arg.term {
                Ok(aterm) => term.mut_args().push(aterm),
                Err(e) => { error = Some(e); }
            }
        }
        if let Some(e) = error {
            self.term = Err(e);
        }
    }

    #[doc(hidden)]
    pub fn add_opt(&mut self, temp_pair: TermPair) {
        if let Ok(ref mut term) = self.term {
            term.mut_optargs().push(temp_pair);
        }
    }

    #[doc(hidden)]
    pub fn create_term_pair<T: ::IntoArg>(key: &str, val: T) -> Result<TermPair> {
        let mut temp = Term::new();
        temp.mut_args().push(val.into_arg().term?);
        let mut temp_pair = TermPair::new();
        temp_pair.set_key(key.into());
        temp_pair.set_val(temp);
        Ok(temp_pair)
    }
}
