#!/usr/bin/env python

from pathlib import Path
import subprocess

def main():
    dir_root = Path(__file__).parent.parent.parent
    dir_src = dir_root / 'src'

    git_info = subprocess.check_output(['git', 'describe', '--tags', '--always'], cwd=dir_root).decode().strip()

    file_version = dir_src / 'stignore-generator' / 'src' / 'version.rs'
    file_version.write_text(file_version.read_text().replace('dev', git_info))

if __name__ == '__main__':
    main()