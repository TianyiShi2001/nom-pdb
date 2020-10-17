import sys
import json
import os

# The [MODRES](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#MODRES)
# record provides descriptions of modifications (e.g., chemical or post-translational) to protein and nucleic acid residues. Included are correlations between residue names given
# in a PDB entry and standard residues.
# # Record Format
# | COLUMNS | DATA TYPE    | FIELD    | DEFINITION                               |
# | ------- | ------------ | -------- | ---------------------------------------- |
# | 1 -  6  | Record name  | "MODRES" |                                          |
# | 8 - 11  | IDcode       | idCode   | ID code of this entry.                   |
# | 13 - 15 | Residue name | resName  | Residue name used in this entry.         |   !
# | 17      | Character    | chainID  | Chain identifier.                        |
# | 19 - 22 | Integer      | seqNum   | Sequence number.                         |
# | 23      | AChar        | iCode    | Insertion code.                          |
# | 25 - 27 | Residue name | stdRes   | Standard residue name.                   |   !
# | 30 - 70 | String       | comment  | Description of the residue modification. |   !


processed = json.load(os.path.join(
    os.path.dirname(__file__), "processed.json"))
modres = json.load(os.path.join(
    os.path.dirname(__file__), "modres.json"))
PDB_DIR = sys.argv[1]
for fn in [f for f in os.listdir(PDB_DIR) if not (f in processed)]:
    print(fn)
    with open(fn) as f:
        read = False
        while True:
            line = f.readline()
            if line[:6] == "MODRES":
                resname = line[12:15]
                stdres = line[24:27]
                desc = line[29:].strip()
                if modres["resname"]:
                    assert(modres["resname"][1] == desc)
                else:
                    modres["resname"] = [stdres, desc]
