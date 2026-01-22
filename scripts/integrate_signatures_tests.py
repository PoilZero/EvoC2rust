import argparse
import os
import re
import shutil
from pathlib import Path

from config.global_config import GlobalConfig


def parse_crate_name(cargo_toml_path: Path) -> str:
    if not cargo_toml_path.exists():
        return "my_proj"
    in_package = False
    for line in cargo_toml_path.read_text().splitlines():
        stripped = line.strip()
        if stripped.startswith("[") and stripped.endswith("]"):
            in_package = stripped == "[package]"
            continue
        if in_package:
            match = re.match(r'name\s*=\s*\"([^\"]+)\"', stripped)
            if match:
                return match.group(1)
    return "my_proj"


def update_cargo_toml(cargo_toml_path: Path, crate_name: str) -> None:
    lines = cargo_toml_path.read_text().splitlines()
    out = []
    in_package = False
    in_lib = False
    found_lib = False
    for line in lines:
        stripped = line.strip()
        if stripped.startswith("[") and stripped.endswith("]"):
            in_package = stripped == "[package]"
            in_lib = stripped == "[lib]"
            if in_lib:
                found_lib = True
            out.append(line)
            continue
        if in_package and stripped.startswith("name"):
            out.append(f'name = "{crate_name}"')
            continue
        if in_lib and stripped.startswith("name"):
            out.append(f'name = "{crate_name}"')
            continue
        out.append(line)
    if not found_lib:
        out.append("")
        out.append("[lib]")
        out.append(f'name = "{crate_name}"')
    cargo_toml_path.write_text("\n".join(out) + "\n")


def ensure_interface_modules(lib_rs_path: Path, interface_files: list[Path]) -> None:
    content = lib_rs_path.read_text()
    additions = []
    for path in interface_files:
        module_name = path.stem
        line = f'#[path = "interfaces/{module_name}.rs"] pub mod {module_name};'
        if line not in content:
            additions.append(line)
    if additions:
        content = content.rstrip() + "\n" + "\n".join(additions) + "\n"
        lib_rs_path.write_text(content)


def collect_expected_signatures(interface_files: list[Path]) -> tuple[set[str], set[str]]:
    fn_pattern = re.compile(r"pub\s+fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(")
    struct_pattern = re.compile(r"pub\s+struct\s+([A-Za-z_][A-Za-z0-9_]*)\b")
    expected_fns = set()
    expected_structs = set()
    for path in interface_files:
        text = path.read_text()
        expected_fns.update(fn_pattern.findall(text))
        expected_structs.update(struct_pattern.findall(text))
    return expected_fns, expected_structs


def collect_generated_signatures(project_src_dir: Path) -> tuple[set[str], set[str]]:
    fn_pattern = re.compile(r"pub\s+fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(")
    struct_pattern = re.compile(r"pub\s+struct\s+([A-Za-z_][A-Za-z0-9_]*)\b")
    found_fns = set()
    found_structs = set()
    for path in project_src_dir.rglob("*.rs"):
        parts = path.parts
        if "interfaces" in parts or "bin" in parts:
            continue
        text = path.read_text()
        found_fns.update(fn_pattern.findall(text))
        found_structs.update(struct_pattern.findall(text))
    return found_fns, found_structs


def main() -> None:
    parser = argparse.ArgumentParser(description="Integrate Rust signatures/tests into final project.")
    parser.add_argument("--project_name", required=True)
    parser.add_argument("--rbench_path", required=True)
    parser.add_argument("--signature_report", default="signature_report.json")
    args = parser.parse_args()

    config = GlobalConfig()
    final_project = Path(config.final_project_dir, args.project_name)
    if not final_project.exists():
        raise FileNotFoundError(f"Final project not found: {final_project}")

    rbench_path = Path(args.rbench_path)
    if not rbench_path.exists():
        raise FileNotFoundError(f"RBench path not found: {rbench_path}")

    crate_name = parse_crate_name(rbench_path / "Cargo.toml")
    update_cargo_toml(final_project / "Cargo.toml", crate_name)

    interfaces_src = rbench_path / "src" / "interfaces"
    interface_files = []
    if interfaces_src.exists():
        final_interfaces = final_project / "src" / "interfaces"
        final_interfaces.mkdir(parents=True, exist_ok=True)
        for path in interfaces_src.glob("*.rs"):
            interface_files.append(path)
            shutil.copy2(path, final_interfaces / path.name)

    lib_rs = final_project / "src" / "lib.rs"
    if interface_files and lib_rs.exists():
        ensure_interface_modules(lib_rs, interface_files)

    bin_src = rbench_path / "src" / "bin"
    if bin_src.exists():
        final_bin = final_project / "src" / "bin"
        final_bin.mkdir(parents=True, exist_ok=True)
        for path in bin_src.glob("*.rs"):
            shutil.copy2(path, final_bin / path.name)

    expected_fns, expected_structs = collect_expected_signatures(
        [final_project / "src" / "interfaces" / f.name for f in interface_files]
    )
    found_fns, found_structs = collect_generated_signatures(final_project / "src")
    missing_fns = sorted(expected_fns - found_fns)
    missing_structs = sorted(expected_structs - found_structs)

    report = {
        "crate_name": crate_name,
        "expected_functions": sorted(expected_fns),
        "expected_structs": sorted(expected_structs),
        "found_functions": sorted(found_fns),
        "found_structs": sorted(found_structs),
        "missing_functions": missing_fns,
        "missing_structs": missing_structs,
    }

    report_path = Path(args.signature_report)
    report_path.write_text(
        "{\n"
        + ",\n".join([f'  "{k}": {report[k]!r}'.replace("'", "\"") for k in report])
        + "\n}\n"
    )


if __name__ == "__main__":
    main()
