### Checksum512

#### Overview

Checksum512 is a small EXE written in Rust that creates a SHA1, SHA-256, and SHA-512 hash from either a file or text. I made it as a simple and fast tool for easy file and text hashing.

Just like all of the EXE's I sign them with a self-signed cert to maintain the integreity of releases.

Their are two options, one if just the CLI and the other a GUI.

#### Features

* Creates SHA1, SHA-256, and SHA-512 hashes
* Supports hashing files and text
* Self-signed with a certificate for integrity verification of releases

#### Options

* **CLI (Command-Line Interface)**: A text-based interface for hashing files and text. BRANCH: cli
* **GUI (Graphical User Interface)**: A user-friendly interface for hashing files and text. BRANCH: main

### Example Usage for the CLI

```bash
checksum512 -f path/to/file
checksum -t "Your text here"
```

CLI Response (TEXT INPUT):
```bash
Hashes for text:
SHA-1:   67bb910c677ab0c5ca66f545f51313e78f557d4b
SHA-256: 762ff8b22af1fb7140f5424ccb8c9ac65faf21e95daba4e9a03d9ebea7964f50
SHA-512: 764ee45108abc26029db739e380783b21c16a8c81f4aca0b6bf2707bec78e5b5ea4497f28c609c28a492465baabcd9da08630440aa2a1a982e369724c436a04c
```