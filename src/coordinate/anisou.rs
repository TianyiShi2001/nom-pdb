//! The [ANISOU](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#ANISOU) records present the anisotropic temperature factors.
//! # Record Format
//! | COLUMNS | DATA  TYPE   | FIELD    | DEFINITION                       |
//! | ------- | ------------ | -------- | -------------------------------- |
//! | 1 - 6   | Record name  | "ANISOU" |                                  |
//! | 7 - 11  | Integer      | serial   | Atom serial number.              |
//! | 13 - 16 | Atom         | name     | Atom name.                       |
//! | 17      | Character    | altLoc   | Alternate location indicator     |
//! | 18 - 20 | Residue name | resName  | Residue name.                    |
//! | 22      | Character    | chainID  | Chain identifier.                |
//! | 23 - 26 | Integer      | resSeq   | Residue sequence number.         |
//! | 27      | AChar        | iCode    | Insertion code.                  |
//! | 29 - 35 | Integer      | u[0][0]  | U(1,1)                           |
//! | 36 - 42 | Integer      | u[1][1]  | U(2,2)                           |
//! | 43 - 49 | Integer      | u[2][2]  | U(3,3)                           |
//! | 50 - 56 | Integer      | u[0][1]  | U(1,2)                           |
//! | 57 - 63 | Integer      | u[0][2]  | U(1,3)                           |
//! | 64 - 70 | Integer      | u[1][2]  | U(2,3)                           |
//! | 77 - 78 | LString(2)   | element  | Element symbol, right-justified. |
//! | 79 - 80 | LString(2)   | charge   | Charge on the atom.              |
