use crate::opm::manifest::{Manifest, PackageInfo};
use crate::opm::resolver::Resolver;
use std::env;
use std::fs;
use std::path::PathBuf;
use anyhow::Result;

pub fn handle_opm_command(args: &[String]) -> Result<()> {
    if args.is_empty() {
        print_help();
        return Ok(());
    }

    let command = &args[0];
    let current_dir = env::current_dir()?;

    match command.as_str() {
        "init" => {
            let name = if args.len() > 1 {
                args[1].clone()
            } else {
                current_dir.file_name().unwrap_or_default().to_string_lossy().to_string()
            };
            init_project(&current_dir, &name)?;
        }
        "install" => {
            install_dependencies(&current_dir)?;
        }
        "build" | "run" => {
            println!("[opm] Perintah '{}' diteruskan ke OmniLang compiler (`omnilang exec`).", command);
        }
        "help" | "--help" | "-h" => {
            print_help();
        }
        _ => {
            println!("opm: Perintah tidak dikenali '{}'", command);
            print_help();
        }
    }

    Ok(())
}

fn init_project(dir: &PathBuf, name: &str) -> Result<()> {
    let manifest_path = dir.join("Omni.toml");
    if manifest_path.exists() {
        println!("[opm] Proyek sudah diinisialisasi. Omni.toml telah tersedia.");
        return Ok(());
    }

    let manifest = Manifest {
        package: PackageInfo {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            authors: Some(vec!["OmniLang Architect <architect@hakato99.com>".to_string()]),
            description: Some("Sebuah arsitektur paket OmniLang baru.".to_string()),
            entry: Some("src/main.omni".to_string()),
        },
        dependencies: Default::default(),
    };

    manifest.save_file(&manifest_path)?;
    println!("[opm] Berhasil membuat Omni.toml untuk '{}'.", name);

    let src_dir = dir.join("src");
    if !src_dir.exists() {
        fs::create_dir_all(&src_dir)?;
        let main_file = src_dir.join("main.omni");
        fs::write(main_file, "module main {\n\n    fn main() {\n        print(\"[opm] Modul ini berjalan sukses!\");\n    }\n}\n")?;
        println!("[opm] Menghasilkan kerangka kerja src/main.omni.");
    }

    Ok(())
}

fn install_dependencies(dir: &PathBuf) -> Result<()> {
    let manifest_path = dir.join("Omni.toml");
    if !manifest_path.exists() {
        anyhow::bail!("Omni.toml tidak ditemukan. Pastikan Anda berada di direktori akar Package Manager OmniLang.");
    }

    let manifest = Manifest::parse_file(&manifest_path)?;
    let resolver = Resolver::new(dir);
    resolver.resolve_all(&manifest)?;

    Ok(())
}

fn print_help() {
    println!("OmniLang Package Manager (opm)");
    println!("Penggunaan: omnilang pkg <perintah> [argumen]");
    println!("\nPerintah:");
    println!("  init [nama]   Inisialisasi proyek dan manifest Omni.toml baru");
    println!("  install       Mengunduh pustaka (*dependencies*) dari Omni.toml");
    println!("  run           Kompilasi dan eksekusi skrip utama");
    println!("  build         Kompilasi modul tanpa eksekusi");
}
