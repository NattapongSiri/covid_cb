#!/usr/bin/env python3
from argparse import ArgumentParser, RawTextHelpFormatter
from lxml import etree
from glob import glob
import csv
import sys
from os import path

def construct_empty_tmx(sourceLang, segtype="sentence", encoding="UTF-8"):
    "Return tmx root element where first child is header and second child is body"
    tmx = etree.Element("tmx")
    tmx.attrib["version"] = "1.4"
    header = etree.SubElement(tmx, "header")
    meta = header.attrib
    meta["creationtool"] = ""
    meta["creationtoolversion"] = ""
    meta["datatype"] = "PlainText"
    meta["segtype"] = segtype
    meta["adminlang"] = "en"
    meta["srclang"] = sourceLang
    meta["targetlang"] = encoding
    etree.SubElement(tmx, "body")

    return tmx

def target_path(source):
    """
    Generate target file name based on original filename.

    If the file has suffix ".csv", it'll be replaced by ".tmx".
    
    If it has any other suffix, it'll have ".tmx" append to it.
    """
    if file.lower().endswith(".csv"):
        return file[:-4] + ".tmx"
    else:
        return file + ".tmx"

def convert(source_path, target_path, source_idx = -1, source_lang = "en", seg_type = "sentence"):
    """
    Convert from source CSV to target TMX in UTF-8 encoding.

    The first line of CSV shall contains ISO-639-1 language code for each column in CSV file.
    It will overwrite target if it's already exist.
    Caller can choose which column will be used as source for generated TMX.
    There's two way to choose, one is through source_idx, another is through source_lang.

    `source_idx` refer to index of column in CSV file.

    `source_lang` will search for specified ISO-639-1 language code from first line of CSV.
    If it cannot find such language code, it'll raise Exception.
    If both parameters are supplied, source_idx will take precedence and ignore source_lang.

    `seg_type` can be either "sentence" or "phrase".
    """
    counter = 0
    # utf-8-sig decode csv with UTF-8 and strip BOM out
    with open(source_path, "r", encoding="utf-8-sig") as fp:
        csv_reader = csv.reader(fp)
        lang_line = csv_reader.__next__()

        if source_idx == -1:
            # Need to seek column number of source
            for i, l in enumerate(lang_line):
                if l == source_lang:
                    print("found at", i)
                    source_idx = i
                    break
        # Can't find such lang id from csv
        if source_idx == -1:
            raise Exception("Unspecified source_idx or specified_lang cannot be found from csv.")
        
        root = construct_empty_tmx(lang_line[source_idx], seg_type)
        
        # print(etree.tostring(root, pretty_print=True))
        for pair in csv_reader:
            counter = counter + 1
            unit = etree.SubElement(root[1], "tu")
            for i, text in enumerate(pair):
                tuv = etree.SubElement(unit, "tuv")
                tuv.attrib["{http://www.w3.org/XML/1998/namespace}lang"] = lang_line[i]
                etree.SubElement(tuv, "seg").text = text

    with open(target_path, encoding="UTF-8", mode="w") as fp:
        # Write header first because etree.tostring with option encoding="unicode" don't allow xml_declaration=True
        # If we use encoding="UTF-8", it'll escape all non US-ASCII String as sequence of bytes
        fp.write('<?xml version="1.0" encoding="UTF-8"?>\n')
        fp.write(etree.tostring(root, pretty_print=True, encoding="unicode"))
    return counter

program_desc = """
Covert csv file(s) to tmx(s).

It has following assumption:
  * The first line must be ISO-639-1 language code
  * All text that have comma must be quoted.

Example:
es, en
"Hola, John", "Hi John"
me gustan las papas, I like potatoes
"""

if __name__ == "__main__":
    parser = ArgumentParser(description=program_desc, formatter_class=RawTextHelpFormatter)
    parser.add_argument('-f', '--force', action="store_true",
                        help='force overwrite output')
    parser.add_argument('-s', '--source', metavar='COL_IDX', type=int, default=-1,
                        help='CSV Column index to be used as source language. If not specified, it will search for "en" from first line of CSV. If both -s and -l are specified, -s will be used.')
    parser.add_argument('-l', '--source-lang', metavar='LANG_ID', type=str, default='en',
                        help='ISO-639-1 source language code. Default is "en". If both -s and -l are specified, -s will be used.')
    parser.add_argument('-t', '--seg-type', metavar='sentence | phrase', type=str, default="sentence",
                        help='Type of this TMX. Supported type is either `sentence` | `phrase`')
    parser.add_argument('glob_path', metavar='GLOB_PATH', type=str, nargs='+',
                        help='Glob compatible path to TMX file to convert')

    args = parser.parse_args()
    paths = args.glob_path
    for p in paths:
        files = glob(p)
        for file in files:
            try:
                target = target_path(file)
                if path.exists(target) and not args.force:
                    raise Exception("{} already exist".format(target))
                pairs = convert(file, target, source_idx=args.source, source_lang=args.source_lang, seg_type=args.seg_type)
                print("\033[92m{} -> {} {} pairs are done.\033[0m".format(file, target, pairs))
            except Exception as e:
                print("\033[91m{} failed.\n{}\033[0m".format(file, e), file=sys.stderr)