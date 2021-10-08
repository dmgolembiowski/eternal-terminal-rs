#!/usr/bin/env python3

"""
codegen.py

This script is responsible for scraping all class names
and function definitions on those types so that they may
be cranelifted into this repository as Rust source.

It's generally a good idea to run `git submodule update --recursive`
beforehand so that the signatures and after-effects match the latest
source.
"""
import gc
import atexit
import os
from collections import namedtuple
from pathlib import Path
import re

# idrc if we GC or not; I just want memory
gc.set_threshold(0)
atexit.register(os._exit, 0)

CPP_INHERIT = []

hf        = namedtuple("HeaderFile", ["cls", "path"])
hpp_files = [os.path.join(root, filename)
          for root, dirnames, filenames in os.walk("./src/EternalTerminal/src")
          for filename in filenames if filename.endswith('.hpp')]

#class_ty  = re.compile('^class (?P<classname>[^\s]*)(.*)$')
class_ty  = re.compile(r'''class\s(?P<class_ty>[^\s]+)''')
#    for m in re.finditer(r'''class\s(?P<phone>[^\s]+)''', this, re.VERBOSE):


for headerfile in hpp_files:
    print(f"Extracting classnames from {headerfile}...")
    with open(headerfile, "r") as source:
        content = source.read()
        for inheriting in re.finditer(r'''class\s(?P<class_ty>[^\s]+)''', content, re.VERBOSE):
            try:
                mem = inheriting.groupdict()
                CPP_INHERIT.append(hf(cls=mem.get("class_ty"), path=str(headerfile)))
                del mem
            except AttributeError:
                pass

def template(cls, path):
    if cls:
        return f'''#[inherit_from({cls})]
#[derive(Debug)]
struct Rust{cls}'''+'{}'+f'''

#[inherit_from_impl(BaseType, "{path}")]
impl Rust{cls}'''+''' {
    fn new() -> Self {
        Self {
            _base: BaseType { vtable_: RustType::VTABLE_ as _, value: 3 }
        }
    }
    // This is a placeholder slot for functions
    #[overridden] fn x(&self) -> i32 {
        99
    }
}
'''#replace("CLASS", cls).replace("PATH", path)
    else:
        return "\n"

with open("src/libbindet.rs", "a+") as f:
    f.write("use cpp_inherit::*;\n")
    for inherited in CPP_INHERIT:
        f.write(template(inherited.cls, inherited.path))


