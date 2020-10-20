# nom-pdb

![CI](https://github.com/TianyiShi2001/nom-pdb/workflows/Continuous%20integration/badge.svg)
[![crates.io](https://img.shields.io/crates/d/nom-pdb.svg)](https://crates.io/crates/nom-pdb)
[![crates.io](https://img.shields.io/crates/v/nom-pdb.svg)](https://crates.io/crates/nom-pdb)
[![crates.io](https://img.shields.io/crates/l/nom-pdb.svg)](https://crates.io/crates/nom-pdb)
[![docs.rs](https://docs.rs/nom-pdb/badge.svg)](https://docs.rs/nom-pdb)

PDB parser implemented in Rust using nom.

**NOTE: This crate is in early development and the API has not yet been stabilized, so do not use this crate in production. If you have any suggestions, please don't hesitate to open an issue or make a PR!**

## Features

- Parses structural information and a subset of important metadata.
  - Primary structure
  - Secondary structure (sheets and helices)
  - Coordinates and bonding
- Able to deal with non-standard residues (not yet mature)
- JSON serialization powered by serde.

The parsed data is stored in a `Structure`, which is a struct provided by the [`protein-core`](https://github.com/TianyiShi2001/protein-core) crate.

# Example

## Read to JSON

```bash
cargo run --example read 1a8o
```

```json
{
  "header": {
    "classification": "VIRAL PROTEIN",
    "deposition_date": "1998-03-27",
    "id_code": "1A8O"
  },
  "title": "HIV CAPSID C-TERMINAL DOMAIN",
  "authors": [
    "T.R.GAMBLE",
    "S.YOO",
    "F.F.VAJDOS",
    "U.K.VON SCHWEDLER",
    "D.K.WORTHYLAKE",
    "H.WANG",
    "J.P.MCCUTCHEON",
    "W.I.SUNDQUIST",
    "C.P.HILL"
  ],
  "experimental_techniques": [
    "XRayDiffraction"
  ],
  "cryst1": {
    "a": 41.98,
    "b": 41.98,
    "c": 88.92,
    "alpha": 90.0,
    "beta": 90.0,
    "gamma": 90.0,
    "lattice_type": "Primitive",
    "space_group": [
      [
        4,
        3
      ],
      [
        2,
        1
      ],
      [
        2,
        1
      ]
    ],
    "z": 8
  },
  "modres": {
    "MSE": {
      "standard_res": "Met",
      "description": "SELENOMETHIONINE",
      "occurence": [
        [
          "A",
          151
        ],
        [
          "A",
          185
        ],
        [
          "A",
          214
        ],
        [
          "A",
          215
        ]
      ]
    }
  },
  "seqres": [
    [
      "A",
      [
        {
          "Custom": "MSE"
        },
        "Asp",
        "Ile",
        "Arg",
        "Gln",
        "Gly",
        "Pro",
    // snip //
      ]
    ]
  ],
  "models": [
    {
      "atoms":  [
          "id": 1,
          "name": "N",
          "id1": " ",
          "residue": "Ser",
          "chain": "A",
          "sequence_number": 0,
          "insertion_code": " ",
          "x": -12.138,
          "y": 1.867,
          "z": 20.782,
          "occupancy": 1.0,
          "temperature_factor": 67.46,
          "element": "N",
          "charge": 0,
          "hetatom": false
        },
        // snip //
      ]
      "anisou": [
        // snip //
      ],
      "sheets": [
        {
          "id": "A",
          "strands": [
            {
              "start": [
                "A",
                34
              ],
              "end": [
                "A",
                38
              ],
              "sense": "Unknown"
            },
            // snip //
          ]
        },
        // snip //
      ]
      "helices": [
        // snip
      ],
      "connect": [
        // snip //
      ]
    }
  ]
}
```

# Notes

# References

- http://www.wwpdb.org/documentation/file-format-content/format33/v3.3.html
- https://proteopedia.org/wiki/index.php/Non-Standard_Residues#cite_note-pdb22-0

# Roadmap

Note: Priority is and is ought to be placed on parsing structural information instead of metadata, since the latter is more or less disordered free-text and usually not of particular interest to users (even in cases where they are, users can examine the PDB file directly).

### Title Section
- [X] [Header](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#HEADER)
- [ ] [Obslte](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#OBSLTE)
- [X] [Title](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#TITLE)
- [ ] [Splt](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#SPLIT)
- [ ] [Caveat](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#CAVEAT)
- [ ] [Compnd](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#COMPND)
- [ ] [Source](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#SOURCE)
- [ ] [Keywds](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#KEYWDS)
- [X] [Expdta](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#EXPDTA)
- [ ] [Nummdl](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#NUMMDL)
- [ ] [Mdltyp](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#MDLTYP)
- [X] [Author](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#AUTHOR)
- [ ] [Sprsde](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#SPRSDE)
- [ ] [Revdat](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#REVDAT)
- [ ] [Jrnl](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#JRNL)
- [ ] [Remarks](http://www.wwpdb.org/documentation/file-format-content/format33/remarks.html)
  - [ ] [Remarks 3](http://www.wwpdb.org/documentation/file-format-content/format33/remark3.html)
  - [ ] [Remarks 0,1,2,4,5-299](http://www.wwpdb.org/documentation/file-format-content/format33/remarks1.html)
  - [ ] [REMARK 300-999](http://www.wwpdb.org/documentation/file-format-content/format33/remarks2.html)
### Primary Structure Section
- [ ] [Dbref](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#DBREF)
- [ ] [Dbref1](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#DBREF1)
- [ ] [Seqadv](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#SEQADV)
- [X] [Seqres](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#SEQRES)
- [X] [Modres](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#MODRES)
### Heterogen Section
- [ ] [Het](http://www.wwpdb.org/documentation/file-format-content/format33/sect4.html#HET)
- [ ] [Formul](http://www.wwpdb.org/documentation/file-format-content/format33/sect4.html#FORMUL)
- [ ] [Hetnam](http://www.wwpdb.org/documentation/file-format-content/format33/sect4.html#HETNAM)
- [ ] [Hetsyn](http://www.wwpdb.org/documentation/file-format-content/format33/sect4.html#HETSYN)
### Secondary Structure Section
- [X] [Helix](http://www.wwpdb.org/documentation/file-format-content/format33/sect5.html#HELIX)
- [X] [Sheet](http://www.wwpdb.org/documentation/file-format-content/format33/sect5.html#SHEET)
### Connectivity Annotation Section
- [ ] [Ssbond](http://www.wwpdb.org/documentation/file-format-content/format33/sect6.html#SSBOND)
- [ ] [Link](http://www.wwpdb.org/documentation/file-format-content/format33/sect6.html#LINK)
- [ ] [Cispep](http://www.wwpdb.org/documentation/file-format-content/format33/sect6.html#CISPEP)
### Miscellaneous Features Section
- [ ] [Site](http://www.wwpdb.org/documentation/file-format-content/format33/sect7.html#SITE)
### Crystallographic and Coordinate Transformation Section
- [X] [Cryst1](http://www.wwpdb.org/documentation/file-format-content/format33/sect8.html#CRYST1)
- [ ] [MtrixN](http://www.wwpdb.org/documentation/file-format-content/format33/sect8.html#MTRIXn)
- [ ] [OrigxN](http://www.wwpdb.org/documentation/file-format-content/format33/sect8.html#ORIGXn)
- [ ] [ScaleN](http://www.wwpdb.org/documentation/file-format-content/format33/sect8.html#SCALEn)
### Coordinate Section
- [X] [Model](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#MODEL)
- [X] [Atom](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#ATOM)
- [X] [Anisou](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#ANISOU)
- [X] [Ter](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#TER)
- [X] [Hetatm](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#HETATM)
- [X] [Endmdl](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#ENDMDL)
### Connectivity Section
- [X] [Conect](http://www.wwpdb.org/documentation/file-format-content/format33/sect10.html#CONECT)
### Bookkeeping Section
- [ ] [Master](http://www.wwpdb.org/documentation/file-format-content/format33/sect11.html#MASTER)
- [X] [End](http://www.wwpdb.org/documentation/file-format-content/format33/sect11.html#END)



## Sample PDB Files

The files in `assets/` are retrieved from [RSCB's FTP server](https://www.rcsb.org/pages/download/ftp) using the method described in [my blog post](https://tianyishi2001.github.io/ox/bioinformatics/download-and-sync-with-the-entire-pdb-database.html). Here are some features of the selected PDB files stored in this directory:


- 1a8o: a simple X-Ray structure
- 4f7i: Lots of sheets
- 7znf: solution NMR; lots of models
- 3l1p: complex with DNA



