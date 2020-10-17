# nom-pdb
![CI](https://github.com/TianyiShi2001/nom-pdb/workflows/Continuous%20integration/badge.svg)

PDB parser implemented in Rust using nom.

# Example (Last Updated 2020-10-17)

```rust
cargo run --example 4f7i
```

```
Pdb { 
    header: Header { 
        classification: "OXIDOREDUCTASE", 
        deposition_date: 2012-05-16, 
        id_code: "4F7I" 
    }, 
    title: "STRUCTURE OF ISOPROPYLMALATE DEHYDROGENASE FROM THERMUS THERMOPHILUS IN COMPLEX WITH IPM, MN AND NADH", 
    authors: ["A.PALLO", "E.GRACZER", "P.ZAVODSZKY", "M.S.WEISS", "M.VAS"], 
    cryst1: Cryst1 { a: 148.38, b: 50.72, c: 178.24, alpha: 90.0, beta: 93.09, gamma: 90.0, lattice_type: SideCentered, space_group: SpaceGroup(GroupAxis(1, 1), None, None), z: 16 }, atoms: [
        Atom { id: 1, id1: ' ', residue: Ser, chain: 'A', sequence_number: 0, insertion_code: ' ', x: -12.138, y: 1.867, z: 20.782, occupancy: 1.0, temperature_factor: 67.46, element: N, charge: 0 }, 
        Atom { id: 2, id1: ' ', residue: Ser, chain: 'A', sequence_number: 0, insertion_code: ' ', x: -11.456, y: 0.553, z: 20.889, occupancy: 1.0, temperature_factor: 64.07, element: C, charge: 0 }, 
        ...
        ......
        Atom { id: 10592, id1: ' ', residue: Ala, chain: 'D', sequence_number: 348, insertion_code: ' ', x: -18.613, y: -18.963, z: 60.665, occupancy: 1.0, temperature_factor: 90.85, element: C, charge: 0 }
    ] 
}
```

# Notes

# References

- http://www.wwpdb.org/documentation/file-format-content/format33/v3.3.html
- https://proteopedia.org/wiki/index.php/Non-Standard_Residues#cite_note-pdb22-0

# Status
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
    - [ ] Auth
    - [ ] Titl
    - [ ] Edit
    - [ ] Ref
    - [ ] Publ
    - [ ] Refn
    - [ ] Pmid
    - [ ] Doi
- [ ] [Remarks](http://www.wwpdb.org/documentation/file-format-content/format33/remarks.html)
### Primary Structure Section
- [ ] [Dbref](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#DBREF)
- [ ] [Dbref1](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#DBREF1)
- [ ] [Seqadv](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#SEQADV)
- [ ] [Seqres](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#SEQRES)
- [ ] [Modres](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#MODRES)
### Heterogen Section
- [ ] [Het](http://www.wwpdb.org/documentation/file-format-content/format33/sect4.html#HET)
- [ ] [Formul](http://www.wwpdb.org/documentation/file-format-content/format33/sect4.html#FORMUL)
- [ ] [Hetnam](http://www.wwpdb.org/documentation/file-format-content/format33/sect4.html#HETNAM)
- [ ] [Hetsyn](http://www.wwpdb.org/documentation/file-format-content/format33/sect4.html#HETSYN)
### Secondary Structure Section
- [ ] [Helix](http://www.wwpdb.org/documentation/file-format-content/format33/sect5.html#HELIX)
- [ ] [Sheet](http://www.wwpdb.org/documentation/file-format-content/format33/sect5.html#SHEET)
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
- [ ] [Model](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#MODEL)
- [X] [Atom](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#ATOM)
- [ ] [Anisou](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#ANISOU)
- [ ] [Ter](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#TER)
- [ ] [Hetatm](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#HETATM)
- [ ] [Endmdl](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#ENDMDL)
### Connectivity Section
- [ ] [Conect](http://www.wwpdb.org/documentation/file-format-content/format33/sect10.html#CONECT)
### Bookkeeping Section
- [ ] [Master](http://www.wwpdb.org/documentation/file-format-content/format33/sect11.html#MASTER)
- [ ] [End](http://www.wwpdb.org/documentation/file-format-content/format33/sect11.html#END)


