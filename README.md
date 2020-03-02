# molarmass

A thing that calculates molar mass of a molecule based on its chemical formula.
Made for personal use because I'm lazy.

The atomic weight data was taken from this [website](https://www.qmul.ac.uk/sbcs/iupac/AtWt/)
([archive](https://web.archive.org/save/https://www.qmul.ac.uk/sbcs/iupac/AtWt/)),
and processed using awk.

## Usage
```console
$ molarmass [formula]
```
to print the molar mass of the compound.

```console
$ molarmass NaCl
58.422 g/mol
$ molarmass "(NH4)2PO4"
132.056 g/mol
```
