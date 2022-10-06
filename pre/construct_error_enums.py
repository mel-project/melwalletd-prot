#!/usr/bin/python3

import re
from typing import List, NamedTuple

from requests import request

open_brace = "{"
close_brace = "}"


class Variant(NamedTuple):
    name: str

    def __str__(self):
        path = "error::"
        name = self.name
        if(self.name.strip() == ""): return ""
        if(self.name[0] == "*"):
            clean_name = self.name[1:]
            path = clean_name.split("::")
            name = path.pop(-1)
            path = "::".join(path)+"::"
        return str(f'''#[error(transparent)]
    {name}(#[from] {path}{name}),
    ''')
class Enum(NamedTuple):
    """a docstring"""
    name: str
    variants: tuple[Variant]


    def __repr__(self):
        variants = "".join(map(str, self.variants))
        return f'''
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum {self.name}{open_brace}
    {variants}
{close_brace}'''





def main():
    code_file = "../src/protocol.rs"
    write_file = "../src/request_errors.rs"
    code = ""

    with open(code_file, 'r') as rust:
        code = rust.read()

    matches = re.findall("ErrorEnum => (.*?);(.*)", code)

    enums = map(
            lambda x: Enum(
                x[0], 
                map(
                    Variant, 
                    x[1].strip().split(" ")
                )
            ),
            matches
        )
    
    with open(write_file, "w+") as request_errors:
        top = """use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::errors;

"""
        request_errors.write(top)
        for enum in enums:
            request_errors.write(str(enum))

if __name__ == "__main__":
    main()
