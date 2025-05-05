use std::path;
use crate::app::error::Error;
#[derive(Debug, PartialEq)]

// struct q armazena os componentes principais de uma URL
pub struct UrlParser {
    // "protocolo" da URL, como "http", "https"..
    pub scheme: String,

    // nome do host ou dominio, como "localhost".
    pub host: String,

    // caminho da URL, como "/index.html" ou "/api/v1/users".
    pub path: String,
}


impl UrlParser{

    pub fn from(url: &str) -> Result<UrlParser, Error> {
        // se a URL ja começa com http ou https, mantém como está
        // else, adiciona http:// no começo
        let addr = if url.starts_with("http") || url.starts_with("https"){
            url.to_owned()
        }else {
            format!("http://{}", url)
        };
        // separa o esquema (http ou https) do resto da URL
        let mut split = addr.split("://");

        let scheme = split.next().unwrap().to_string();// pega o esquema
        split = split.next().unwrap().split("/");// agora separa o restante por /

        let host= split.next().unwrap().to_string(); // pega o host (domínio) jura? olha o nome da variável catapimbas

        let mut path = String::new();

         // Monta o path juntando tudo que sobrou depois do host
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
