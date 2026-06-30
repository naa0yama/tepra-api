//! Build script: compiles CSS via Tailwind 4 + `DaisyUI` 5, copies fonts and HTMX into `OUT_DIR`.

use std::path::Path;
use std::process::Command;

use anyhow::{Context as _, anyhow};

fn main() -> anyhow::Result<()> {
    // 1. pnpm presence check
    let pnpm_check = Command::new("pnpm").arg("--version").output();
    match pnpm_check {
        Ok(out) if out.status.success() => {}
        Ok(out) => {
            return Err(anyhow!(
                "pnpm --version failed: {}\nHint: run `mise install` at the repo root.",
                String::from_utf8_lossy(&out.stderr)
            ));
        }
        Err(e) => {
            return Err(anyhow!(
                "pnpm not found on PATH ({e}).\nHint: mise must be activated. Run `mise exec -- cargo build`."
            ));
        }
    }

    // 2. Install frontend deps from lockfile
    let install = Command::new("pnpm")
        .args(["install", "--frozen-lockfile"])
        .status()
        .context("failed to run pnpm install")?;
    if !install.success() {
        return Err(anyhow!(
            "pnpm install --frozen-lockfile failed. \
             Run `pnpm install` locally, commit the updated lockfile, retry."
        ));
    }

    // 3. Compile CSS
    let out_dir = std::env::var("OUT_DIR").context("OUT_DIR not set")?;
    let out_css = Path::new(&out_dir).join("app.css");
    let build = Command::new("pnpm")
        .args(["exec", "tailwindcss", "-i", "src/styles/app.css", "-o"])
        .arg(&out_css)
        .arg("--minify")
        .status()
        .context("failed to run tailwindcss")?;
    if !build.success() {
        return Err(anyhow!(
            "tailwindcss compilation failed. Check src/styles/app.css."
        ));
    }

    // 4. Copy @fontsource woff2 files
    let fonts_dir = Path::new(&out_dir).join("fonts");
    for family in ["ibm-plex-sans-jp", "ibm-plex-mono"] {
        let font_src = Path::new("node_modules/@fontsource")
            .join(family)
            .join("files");
        let font_out = fonts_dir.join(family);
        std::fs::create_dir_all(&font_out)
            .with_context(|| format!("failed to create font dir: {}", font_out.display()))?;
        for entry in walkdir::WalkDir::new(&font_src)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
            .filter(|e| e.path().extension().is_some_and(|x| x == "woff2"))
        {
            let dest = font_out.join(entry.file_name());
            std::fs::copy(entry.path(), &dest).with_context(|| {
                format!(
                    "failed to copy font: {} -> {}",
                    entry.path().display(),
                    dest.display()
                )
            })?;
        }
        println!("cargo:rerun-if-changed={}", font_src.display());
    }

    // 5. Copy HTMX
    let htmx_src = Path::new("node_modules/htmx.org/dist/htmx.min.js");
    let htmx_dst = Path::new(&out_dir).join("htmx.min.js");
    std::fs::copy(htmx_src, &htmx_dst).with_context(|| {
        format!(
            "failed to copy htmx.min.js: {}\nEnsure htmx.org is in package.json devDependencies.",
            htmx_src.display()
        )
    })?;
    println!("cargo:rerun-if-changed={}", htmx_src.display());

    // 6. rerun-if-changed
    println!("cargo:rerun-if-changed=package.json");
    println!("cargo:rerun-if-changed=pnpm-lock.yaml");
    println!("cargo:rerun-if-changed=src/styles");

    for entry in walkdir::WalkDir::new("../tepra/templates")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        println!("cargo:rerun-if-changed={}", entry.path().display());
    }

    Ok(())
}
