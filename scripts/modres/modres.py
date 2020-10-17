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


PROCESSED = os.path.join(
    os.path.dirname(__file__), "processed.json")
MODRES = os.path.join(
    os.path.dirname(__file__), "modres.json")
ANB = os.path.join(
    os.path.dirname(__file__), "ambiguous.json")

with open(PROCESSED) as f:
    processed = json.load(f)
with open(MODRES) as f:
    modres = json.load(f)
with open(ANB) as f:
    anb = json.load(f)
PDB_DIR = sys.argv[1] if len(
    sys.argv) >= 2 else "/run/media/tianyi/LaCie/pdb/decompressed"
for fn in [f for f in os.listdir(PDB_DIR) if not (f in processed)]:
    print(fn)
    with open(fn) as f:
        read = [False]
        while line := f.readline():
            if line[:6] == "MODRES":
                resname = line[12:15]
                stdres = line[24:27]
                desc = line[29:].strip()
                if anb.get(resname):
                    if [stdres, desc] not in anb[resname]:
                        anb[resname].append([stdres, desc])
                elif modres.get(resname):
                    if modres[resname][1] != desc:
                        anb[resname] = [modres[resname][:2], [stdres, desc]]
                        del modres[resname]
                    else:
                        modres[resname][2] += 1
                else:
                    modres[resname] = [stdres, desc, 1]
                read[0] = True
            else:
                if read[0]:
                    break
        processed.append(fn)

with open(MODRES, "w") as f:
    json.dump(modres, f)
with open(PROCESSED, "w") as f:
    json.dump(processed, f)
with open(ANB, "w") as f:
    json.dump(anb, f)
