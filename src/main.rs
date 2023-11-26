use csv::{QuoteStyle, WriterBuilder};
use dbase::Reader;
use indicatif::{ProgressBar, ProgressStyle};
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        print_help(&args[0]);
        return Ok(());
    }

    if args.len() != 3 {
        eprintln!(
            "Uso: {} <caminho_para_arquivo_dbf> <caminho_para_arquivo_csv_de_saida>",
            args[0]
        );
        std::process::exit(1);
    }

    let caminho_dbf = &args[1];
    let caminho_csv = &args[2];
    println!("Arquivo Origem: {}", caminho_dbf);
    println!("Arquivo Destino: {}", caminho_csv);

    let metadata = std::fs::metadata(caminho_dbf)?;
    let tamanho = metadata.len() as f64 / 1024.0 / 1024.0;
    println!("Tamanho do arquivo: {:.2} MB", tamanho);

    let mut reader = Reader::from_path(caminho_dbf)?;

    let record_count = reader.header().num_records as u64;
    println!("Número de registros: {}", record_count);
    let pb = ProgressBar::new(record_count);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
        )
        .unwrap(),
    );

    let headers: Vec<String> = reader
        .fields()
        .iter()
        .map(|field| field.name().to_string())
        .collect();

    let mut wtr = WriterBuilder::new()
        .delimiter(b';')
        .quote_style(QuoteStyle::Always)
        .from_path(caminho_csv)?;

    wtr.write_record(&headers)?;

    for record_result in reader.iter_records() {
        let record = record_result?;
        let mut row: Vec<String> = Vec::new();

        for field_name in &headers {
            let value = match record.get(field_name.as_str()) {
                Some(dbase::FieldValue::Character(Some(value))) => value.trim().to_string(),
                Some(dbase::FieldValue::Character(None)) => "".to_string(),
                Some(dbase::FieldValue::Date(Some(value))) => value.to_string(),
                Some(dbase::FieldValue::Date(None)) => "".to_string(),
                Some(dbase::FieldValue::Float(Some(value))) => value.to_string(),
                Some(dbase::FieldValue::Float(None)) => "".to_string(),
                Some(dbase::FieldValue::Logical(Some(value))) => value.to_string(),
                Some(dbase::FieldValue::Logical(None)) => "".to_string(),
                Some(dbase::FieldValue::Numeric(Some(value))) => value.to_string(),
                Some(dbase::FieldValue::Numeric(None)) => "".to_string(),
                Some(dbase::FieldValue::Memo(value)) => value.to_string(),
                Some(dbase::FieldValue::Integer(value)) => value.to_string(),
                _ => "".to_string(),
            };

            row.push(value);
        }

        // Escreve a linha no arquivo CSV
        wtr.write_record(&row)?;
        pb.inc(1);
    }
    pb.finish_with_message("Conversão concluída");
    wtr.flush()?;
    println!("Arquivo convertido com sucesso!");
    println!("");
    println!("Criado por: Tiago Jaques Pereira <tjaquespereira@gmail.com>");
    Ok(())
}

fn print_help(program_name: &str) {
    println!(
        "Uso: {} <caminho_para_arquivo_dbf> <caminho_para_arquivo_csv_de_saida>",
        program_name
    );
    println!("Converte um arquivo DBF para um arquivo CSV.");
    println!("\nParâmetros:");
    println!("  <caminho_para_arquivo_dbf>    Caminho para o arquivo DBF de entrada.");
    println!("  <caminho_para_arquivo_csv_de_saida>    Caminho para o arquivo CSV de saída.");
    println!("\nExemplo de uso (Windows):");
    println!(
        "  {} C:\\Users\\user\\arquivo.dbf C:\\Users\\user\\arquivo.csv",
        program_name
    );
}
