#!/usr/bin/python3

from copy import deepcopy
import re
from typing import List, NamedTuple
import dataclasses as dc

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

@dc.dataclass
class Enum:
    """a docstring"""
    name: str
    variants: tuple[Variant]

    def __str__(self):
        variants = "".join(map(str, self.variants))
        return f'''
#[derive(Error, Debug, Serialize, Deserialize)]
pub enum {self.name}{open_brace}
    {variants}
{close_brace}'''


def _print(args):
    for a in args:
        print(a)
    print()

def filter_variant(v):
    print(f'`{v}`')
    return v
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
                list(map(
                    Variant, 
                    filter(
                        lambda x: x,
                        x[1].strip().split(" ")
                    )
                ))
            ),
            matches
        )
    no_variants = filter(lambda enum: not len(enum.variants), deepcopy(enums))
    with_variants = filter(lambda enum: len(enum.variants), enums)


    # check for duplicates
    for enum in deepcopy(with_variants):
        no_duplicates_with_variants = set()
        duplicates = []
        if enum.name in no_duplicates_with_variants:
            duplicates.append(enum.name)
        no_duplicates_with_variants.add(enum.name)
        if len(duplicates) > 0:
            raise f'DUPLICATES: {duplicates}; remove the variant definition or change enum names'
    
    
    enums_no_variants = dict(map(lambda enum: (enum.name, enum), no_variants))

    enums = enums_no_variants
    for enum_with_variants in with_variants:
        enums[enum_with_variants.name] = enum_with_variants
    
    enums = enums.values()


    with open(write_file, "w+") as request_errors:
        top = """use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::error;

"""
        request_errors.write(top)
        for enum in enums:
            request_errors.write(str(enum))

if __name__ == "__main__":
    print("Start###########################################")
    main()
    print("DONE############################################")
