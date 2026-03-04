use std::path::{Path, PathBuf};
use std::process::Command;
use crate::opm::manifest::{Dependency, Manifest};
use anyhow::{Context, Result};
use std::fs;

pub struct Resolver {
    pub project_dir: PathBuf,
    pub modules_dir: PathBuf,
}

impl Resolver {
    pub fn new<P: AsRef<Path>>(project_dir: P) -> Self {
        let p = project_dir.as_ref().to_path_buf();
        let m = p.join(".omni_modules");
        Self { project_dir: p, modules_dir: m }
    }

    pub fn resolve_all(&self, manifest: &Manifest) -> Result<()> {
        if manifest.dependencies.is_empty() {
            println!("[opm] No dependencies to install.");
            return Ok(());
        }

        if !self.modules_dir.exists() {
            fs::create_dir_all(&self.modules_dir)
                .with_context(|| "Failed to create .omni_modules directory")?;
        }

        for (name, dep) in &manifest.dependencies {
            self.resolve_dependency(name, dep)?;
        }
        
        println!("[opm] Semua dependensi selesai dikonfigurasi.");
        Ok(())
    }

    fn resolve_dependency(&self, name: &str, dep: &Dependency) -> Result<()> {
        let target_dir = self.modules_dir.join(name);
        
        // Cek jika modul sudah ada, asumsi ter-cache.
        // Pada versi stabil, harus cek tag/commit.
        if target_dir.exists() {
            println!("[opm] Dependency '{}' sudah ada di .omni_modules.", name);
            return Ok(());
        }

        match dep {
            Dependency::Git { git, tag, branch } => {
                println!("[opm] Mengunduh `{}` dari repository Git: {} ...", name, git);
                let mut cmd = Command::new("git");
                cmd.arg("clone").arg(git).arg(&target_dir);
                
                if let Some(t) = tag {
                     cmd.arg("--branch").arg(t);
                     cmd.arg("--depth").arg("1");
                } else if let Some(b) = branch {
                     cmd.arg("--branch").arg(b);
                     cmd.arg("--depth").arg("1");
                }

                let status = cmd.status().with_context(|| "Gagal mengeksekusi git clone. Apakah program git sudah terpasang?")?;
                if !status.success() {
                    anyhow::bail!("Git clone gagal untuk module '{}'", name);
                }
            }
            Dependency::Path { path } => {
                println!("[opm] Menautkan lokal module `{}` ...", name);
                let source_path = self.project_dir.join(path);
                if !source_path.exists() {
                    anyhow::bail!("Path lokal modul {:?} tidak ditemukan", source_path);
                }
                
                self.copy_dir_all(&source_path, &target_dir)?;
            }
            Dependency::Version(v) => {
                println!("[opm] Peringatan: Registry Terpusat belum diluncurkan. Tidak bisa resolve versi `{}` untuk `{}`.", v, name);
            }
        }
        Ok(())
    }

    fn copy_dir_all(&self, src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
        fs::create_dir_all(&dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let ty = entry.file_type()?;
            if ty.is_dir() {
                self.copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
            } else {
                fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
            }
        }
        Ok(())
    }
}
