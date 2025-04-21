#!/usr/bin/env python3
import argparse
import logging
import os
import sys
from glob import glob
from itertools import chain
from typing import Generator, TextIO, Union, NamedTuple, List, Dict

logging.basicConfig(level=logging.DEBUG)

logger = logging.getLogger(__name__)

try:
    import regex as re
except ImportError:
    logger.warning("Couldn't import regex")
    import re

scripts_directory = os.path.dirname(os.path.abspath(__file__))
repo_directory = os.path.dirname(scripts_directory)
sys_directory = os.path.join(repo_directory, 'ueye-sys')
sys_src_directory = os.path.join(sys_directory, 'src')

rust_glob_pattern = os.path.join(sys_src_directory, '**/*.rs')

re_c_func = re.compile(r"(?P<returns>IDSEXP(?:UL|DEP)?)\s+(?P<name>is_\w+)\s*\((?P<args>[^)]*)\)\s*;")
re_rust_func = re.compile(r"fn\s*(?P<name>is_[\w_\d]+)\s*\((?P<args>[^)]+)\)\s*->\s*(?P<returns>[\w\d_]+);")


class FunctionMatch(NamedTuple):
    name: str
    arguments: str
    returns: str
    rust: bool


FunctionDict = Dict[str, FunctionMatch]


def extract_c_fns(fd: Union[str, TextIO]) -> Generator[FunctionMatch, None, None]:
    if isinstance(fd, str):
        fd = open(str(fd), "r")

    for match in re_c_func.finditer(fd.read()):
        yield FunctionMatch(match.group("name"), match.group("args"), match.group("returns"), False)


def extract_rust_fns(fd: Union[str, TextIO]) -> Generator[FunctionMatch, None, None]:
    if isinstance(fd, str):
        fd = open(str(fd), "r")

    for match in re_rust_func.finditer(fd.read()):
        yield FunctionMatch(match.group("name"), match.group("args"), match.group("returns"), True)


def dump_fns_md(d: FunctionDict) -> Generator[str, None, None]:
    for name, fn in d.items():
        if fn.rust:
            check = "x"
            warn = ""
        else:
            check = " "
            warn = "**⚠**"

        yield f"* [{check}] {warn}`{name}` (`{fn.arguments}`) ⇝ `{fn.returns}`\n"


def get_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser()

    parser.add_argument("-u", "--ueye", type=argparse.FileType("r"), default="/opt/ids/ueye/include/ueye.h",
                        help="Path to the ueye.h header")
    parser.add_argument("-d", "--ueye-deprecated", type=argparse.FileType("r"),
                        default="/opt/ids/ueye/include/ueye_deprecated.h", help="Path to the ueye_deprecated.h header")
    parser.add_argument("-o", "--output", type=argparse.FileType("w"),
                        default=os.path.join(sys_directory, "COVERAGE.md"),
                        help="Path to the output file")

    return parser


def parse_args(argv: Union[List[str], None] = None) -> argparse.Namespace:
    return get_parser().parse_args(sys.argv[1:] if argv is None else argv)


def main(argv: Union[List[str], None] = None) -> int:
    args = parse_args(argv)

    fns: FunctionDict = {}

    for c_fn in chain(extract_c_fns(args.ueye), extract_c_fns(args.ueye_deprecated)):
        fns[c_fn.name] = c_fn

    logger.info("Loading Rust files from pattern: %s", rust_glob_pattern)
    for rust_path in glob(rust_glob_pattern, recursive=True):
        logger.debug("Parsing Rust file: %s", rust_path)

        for rust_fn in extract_rust_fns(rust_path):
            name, arguments, returns, _ = fns[rust_fn.name]
            fns[rust_fn.name] = FunctionMatch(name, arguments, returns, True)

    args.output.write("# uEye API Coverage\n## Functions\n")
    for line in dump_fns_md(fns):
        args.output.write(line)

    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
