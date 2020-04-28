#!/usr/bin/env python3
from argparse import ArgumentParser
from glob import glob
from lxml import etree
import requests
from io import StringIO
import sys

tmx_parser = etree.XMLParser(dtd_validation=True, no_network=False)
tmx_dtd_txt = StringIO(requests.get(url=r"https://www.gala-global.org/sites/default/files/uploads/pdfs/tmx14%20%281%29.dtd").text)

tmx_dtd = etree.DTD(file=tmx_dtd_txt, external_id="-//LISA OSCAR:1998//DTD for Translation Memory eXchange//EN")

def validate(file):
    xml_file = etree.parse(file)
    tmx_dtd.validate(xml_file)

if __name__ == "__main__":
    parser = ArgumentParser(description='Validate tmx file')
    parser.add_argument('glob_path', metavar='GLOB_PATH', type=str, nargs='+',
                        help='Glob compatible path to TMX file to check')

    args = parser.parse_args()
    paths = args.glob_path
    for path in paths:
        files = glob(path)
        for file in files:
            try:
                validate(file)
                print("\033[92m{} passed.\033[0m".format(file))
            except Exception as e:
                print("\033[91m{} failed. {}\033[0m".format(file, e), file=sys.stderr)