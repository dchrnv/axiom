// axiom-seed CLI — компилятор кристалла якорей.
//
// Использование:
//   axiom-seed compile \
//     --charset config/charsets/ru_en_base.yaml \
//     --region  config/crystal_region.yaml \
//     [--output seeds/crystal_c0.yaml]
//     [--anchors-dir config/anchors]     # для collision-check
#![deny(unsafe_code)]

use std::path::PathBuf;

use axiom_config::anchor::AnchorSet;
use axiom_seed::charset::CharsetFile;
use axiom_seed::compiler::{SeedCompiler, anchors_to_yaml};
use axiom_seed::layout::CrystalRegion;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 || args[1] != "compile" {
        eprintln!("Использование: axiom-seed compile --charset <path> --region <path> [--output <path>] [--anchors-dir <path>]");
        eprintln!("  --charset     Путь к charset YAML (например, config/charsets/ru_en_base.yaml)");
        eprintln!("  --region      Путь к конфигурации региона (например, config/crystal_region.yaml)");
        eprintln!("  --output      Путь для записи результата (по умолчанию: seeds/crystal_c0.yaml)");
        eprintln!("  --anchors-dir Директория существующих якорей для collision-check");
        std::process::exit(1);
    }

    let opts = match parse_args(&args[2..]) {
        Ok(o) => o,
        Err(e) => { eprintln!("Ошибка аргументов: {e}"); std::process::exit(1); }
    };

    let charset = match CharsetFile::load(&opts.charset) {
        Ok(c) => c,
        Err(e) => { eprintln!("Ошибка загрузки charset {}: {e}", opts.charset.display()); std::process::exit(1); }
    };
    eprintln!("[axiom-seed] charset: {} графем (объявлено: {})", charset.graphemes().len(), charset.declared_total());

    let region = match CrystalRegion::load(&opts.region) {
        Ok(r) => r,
        Err(e) => { eprintln!("Ошибка загрузки region {}: {e}", opts.region.display()); std::process::exit(1); }
    };
    eprintln!("[axiom-seed] регион: origin={:?} size={:?}", region.origin, region.size);

    // Загрузить существующие якоря для collision-check
    let existing_positions: Vec<[i16; 3]> = if let Some(ref dir) = opts.anchors_dir {
        let set = AnchorSet::load_dir(dir).unwrap_or_else(|e| {
            eprintln!("[axiom-seed] предупреждение: не удалось загрузить якоря из {}: {e}", dir.display());
            AnchorSet::empty()
        });
        set.all_positions()
    } else {
        eprintln!("[axiom-seed] --anchors-dir не задан, collision-check пропущен");
        vec![]
    };
    eprintln!("[axiom-seed] существующих якорей для collision-check: {}", existing_positions.len());

    // Компиляция
    let anchors = match SeedCompiler::compile(&charset, &region, &existing_positions) {
        Ok(a) => a,
        Err(e) => { eprintln!("Ошибка компиляции: {e}"); std::process::exit(1); }
    };
    eprintln!("[axiom-seed] скомпилировано: {} якорей", anchors.len());

    let yaml = match anchors_to_yaml(
        &anchors,
        "Crystal C0 grapheme anchors — Anchor Crystal V1.0",
        "1.0",
    ) {
        Ok(y) => y,
        Err(e) => { eprintln!("Ошибка сериализации: {e}"); std::process::exit(1); }
    };

    if let Some(parent) = opts.output.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).ok();
        }
    }
    match std::fs::write(&opts.output, &yaml) {
        Ok(_) => eprintln!("[axiom-seed] записан: {}", opts.output.display()),
        Err(e) => { eprintln!("Ошибка записи {}: {e}", opts.output.display()); std::process::exit(1); }
    }
}

struct CompileOpts {
    charset: PathBuf,
    region: PathBuf,
    output: PathBuf,
    anchors_dir: Option<PathBuf>,
}

fn parse_args(args: &[String]) -> Result<CompileOpts, String> {
    let mut charset = None;
    let mut region = None;
    let mut output = PathBuf::from("seeds/crystal_c0.yaml");
    let mut anchors_dir = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--charset" => { charset = Some(PathBuf::from(next_arg(args, &mut i)?)); }
            "--region"  => { region  = Some(PathBuf::from(next_arg(args, &mut i)?)); }
            "--output"  => { output  = PathBuf::from(next_arg(args, &mut i)?); }
            "--anchors-dir" => { anchors_dir = Some(PathBuf::from(next_arg(args, &mut i)?)); }
            other => return Err(format!("неизвестный аргумент: {other}")),
        }
        i += 1;
    }
    Ok(CompileOpts {
        charset: charset.ok_or("--charset обязателен")?,
        region: region.ok_or("--region обязателен")?,
        output,
        anchors_dir,
    })
}

fn next_arg<'a>(args: &'a [String], i: &mut usize) -> Result<&'a str, String> {
    *i += 1;
    args.get(*i).map(|s| s.as_str()).ok_or_else(|| "ожидалось значение после флага".to_string())
}
