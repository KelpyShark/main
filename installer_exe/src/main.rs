/// KelpyShark Standalone Installer
///
/// A self-contained EXE that:
///   1. Shows a welcome banner
///   2. Asks the user for an install directory (with sensible default)
///   3. Extracts the embedded `kelpy` CLI binary
///   4. Creates standard directory structure
///   5. Creates example files and starter project
///   6. Adds the install directory to the user PATH (Windows)
///   7. Creates an uninstaller script
///   8. Verifies the installation
///
/// Build workflow:
///   1. `cargo build --release --bin kelpyshark-cli`   (build the language)
///   2. `cargo build --release --bin kelpyshark-setup`  (build the installer)
///
/// The installer embeds the CLI binary at compile time via `include_bytes!`.

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

// ═══════════════════════════════════════════════════════════════
// Embedded binary
// ═══════════════════════════════════════════════════════════════

/// The pre-built kelpy CLI binary, embedded at compile time.
/// The path is set by build.rs via KELPY_CLI_BINARY env var.
const KELPY_BINARY: &[u8] = include_bytes!(env!("KELPY_CLI_BINARY"));

// ═══════════════════════════════════════════════════════════════
// Constants
// ═══════════════════════════════════════════════════════════════

const VERSION: &str = "0.1.0";

#[cfg(target_os = "windows")]
const BINARY_NAME: &str = "kelpy.exe";

#[cfg(not(target_os = "windows"))]
const BINARY_NAME: &str = "kelpy";

// ═══════════════════════════════════════════════════════════════
// Colours (ANSI escape codes)
// ═══════════════════════════════════════════════════════════════

const CYAN: &str = "\x1b[36m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const RESET: &str = "\x1b[0m";

// ═══════════════════════════════════════════════════════════════
// Entry point
// ═══════════════════════════════════════════════════════════════

fn main() {
    let result = run_installer();
    match result {
        Ok(()) => {
            println!();
            println!("  {GREEN}{BOLD}Installation complete!{RESET}");
            println!();
            prompt_enter("  Press Enter to exit...");
        }
        Err(e) => {
            println!();
            println!("  {RED}{BOLD}Installation failed:{RESET} {e}");
            println!();
            prompt_enter("  Press Enter to exit...");
            std::process::exit(1);
        }
    }
}

fn run_installer() -> Result<(), String> {
    print_banner();
    print_system_info();

    // ── Choose install directory ──
    let default_dir = default_install_dir();
    println!(
        "  {BOLD}Install directory{RESET} {DIM}(press Enter for default){RESET}"
    );
    println!("  Default: {CYAN}{}{RESET}", default_dir.display());
    print!("  > ");
    io::stdout().flush().ok();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("Failed to read input: {e}"))?;
    let input = input.trim();

    let install_dir = if input.is_empty() {
        default_dir
    } else {
        PathBuf::from(input)
    };

    println!();
    println!(
        "  {YELLOW}▸{RESET} Installing to: {CYAN}{}{RESET}",
        install_dir.display()
    );
    println!();

    // ── Confirm ──
    print!("  Proceed with installation? {DIM}[Y/n]{RESET} ");
    io::stdout().flush().ok();
    let mut confirm = String::new();
    io::stdin()
        .read_line(&mut confirm)
        .map_err(|e| format!("Failed to read input: {e}"))?;
    let confirm = confirm.trim().to_lowercase();
    if confirm == "n" || confirm == "no" {
        return Err("Installation cancelled by user.".to_string());
    }
    println!();

    // ── Step 1: Create directory structure ──
    step("1/7", "Creating directory structure...");
    let bin_dir = install_dir.join("bin");
    let lib_dir = install_dir.join("lib");
    let registry_dir = install_dir.join("registry");
    let examples_dir = install_dir.join("examples");

    for dir in [&bin_dir, &lib_dir, &registry_dir, &examples_dir] {
        fs::create_dir_all(dir).map_err(|e| format!("Cannot create {}: {e}", dir.display()))?;
    }
    done();

    // ── Step 2: Extract kelpy binary ──
    step("2/7", "Extracting kelpy binary...");
    let binary_path = bin_dir.join(BINARY_NAME);
    fs::write(&binary_path, KELPY_BINARY)
        .map_err(|e| format!("Failed to write binary: {e}"))?;

    // Make executable on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&binary_path)
            .map_err(|e| format!("Cannot read permissions: {e}"))?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&binary_path, perms)
            .map_err(|e| format!("Cannot set permissions: {e}"))?;
    }

    let binary_size = KELPY_BINARY.len();
    println!(
        "    {DIM}Wrote {BINARY_NAME} ({:.1} MB){RESET}",
        binary_size as f64 / (1024.0 * 1024.0)
    );
    done();

    // ── Step 3: Write example files ──
    step("3/7", "Creating example programs...");
    write_example_files(&examples_dir)?;
    done();

    // ── Step 4: Create starter kelpy.toml ──
    step("4/7", "Creating default configuration...");
    let toml_path = install_dir.join("kelpy.toml");
    if !toml_path.exists() {
        fs::write(
            &toml_path,
            "[package]\nname = \"kelpyshark\"\nversion = \"0.1.0\"\ndescription = \"KelpyShark installation\"\n",
        )
        .map_err(|e| format!("Failed to write kelpy.toml: {e}"))?;
    }
    done();

    // ── Step 5: Create uninstaller ──
    step("5/7", "Creating uninstaller...");
    write_uninstaller(&install_dir)?;
    done();

    // ── Step 6: Update PATH ──
    step("6/7", "Updating system PATH...");
    let path_updated = add_to_path(&bin_dir);
    if path_updated {
        println!("    {DIM}Added {} to user PATH{RESET}", bin_dir.display());
    } else {
        println!("    {DIM}PATH already contains {}{RESET}", bin_dir.display());
    }
    done();

    // ── Step 7: Verify ──
    step("7/7", "Verifying installation...");
    verify_installation(&binary_path)?;
    done();

    // ── Summary ──
    println!();
    println!("  {GREEN}═══════════════════════════════════════════{RESET}");
    println!("  {GREEN}{BOLD}  🦈 KelpyShark {VERSION} installed!{RESET}");
    println!("  {GREEN}═══════════════════════════════════════════{RESET}");
    println!();
    println!("  {BOLD}Installed to:{RESET}  {}", install_dir.display());
    println!("  {BOLD}Binary:{RESET}        {}", binary_path.display());
    println!("  {BOLD}Examples:{RESET}      {}", examples_dir.display());
    println!();
    println!("  {BOLD}Get started:{RESET}");
    println!("    {CYAN}kelpy repl{RESET}                        — Start the REPL");
    println!(
        "    {CYAN}kelpy run {}{RESET}  — Run an example",
        examples_dir.join("hello.ks").display()
    );
    println!("    {CYAN}kelpy new myproject{RESET}               — Create a project");
    println!("    {CYAN}kelpy build file.ks --target c{RESET}    — Compile to C");
    println!("    {CYAN}kelpy build file.ks --target js{RESET}   — Compile to JavaScript");
    println!();

    if path_updated {
        println!(
            "  {YELLOW}⚠  Restart your terminal for PATH changes to take effect.{RESET}"
        );
        println!();
    }

    Ok(())
}

// ═══════════════════════════════════════════════════════════════
// UI helpers
// ═══════════════════════════════════════════════════════════════

fn print_banner() {
    println!();
    println!("  {CYAN}{BOLD}╔══════════════════════════════════════════╗{RESET}");
    println!("  {CYAN}{BOLD}║                                          ║{RESET}");
    println!("  {CYAN}{BOLD}║   🦈  KelpyShark Installer  v{VERSION}      ║{RESET}");
    println!("  {CYAN}{BOLD}║                                          ║{RESET}");
    println!("  {CYAN}{BOLD}║   A readable, versatile programming      ║{RESET}");
    println!("  {CYAN}{BOLD}║   language for everyone.                 ║{RESET}");
    println!("  {CYAN}{BOLD}║                                          ║{RESET}");
    println!("  {CYAN}{BOLD}╚══════════════════════════════════════════╝{RESET}");
    println!();
}

fn print_system_info() {
    println!("  {DIM}OS:   {}{RESET}", env::consts::OS);
    println!("  {DIM}Arch: {}{RESET}", env::consts::ARCH);
    println!();
}

fn step(num: &str, msg: &str) {
    println!("  {CYAN}[{num}]{RESET} {msg}");
}

fn done() {
    println!("        {GREEN}✓ Done{RESET}");
}

fn prompt_enter(msg: &str) {
    print!("{msg}");
    io::stdout().flush().ok();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).ok();
}

// ═══════════════════════════════════════════════════════════════
// Default install directory
// ═══════════════════════════════════════════════════════════════

fn default_install_dir() -> PathBuf {
    if cfg!(target_os = "windows") {
        if let Ok(profile) = env::var("USERPROFILE") {
            PathBuf::from(profile).join(".kelpyshark")
        } else {
            PathBuf::from("C:\\KelpyShark")
        }
    } else if let Ok(home) = env::var("HOME") {
        PathBuf::from(home).join(".kelpyshark")
    } else {
        PathBuf::from("/usr/local/kelpyshark")
    }
}

// ═══════════════════════════════════════════════════════════════
// Example files
// ═══════════════════════════════════════════════════════════════

fn write_example_files(dir: &Path) -> Result<(), String> {
    let examples = [
        (
            "hello.ks",
            r#"# Welcome to KelpyShark!

name = "World"
print "Hello, {$name}! 🦈"

def greet(who) {
    print "Welcome to KelpyShark, {$who}!"
}

greet("Developer")
"#,
        ),
        (
            "fizzbuzz.ks",
            r#"# FizzBuzz in KelpyShark
i = 1
while i <= 30 {
    if i % 15 == 0 {
        print "FizzBuzz"
    } else {
        if i % 3 == 0 {
            print "Fizz"
        } else {
            if i % 5 == 0 {
                print "Buzz"
            } else {
                print i
            }
        }
    }
    i = i + 1
}
"#,
        ),
        (
            "functions.ks",
            r#"# Functions and recursion

def factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

def fibonacci(n) {
    if n <= 1 {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

print "5! = " + str(factorial(5))
print "fib(10) = " + str(fibonacci(10))
"#,
        ),
        (
            "data_structures.ks",
            r#"# Lists and Dictionaries

fruits = ["apple", "banana", "cherry", "date"]
print "Fruits:"
for fruit in fruits {
    print "  - " + fruit
}
print "Count: " + str(len(fruits))

person = {"name": "Alice", "language": "KelpyShark"}
print person["name"] + " uses " + person["language"]
"#,
        ),
    ];

    for (name, content) in &examples {
        let path = dir.join(name);
        fs::write(&path, content).map_err(|e| format!("Failed to write {name}: {e}"))?;
        println!("    {DIM}Created {name}{RESET}");
    }

    Ok(())
}

// ═══════════════════════════════════════════════════════════════
// Uninstaller
// ═══════════════════════════════════════════════════════════════

fn write_uninstaller(install_dir: &Path) -> Result<(), String> {
    if cfg!(target_os = "windows") {
        write_windows_uninstaller(install_dir)
    } else {
        write_unix_uninstaller(install_dir)
    }
}

fn write_windows_uninstaller(install_dir: &Path) -> Result<(), String> {
    let script = format!(
        r#"@echo off
echo.
echo   KelpyShark Uninstaller
echo   ======================
echo.
echo   This will remove KelpyShark from:
echo     {}
echo.
set /p CONFIRM="  Are you sure? (Y/N) "
if /I not "%CONFIRM%"=="Y" (
    echo   Cancelled.
    pause
    exit /b 0
)

echo.
echo   Removing from PATH...
for /F "tokens=2*" %%A in ('reg query "HKCU\Environment" /v Path 2^>nul') do set "CURRENT_PATH=%%B"
set "REMOVE={}\bin"
call set "NEW_PATH=%%CURRENT_PATH:%REMOVE%;=%%"
call set "NEW_PATH=%%NEW_PATH:;%REMOVE%=%%"
reg add "HKCU\Environment" /v Path /t REG_EXPAND_SZ /d "%NEW_PATH%" /f >nul 2>&1

echo   Deleting files...
rd /s /q "{}"

echo.
echo   KelpyShark has been uninstalled.
echo.
pause
"#,
        install_dir.display(),
        install_dir.display(),
        install_dir.display()
    );

    let path = install_dir.join("uninstall.bat");
    fs::write(&path, script).map_err(|e| format!("Failed to write uninstaller: {e}"))?;
    println!("    {DIM}Created uninstall.bat{RESET}");
    Ok(())
}

fn write_unix_uninstaller(install_dir: &Path) -> Result<(), String> {
    let script = format!(
        r#"#!/usr/bin/env bash
set -euo pipefail

echo ""
echo "  KelpyShark Uninstaller"
echo "  ======================"
echo ""
echo "  This will remove KelpyShark from:"
echo "    {dir}"
echo ""
read -p "  Are you sure? (y/N) " confirm
if [[ "$confirm" != [yY] ]]; then
    echo "  Cancelled."
    exit 0
fi

echo ""
echo "  Removing from PATH..."

SHELL_RC=""
if [ -f "$HOME/.bashrc" ]; then
    SHELL_RC="$HOME/.bashrc"
    sed -i '/\.kelpyshark\/bin/d' "$SHELL_RC" 2>/dev/null || true
fi
if [ -f "$HOME/.zshrc" ]; then
    SHELL_RC="$HOME/.zshrc"
    sed -i '/\.kelpyshark\/bin/d' "$SHELL_RC" 2>/dev/null || true
fi

echo "  Deleting files..."
rm -rf "{dir}"

echo ""
echo "  KelpyShark has been uninstalled."
echo "  Restart your terminal for PATH changes to take effect."
echo ""
"#,
        dir = install_dir.display()
    );

    let path = install_dir.join("uninstall.sh");
    fs::write(&path, script).map_err(|e| format!("Failed to write uninstaller: {e}"))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&path)
            .map_err(|e| format!("Cannot read permissions: {e}"))?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&path, perms)
            .map_err(|e| format!("Cannot set permissions: {e}"))?;
    }

    println!("    {DIM}Created uninstall.sh{RESET}");
    Ok(())
}

// ═══════════════════════════════════════════════════════════════
// PATH management
// ═══════════════════════════════════════════════════════════════

fn add_to_path(bin_dir: &Path) -> bool {
    let bin_str = bin_dir.to_string_lossy().to_string();

    if cfg!(target_os = "windows") {
        add_to_path_windows(&bin_str)
    } else {
        add_to_path_unix(&bin_str)
    }
}

#[cfg(target_os = "windows")]
fn add_to_path_windows(bin_str: &str) -> bool {
    // Read current user PATH from the registry
    let output = std::process::Command::new("reg")
        .args([
            "query",
            "HKCU\\Environment",
            "/v",
            "Path",
        ])
        .output();

    let current_path = match output {
        Ok(o) => {
            let stdout = String::from_utf8_lossy(&o.stdout).to_string();
            // Parse the REG_EXPAND_SZ value from the output
            if let Some(line) = stdout.lines().find(|l| l.contains("Path") || l.contains("PATH")) {
                let parts: Vec<&str> = line.splitn(3, "    ").collect();
                if parts.len() >= 3 {
                    parts[2].trim().to_string()
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        }
        Err(_) => String::new(),
    };

    // Check if already in PATH
    if current_path
        .split(';')
        .any(|p| p.eq_ignore_ascii_case(bin_str))
    {
        return false;
    }

    // Append to user PATH
    let new_path = if current_path.is_empty() {
        bin_str.to_string()
    } else {
        format!("{};{}", current_path, bin_str)
    };

    let result = std::process::Command::new("reg")
        .args([
            "add",
            "HKCU\\Environment",
            "/v",
            "Path",
            "/t",
            "REG_EXPAND_SZ",
            "/d",
            &new_path,
            "/f",
        ])
        .output();

    if result.is_ok() {
        // Broadcast WM_SETTINGCHANGE so Explorer picks up the change
        let _ = std::process::Command::new("cmd")
            .args(["/c", "setx", "KELPYSHARK_HOME", &format!("{}", Path::new(bin_str).parent().unwrap_or(Path::new(bin_str)).display())])
            .output();
    }

    true
}

#[cfg(not(target_os = "windows"))]
fn add_to_path_windows(_bin_str: &str) -> bool {
    false
}

#[cfg(not(target_os = "windows"))]
fn add_to_path_unix(bin_str: &str) -> bool {
    // Check if already in PATH
    if let Ok(path) = env::var("PATH") {
        if path.split(':').any(|p| p == bin_str) {
            return false;
        }
    }

    let export_line = format!("\n# KelpyShark\nexport PATH=\"{}:$PATH\"\n", bin_str);

    // Try .bashrc and .zshrc
    let home = env::var("HOME").unwrap_or_default();
    let mut updated = false;

    for rc_file in &[".bashrc", ".zshrc", ".profile"] {
        let rc_path = PathBuf::from(&home).join(rc_file);
        if rc_path.exists() {
            if let Ok(contents) = fs::read_to_string(&rc_path) {
                if contents.contains(bin_str) {
                    continue;
                }
            }
            if let Ok(mut f) = fs::OpenOptions::new().append(true).open(&rc_path) {
                let _ = f.write_all(export_line.as_bytes());
                updated = true;
            }
        }
    }

    updated
}

#[cfg(target_os = "windows")]
fn add_to_path_unix(_bin_str: &str) -> bool {
    false
}

// ═══════════════════════════════════════════════════════════════
// Verification
// ═══════════════════════════════════════════════════════════════

fn verify_installation(binary_path: &Path) -> Result<(), String> {
    if !binary_path.exists() {
        return Err("Binary not found after extraction!".to_string());
    }

    let metadata = fs::metadata(binary_path)
        .map_err(|e| format!("Cannot read binary metadata: {e}"))?;

    if metadata.len() == 0 {
        return Err("Binary is empty!".to_string());
    }

    // Try running --version
    let output = std::process::Command::new(binary_path)
        .arg("--version")
        .output();

    match output {
        Ok(o) if o.status.success() => {
            let version = String::from_utf8_lossy(&o.stdout);
            println!("    {DIM}Verified: {}{RESET}", version.trim());
        }
        Ok(_) => {
            println!("    {DIM}Binary exists but --version returned non-zero (may be OK){RESET}");
        }
        Err(e) => {
            println!("    {YELLOW}Warning: could not run binary for verification: {e}{RESET}");
        }
    }

    Ok(())
}

// ═══════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_embedded_binary_is_not_empty() {
        assert!(
            KELPY_BINARY.len() > 1000,
            "Embedded binary should be at least 1 KB, got {} bytes",
            KELPY_BINARY.len()
        );
    }

    #[test]
    fn test_embedded_binary_has_valid_header() {
        // Windows EXE starts with "MZ", ELF starts with 0x7f "ELF"
        if cfg!(target_os = "windows") {
            assert_eq!(
                &KELPY_BINARY[0..2],
                b"MZ",
                "Windows binary should start with MZ header"
            );
        } else {
            assert_eq!(
                KELPY_BINARY[0], 0x7f,
                "Unix binary should start with ELF magic byte"
            );
        }
    }

    #[test]
    fn test_default_install_dir_is_valid() {
        let dir = default_install_dir();
        assert!(
            dir.to_string_lossy().contains("kelpyshark"),
            "Default install dir should contain 'kelpyshark'"
        );
    }

    #[test]
    fn test_write_example_files() {
        let tmp = env::temp_dir().join("ks_installer_test_examples");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();

        write_example_files(&tmp).unwrap();

        assert!(tmp.join("hello.ks").exists());
        assert!(tmp.join("fizzbuzz.ks").exists());
        assert!(tmp.join("functions.ks").exists());
        assert!(tmp.join("data_structures.ks").exists());

        let hello = fs::read_to_string(tmp.join("hello.ks")).unwrap();
        assert!(hello.contains("KelpyShark"));

        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_write_uninstaller() {
        let tmp = env::temp_dir().join("ks_installer_test_uninstall");
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();

        write_uninstaller(&tmp).unwrap();

        if cfg!(target_os = "windows") {
            assert!(tmp.join("uninstall.bat").exists());
        } else {
            assert!(tmp.join("uninstall.sh").exists());
        }

        let _ = fs::remove_dir_all(&tmp);
    }
}
