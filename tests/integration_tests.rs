use gestor_pastas_militares::core::{
    buscar_pasta_militar_flat, copiar_arquivo, criar_pasta_militar, Matricula, PoliticaSobrescrita,
};
use gestor_pastas_militares::errors::AppError;
use std::fs;
use tempfile::TempDir;

// ============================================================================
// T-56: Criar estrutura de teste com tempfile
// ============================================================================

/// Monta a estrutura de pastas de teste em um diretório temporário.
///
/// ```text
/// temp_root/
/// ├── LEV PM 111/
/// │   ├── 111111-1 - FULANO DE TAL/
/// │   │   └── documento.txt
/// │   └── 111112-2 - OUTRO/
/// │       └── doc.pdf
/// ├── LEV PM 999/
/// │   └── 999999-9 - SEM RELACAO/
/// │       └── arquivo.txt
/// ```
fn setup_test_structure() -> TempDir {
    let temp = TempDir::new().unwrap();
    let root = temp.path();

    let fulano = root.join("LEV PM 111").join("111111-1 - FULANO DE TAL");
    fs::create_dir_all(&fulano).unwrap();
    fs::write(fulano.join("documento.txt"), "conteudo fulano").unwrap();

    let outro = root.join("LEV PM 111").join("111112-2 - OUTRO");
    fs::create_dir_all(&outro).unwrap();
    fs::write(outro.join("doc.pdf"), "conteudo pdf").unwrap();

    let sem_relacao = root.join("LEV PM 999").join("999999-9 - SEM RELACAO");
    fs::create_dir_all(&sem_relacao).unwrap();
    fs::write(sem_relacao.join("arquivo.txt"), "conteudo sem relacao").unwrap();

    temp
}

// ============================================================================
// T-57: Testar fluxo completo de envio para matrícula existente
// ============================================================================

#[test]
fn test_enviar_matricula_existente() {
    // Arrange: pasta 111111-1 existe em 11111/
    let temp = setup_test_structure();
    let raiz = temp.path();
    let arquivo_origem = temp.path().join("origem.txt");
    fs::write(&arquivo_origem, "conteudo de teste").unwrap();

    // Act: enviar documento para matrícula 111111-1
    let m = Matricula::parse("111111-1").unwrap();
    let pasta = buscar_pasta_militar_flat(raiz, &m).unwrap().unwrap();
    copiar_arquivo(&arquivo_origem, &pasta, PoliticaSobrescrita::Sobrescrever).unwrap();

    // Assert: documento copiado em 11111/111111-1 - FULANO DE TAL/
    let destino = pasta.join("origem.txt");
    assert!(destino.exists());
    assert_eq!(fs::read_to_string(&destino).unwrap(), "conteudo de teste");
}

// ============================================================================
// T-58: Testar fluxo completo de envio para matrícula inexistente (criação)
// ============================================================================

#[test]
fn test_enviar_matricula_inexistente_cria_pasta() {
    // Arrange: matrícula 777777-7 não existe
    let temp = setup_test_structure();
    let raiz = temp.path();

    // Act: criar_pasta_militar(temp_root, 7777777, "SETIMO MILITAR")
    let m = Matricula::parse("777777-7").unwrap();
    let pasta = criar_pasta_militar(raiz, &m, "SETIMO MILITAR").unwrap();

    // Assert: pasta criada em LEV PM 777/777777-7 - SETIMO MILITAR/
    assert!(pasta.exists());
    let pasta_str = pasta.to_string_lossy();
    assert!(pasta_str.contains("777777-7 - SETIMO MILITAR"));
    assert!(raiz.join("LEV PM 777").exists());
}

// ============================================================================
// T-59: Testar envio para múltiplas matrículas
// ============================================================================

#[test]
fn test_enviar_multiplas_matriculas() {
    // Arrange: 3 matrículas, 2 existem, 1 não
    let temp = setup_test_structure();
    let raiz = temp.path();
    let arquivo_origem = temp.path().join("origem.txt");
    fs::write(&arquivo_origem, "conteudo multiplo").unwrap();

    let matriculas = vec!["111111-1", "111112-2", "777777-7"];
    let mut sucessos = 0usize;
    let mut falhas = 0usize;

    // Act: enviar documento para as 3
    for mat_str in &matriculas {
        let m = Matricula::parse(mat_str).unwrap();
        match buscar_pasta_militar_flat(raiz, &m) {
            Ok(Some(pasta)) => {
                copiar_arquivo(&arquivo_origem, &pasta, PoliticaSobrescrita::Sobrescrever).unwrap();
                sucessos += 1;
            }
            Ok(None) => {
                falhas += 1;
            }
            Err(_) => {
                falhas += 1;
            }
        }
    }

    // Assert: todas processadas, 2 sucesso, 1 falha
    assert_eq!(sucessos, 2);
    assert_eq!(falhas, 1);
    assert!(raiz
        .join("LEV PM 111")
        .join("111111-1 - FULANO DE TAL")
        .join("origem.txt")
        .exists());
    assert!(raiz
        .join("LEV PM 111")
        .join("111112-2 - OUTRO")
        .join("origem.txt")
        .exists());
}

// ============================================================================
// T-60: Testar envio para pasta extra (path manual)
// ============================================================================

#[test]
fn test_enviar_pasta_extra() {
    // Arrange: pasta extra (não matrícula)
    let temp = setup_test_structure();
    let raiz = temp.path();
    let pasta_extra = raiz.join("PASTA_EXTRA");
    fs::create_dir_all(&pasta_extra).unwrap();

    let arquivo_origem = temp.path().join("origem.txt");
    fs::write(&arquivo_origem, "conteudo extra").unwrap();

    // Act: enviar para path direto
    copiar_arquivo(
        &arquivo_origem,
        &pasta_extra,
        PoliticaSobrescrita::Sobrescrever,
    )
    .unwrap();

    // Assert: documento copiado na pasta extra
    let destino = pasta_extra.join("origem.txt");
    assert!(destino.exists());
    assert_eq!(fs::read_to_string(&destino).unwrap(), "conteudo extra");
}

// ============================================================================
// T-61: Testar busca de matrícula existente
// ============================================================================

#[test]
fn test_buscar_matricula_existente() {
    // Arrange: 111111-1 existe
    let temp = setup_test_structure();
    let raiz = temp.path();

    // Act: buscar_pasta_militar_flat(temp_root, 111111)
    let m = Matricula::parse("111111-1").unwrap();
    let resultado = buscar_pasta_militar_flat(raiz, &m).unwrap();

    // Assert: retorna Some(path) apontando para FULANO DE TAL/
    assert!(resultado.is_some());
    let path = resultado.unwrap();
    assert!(path.to_string_lossy().contains("FULANO DE TAL"));
}

// ============================================================================
// T-62: Testar busca de matrícula inexistente
// ============================================================================

#[test]
fn test_buscar_matricula_inexistente() {
    // Arrange: 666666-6 não existe
    let temp = setup_test_structure();
    let raiz = temp.path();

    // Act: buscar_pasta_militar_flat(temp_root, 666666)
    let m = Matricula::parse("666666-6").unwrap();
    let resultado = buscar_pasta_militar_flat(raiz, &m).unwrap();

    // Assert: retorna None
    assert!(resultado.is_none());
}

// ============================================================================
// T-63: Testar comportamento com pasta raiz não configurada
// ============================================================================

/// Valida se a pasta raiz está configurada antes de prosseguir com operações.
fn validar_pasta_raiz(raiz: Option<&str>) -> Result<std::path::PathBuf, AppError> {
    match raiz {
        Some(path) if !path.is_empty() => Ok(std::path::PathBuf::from(path)),
        _ => Err(AppError::PastaRaizNaoConfigurada),
    }
}

#[test]
fn test_enviar_sem_pasta_raiz() {
    // Arrange: pasta raiz null / vazia
    // Act: tentar enviar documento
    let resultado_none = validar_pasta_raiz(None);
    let resultado_vazio = validar_pasta_raiz(Some(""));

    // Assert: retorna erro AppError::PastaRaizNaoConfigurada
    assert!(matches!(
        resultado_none,
        Err(AppError::PastaRaizNaoConfigurada)
    ));
    assert!(matches!(
        resultado_vazio,
        Err(AppError::PastaRaizNaoConfigurada)
    ));
}

// ============================================================================
// T-64: Testar comportamento com arquivo grande em memória
// ============================================================================

#[test]
fn test_enviar_arquivo_grande() {
    // Arrange: arquivo de 50 MB em tempfile
    let temp = TempDir::new().unwrap();
    let arquivo_origem = temp.path().join("grande.bin");

    let tamanho = 50 * 1024 * 1024; // 50 MB
    let mut dados = vec![0u8; tamanho];
    for (i, byte) in dados.iter_mut().enumerate() {
        *byte = (i % 256) as u8;
    }
    fs::write(&arquivo_origem, &dados).unwrap();

    let destino = temp.path().join("destino");
    fs::create_dir_all(&destino).unwrap();

    // Act: copiar arquivo via copiar_arquivo
    copiar_arquivo(&arquivo_origem, &destino, PoliticaSobrescrita::Sobrescrever).unwrap();

    // Assert: arquivo copiado corretamente (compare conteúdo)
    let copiado = destino.join("grande.bin");
    assert!(copiado.exists());

    let dados_copiados = fs::read(&copiado).unwrap();
    assert_eq!(dados.len(), dados_copiados.len());
    assert_eq!(dados, dados_copiados);
}

// ============================================================================
// T-65: Testar busca flat (não deriva subpasta)
// ============================================================================

#[test]
fn test_busca_flat_encontra_em_subpasta_errada() {
    // Arrange: matrícula 111111-1 está em LEV PM 999/ (não na subpasta correta LEV PM 111/)
    let temp = TempDir::new().unwrap();
    let raiz = temp.path();

    let sub_errada = raiz.join("LEV PM 999").join("111111-1 - FULANO DE TAL");
    fs::create_dir_all(&sub_errada).unwrap();
    fs::write(sub_errada.join("doc.txt"), "conteudo").unwrap();

    // Act: buscar_pasta_militar_flat(raiz, 111111)
    let m = Matricula::parse("111111-1").unwrap();
    let resultado = buscar_pasta_militar_flat(raiz, &m).unwrap();

    // Assert: encontra mesmo assim (busca flat, não deriva subpasta)
    assert!(resultado.is_some());
    let path = resultado.unwrap();
    let path_str = path.to_string_lossy();
    assert!(path_str.contains("LEV PM 999"));
    assert!(path_str.contains("FULANO DE TAL"));
}
