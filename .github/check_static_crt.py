"""Fail if a Windows executable imports the dynamic C runtime."""

import pathlib
import struct
import sys


def imports(path: pathlib.Path) -> list[str]:
    data = path.read_bytes()
    e_lfanew = struct.unpack_from("<I", data, 0x3C)[0]
    if data[e_lfanew : e_lfanew + 4] != b"PE\0\0":
        raise SystemExit(f"{path}: not a PE file")
    coff = e_lfanew + 4
    _machine, nsects, _t, _sym, _nsym, opt_size, _chars = struct.unpack_from(
        "<HHIIIHH", data, coff
    )
    opt = coff + 20
    magic = struct.unpack_from("<H", data, opt)[0]
    if magic == 0x10B:
        data_dir = opt + 96
    elif magic == 0x20B:
        data_dir = opt + 112
    else:
        raise SystemExit(f"{path}: unknown PE magic {magic:#x}")
    import_rva, import_size = struct.unpack_from("<II", data, data_dir + 8)
    if import_rva == 0 or import_size == 0:
        raise SystemExit(f"{path}: missing import table")
    sections = []
    sec = opt + opt_size
    for index in range(nsects):
        off = sec + index * 40
        _name, vsize, va, rawsize, rawptr = struct.unpack_from("<8sIIII", data, off)
        sections.append((va, max(vsize, rawsize), rawptr))

    def rva_off(rva: int) -> int:
        for va, size, rawptr in sections:
            if va <= rva < va + size:
                return rawptr + (rva - va)
        raise SystemExit(f"{path}: RVA {rva:#x} is outside every section")

    names = []
    cursor = rva_off(import_rva)
    while True:
        _ilt, _time, _fwd, name_rva, _ft = struct.unpack_from("<IIIII", data, cursor)
        if name_rva == 0:
            break
        start = rva_off(name_rva)
        end = data.index(b"\0", start)
        names.append(data[start:end].decode("ascii"))
        cursor += 20
    return names


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: check_static_crt.py DIRECTORY", file=sys.stderr)
        return 2
    root = pathlib.Path(sys.argv[1])
    exes = sorted(root.rglob("*.exe"))
    if not exes:
        print(f"no executables under {root}", file=sys.stderr)
        return 1
    banned_names = {"ucrtbase.dll", "msvcrt.dll"}
    banned_prefixes = ("vcruntime", "api-ms-win-crt")
    failed = False
    for exe in exes:
        libs = imports(exe)
        print(f"imports: {exe}: {', '.join(libs)}", flush=True)
        bad = [
            name
            for name in libs
            if name.lower() in banned_names or name.lower().startswith(banned_prefixes)
        ]
        if bad:
            print(
                f"dynamic CRT dependency in {exe}: {', '.join(bad)}",
                file=sys.stderr,
            )
            failed = True
    return 1 if failed else 0


if __name__ == "__main__":
    raise SystemExit(main())
