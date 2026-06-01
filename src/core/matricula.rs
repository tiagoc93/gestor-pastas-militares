use crate::errors::AppError;

#[derive(Debug, Clone)]
pub struct Matricula {
    pub digitos: String,
    pub com_hifen: String,
    pub subpasta: String,
}

impl Matricula {
    pub fn parse(input: &str) -> Result<Self, AppError> {
        let input = input.trim();

        if input.is_empty() {
            return Err(AppError::MatriculaInvalida("Entrada vazia".to_string()));
        }

        let digitos = input.replace("-", "");

        if !digitos.chars().all(|c| c.is_ascii_digit()) {
            return Err(AppError::MatriculaInvalida(
                "Matrícula deve conter apenas números".to_string(),
            ));
        }

        if digitos.len() < 3 {
            return Err(AppError::MatriculaInvalida(
                "Matrícula deve ter pelo menos 3 dígitos".to_string(),
            ));
        }

        if digitos.len() > 8 {
            return Err(AppError::MatriculaInvalida(
                "Matrícula deve ter no máximo 8 dígitos".to_string(),
            ));
        }

        let com_hifen = if input.contains('-') {
            input.to_string()
        } else {
            let len = digitos.len();
            let (prefix, suffix) = digitos.split_at(len - 1);
            format!("{}-{}", prefix, suffix)
        };

        let prefixo = if digitos.len() > 4 {
            &digitos[..digitos.len() - 4]
        } else {
            &digitos
        };
        let subpasta = format!("LEV PM {}", prefixo);

        Ok(Matricula {
            digitos,
            com_hifen,
            subpasta,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_com_hifen() {
        let m = Matricula::parse("111111-1").unwrap();
        assert_eq!(m.digitos, "1111111");
        assert_eq!(m.com_hifen, "111111-1");
        assert_eq!(m.subpasta, "LEV PM 111");
    }

    #[test]
    fn test_parse_sem_hifen() {
        let m = Matricula::parse("1111111").unwrap();
        assert_eq!(m.digitos, "1111111");
        assert_eq!(m.com_hifen, "111111-1");
        assert_eq!(m.subpasta, "LEV PM 111");
    }

    #[test]
    fn test_parse_3_digitos() {
        let m = Matricula::parse("111").unwrap();
        assert_eq!(m.digitos, "111");
        assert_eq!(m.com_hifen, "11-1");
        assert_eq!(m.subpasta, "LEV PM 111");
    }

    #[test]
    fn test_parse_7_digitos() {
        let m = Matricula::parse("9207901").unwrap();
        assert_eq!(m.digitos, "9207901");
        assert_eq!(m.com_hifen, "920790-1");
        assert_eq!(m.subpasta, "LEV PM 920");
    }

    #[test]
    fn test_parse_invalido_vazio() {
        let result = Matricula::parse("");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalido_letras() {
        let result = Matricula::parse("111111a");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalido_curto() {
        let result = Matricula::parse("11");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalido_longo() {
        let result = Matricula::parse("123456789");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_8_digitos() {
        let m = Matricula::parse("12345678").unwrap();
        assert_eq!(m.digitos, "12345678");
        assert_eq!(m.com_hifen, "1234567-8");
        assert_eq!(m.subpasta, "LEV PM 1234");
    }

    #[test]
    fn test_parse_6_digitos() {
        let m = Matricula::parse("14320-0").unwrap();
        assert_eq!(m.digitos, "143200");
        assert_eq!(m.com_hifen, "14320-0");
        assert_eq!(m.subpasta, "LEV PM 14");
    }
}
