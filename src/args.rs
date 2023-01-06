use std::{
    collections::HashMap,
    net::{AddrParseError, Ipv4Addr, Ipv6Addr},
    num::ParseIntError,
};

use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArgError {
    #[error("Invalid arg count")]
    InvalidArgCount,

    #[error("No such arg")]
    NoSuchArg,

    #[error("Parse error")]
    ParseError(String),
}

impl From<ParseIntError> for ArgError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseError(value.to_string())
    }
}

impl From<AddrParseError> for ArgError {
    fn from(value: AddrParseError) -> Self {
        Self::ParseError(value.to_string())
    }
}

#[derive(Debug, Clone)]
pub struct Args {
    params: HashMap<String, (usize, Arg)>,
    values: Vec<String>,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            params: Default::default(),
            values: Default::default(),
        }
    }
}

impl Args {
    pub fn new<T>(input: T, params: Vec<Arg>) -> Result<Self, ArgError>
    where
        T: Into<String>,
    {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"--([a-zA-Z-]+)[\s|=]([^=\s]+)").unwrap();
        }

        let input: String = input.into();

        let pairs: Vec<(String, String)> = RE
            .captures_iter(&input)
            .map(|c| (c[1].to_string(), c[2].to_string()))
            .collect();

        if params.len() != pairs.len() {
            return Err(ArgError::InvalidArgCount);
        }

        let mut inner_params = HashMap::<String, (usize, Arg)>::new();
        let mut values = Vec::new();

        for (index, param) in params.iter().enumerate() {
            for pair in &pairs {
                if param.name != pair.0 {
                    continue;
                }

                inner_params.insert(param.name.clone(), (index, param.clone()));
                values.push(pair.1.clone());
            }
        }

        if params.len() != inner_params.len() {
            return Err(ArgError::InvalidArgCount);
        }

        Ok(Self {
            params: inner_params,
            values,
        })
    }

    pub fn get<T, N>(&self, name: N) -> Result<T, ArgError>
    where
        T: ConvertFrom<String>,
        N: Into<String>,
    {
        let (index, _) = match self.params.get(&name.into()) {
            Some(kv) => kv,
            None => return Err(ArgError::NoSuchArg),
        };

        match T::convert(self.values[*index].clone()) {
            Ok(v) => Ok(v),
            Err(err) => Err(err),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Arg {
    name: String,
}

// TODO (Techassi): Add optional parameters
impl Arg {
    pub fn new<N>(name: N) -> Self
    where
        N: Into<String>,
    {
        Self { name: name.into() }
    }
}

macro_rules! convert_from_impl {
    ($SelfT:ty) => {
        impl ConvertFrom<String> for $SelfT {
            fn convert(input: String) -> Result<Self, ArgError> {
                Ok(input.parse::<$SelfT>()?)
            }
        }
    };
}

pub trait ConvertFrom<T>: Sized {
    fn convert(input: T) -> Result<Self, ArgError>;
}

impl ConvertFrom<String> for String {
    fn convert(input: String) -> Result<Self, ArgError> {
        Ok(input)
    }
}

convert_from_impl!(u8);
convert_from_impl!(u16);
convert_from_impl!(u32);
convert_from_impl!(u64);
convert_from_impl!(u128);
convert_from_impl!(usize);

convert_from_impl!(i8);
convert_from_impl!(i16);
convert_from_impl!(i32);
convert_from_impl!(i64);
convert_from_impl!(i128);
convert_from_impl!(isize);

convert_from_impl!(Ipv4Addr);
convert_from_impl!(Ipv6Addr);
