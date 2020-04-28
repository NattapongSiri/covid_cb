# Watson Language Translate training corpus
It has parallel corpus and a force glossary corpus. It's on early stage. The parallel corpus must have at least 5,000 pair in order to do custom model.

# Provided tools
1. validate.py - Used for validate TMX files
1. csv2tmx.py - Used to convert csv files to tmx files.
1. tmx2csv.py - Used to convert tmx files to csv files.

# Example usage
## Validate TMX files
These command will install libraries into your global Python environment.
If this is not desired, you need to have virtualenv or Anaconda.
If your system has both Python2 and Python3 installed, it typical need:
```
pip3 install -r ./requirements.txt
python3 validate.py ./**/*.tmx
```
If your system has only Python3
```
pip install -r ./requirements.txt
python validate.py ./**/*.tmx
```

## Generate CSV files from TMX
These command will install libraries into your global Python environment.
If this is not desired, you need to have virtualenv or Anaconda.
If your system has both Python2 and Python3 installed, it typical need:
```
pip3 install -r ./requirements.txt
python3 tmx2csv.py ./**/*.tmx
```
If your system has only Python3
```
pip install -r ./requirements.txt
python tmx2csv.py ./**/*.tmx
```

## Generate TMX files from CSV
These command will install libraries into your global Python environment.
If this is not desired, you need to have virtualenv or Anaconda.
If your system has both Python2 and Python3 installed, it typical need:
```
pip3 install -r ./requirements.txt
python3 csv2tmx.py -l th -t sentence ./**/*.tmx
```
If your system has only Python3
```
pip install -r ./requirements.txt
python csv2tmx.py -l th -t sentence ./**/*.tmx
```