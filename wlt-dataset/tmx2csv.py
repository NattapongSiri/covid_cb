#!/usr/bin/env python3
from argparse import ArgumentParser, RawTextHelpFormatter
from lxml import etree
from glob import glob
import csv
import sys
from os import path

def target_path(source):
    """
    Generate target file name based on original filename.

    If the file has suffix ".tmx", it'll be replaced by ".csv".
    
    If it has any other suffix, it'll have ".csv" append to it.
    """
    if file.lower().endswith(".tmx"):
        return file[:-4] + ".csv"
    else:
        return file + ".csv"

def convert(source_path, target_path):
    """
    Convert TMX file to CSV file.

    It'll overwrite target_path if it is already exist.
    The CSV file will have language code on the first line.
    The language code is determined by first `tmx/body/tu` element.
    """
    parsed_tmx = etree.parse(source_path)
    all_pairs = parsed_tmx.xpath('//tu')
    if len(all_pairs) > 0:
        with open(target_path, "w") as fp: 
            csv_writer = csv.writer(fp)
            lang_pair = all_pairs[0].xpath("tuv/@xml:lang")
            csv_writer.writerow(lang_pair)
            for pair in all_pairs:
                csv_writer.writerow([seg.text for seg in pair.xpath("tuv/seg")])
        return len(all_pairs)
    else:
        raise Exception("{} has zero translation pair".format(file))

program_desc = """
Covert tmx file(s) to csv(s).
It perform based on following assumption:
  * All <tu> are having the same <tuv> order by lang attribute.
  * All <tuv> are having exactly 1 <seg>.
It'll result in undefined behavior if above assumption are broke.

Example of expected `tu` nodes:
    <tu>
        <tuv xml:lang="en"><seg>Hi</seg></tuv>
        <tuv xml:lang="es"><seg>Hola</seg></tuv>
    </tu>
    <tu>
        <tuv xml:lang="en"><seg>Potatoes</seg></tuv>
        <tuv xml:lang="es"><seg>patatas</seg></tuv>
    </tu>

Notice that both <tu> node have "en" language first then "es" as second node.
It is not important what language come first but that order must be consistent for
every <tu>.
The example above will result in csv file with first row being en, es
"""

if __name__ == "__main__":
    parser = ArgumentParser(description=program_desc, formatter_class=RawTextHelpFormatter)
    parser.add_argument('-f', '--force', action="store_true",
                        help='force overwrite output')
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
                pairs = convert(file, target)
                print("\033[92m{} -> {} {} pairs are done.\033[0m".format(file, target, pairs))
            except Exception as e:
                print("\033[91m{} failed.\n{}\033[0m".format(file, e), file=sys.stderr)