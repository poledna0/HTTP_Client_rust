use std::path;
use crate::app::error::Error;
#[derive(Debug, PartialEq)]
pub struct UrlParser{
    pub scheme : String,
    pub host: String,
    pub path: String
}

impl UrlParser{

    pub fn from(url: &str) -> Result<UrlParser, Error> {
        let addr = if url.starts_with("http") || url.starts_with("https"){
            url.to_owned()
        }else {
            format!("http://{}", url)
        };

        let mut split = addr.split("://");

        let scheme = split.next().unwrap().to_string();
        split = split.next().unwrap().split("/");

        let host= split.next().unwrap().to_string();

        let mut path = String::new();

        loop{

            match split.next() {
                Some(v) => path.push_str(format!("/{}", v).as_str()),
                None => {
                    if path.is_empty(){
                        path.push('/');
                    }
                    break;
                }
                
            }
        }



        Ok(
            UrlParser { scheme, host, path }
        )
    }
}

#[cfg(test)]
mod test {

    use super::UrlParser;

    #[test]
    fn test1_funciona() {
        let url = "https://pucpr.br";
        let result = UrlParser::from(url).unwrap();

        let esperado = UrlParser {
            scheme: "https".to_owned(),
            host: "pucpr.br".to_owned(),
            path: "/".to_owned(),
        };
        assert_eq!(result, esperado)
    }

    #[test]
    fn test2_funciona() {
        let url = "pucpr.br";
        let result = UrlParser::from(url).unwrap();

        let esperado = UrlParser {
            scheme: "http".to_owned(),
            host: "pucpr.br".to_owned(),
            path: "/".to_owned(),
        };

        assert_eq!(result, esperado)
    }

    #[test]
    fn test3_naofunciona() {
        let url = "pucpr.br";
        let result = UrlParser::from(url).unwrap();

        let esperado = UrlParser {
            scheme: "".to_owned(),
            host: "pucpr.br".to_owned(),
            path: "/".to_owned(),
        };

        assert_ne!(result, esperado)
    }
}