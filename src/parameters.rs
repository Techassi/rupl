use std::collections::HashMap;

use thiserror::Error;

use crate::error::ReplResult;

pub trait ConvertFrom<T>: Sized {
    fn convert(input: T) -> ReplResult<Self>;
}

impl ConvertFrom<String> for String {
    fn convert(input: String) -> ReplResult<Self> {
        Ok(input)
    }
}

#[derive(Debug, Error)]
pub enum ParameterError {
    #[error("Invalid parameter count")]
    InvalidParameterCount,

    #[error("No such parameter")]
    NoSuchParameter,

    #[error("Parse error")]
    ParseError,
}

#[derive(Debug, Clone)]
pub struct Parameters {
    inner: HashMap<String, (usize, Parameter)>,
    input: Vec<String>,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            inner: Default::default(),
            input: Default::default(),
        }
    }
}

impl Parameters {
    pub fn new<T>(input: T, params: Vec<Parameter>) -> Result<Self, ParameterError>
    where
        T: Into<String>,
    {
        let parts: Vec<String> = input
            .into()
            .trim()
            .split(" ")
            .map(|p| p.to_string())
            .collect();

        if parts.len() != params.len() {
            return Err(ParameterError::InvalidParameterCount);
        }

        let mut inner = HashMap::<String, (usize, Parameter)>::new();

        for (index, param) in params.iter().enumerate() {
            inner.insert(param.name.clone(), (index, param.clone()));
        }

        Ok(Self {
            input: parts,
            inner,
        })
    }

    pub fn get<T, N>(&self, name: N) -> Result<T, ParameterError>
    where
        T: ConvertFrom<String>,
        N: Into<String>,
    {
        let (index, _) = match self.inner.get(&name.into()) {
            Some(kv) => kv,
            None => return Err(ParameterError::NoSuchParameter),
        };

        match T::convert(self.input[*index].clone()) {
            Ok(v) => Ok(v),
            Err(_) => Err(ParameterError::ParseError),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Parameter {
    name: String,
}

impl Parameter {
    pub fn new<N>(name: N) -> Self
    where
        N: Into<String>,
    {
        Self { name: name.into() }
    }
}
