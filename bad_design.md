# Bad Designs of the PDB File Format

### Peptide and Nucleic Acid Sequences Both Being Recorded in `SEQRES`

Peptides and nucleic acids are entirely distinct entities, either from a biologist's or a programmer's point of view. How can you store them in the same field without explicit indication of whether it's a peptide or nucleic acid? For polymers composed solely of standard residues it is easy to guess, but the sequences often also contain modified residues! Who knows if `IMA` is a modified amino acid or nucleotide without looking at the `MODRES` records? (see also below)

### `MODRES` Coming after `SEQRES`

If `MODRES` came before `SEQRES`, the previous problem could be solved (although not elegantly). But the big problem is that `MODRES` comes after `SEQRES`! Using a modified residue in `SEQRES` before declaring its meaning in `MODRES` is just like reading an uninitialized variable.

### The `REMARK` Hell

If you want free text/free key-value pairs/free hierarchies, why not using a consistent syntax for all fields?