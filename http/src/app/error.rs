use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    UrlParsingError,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        // faz que os erros sejam exibidos de forma legivel
        match self{
            Error::UrlParsingError => write!(f, "Erro em fazer parsing da URL :("),
        }
    }
}