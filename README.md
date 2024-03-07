# `secure-store`

I often find that I have a bunch of repositories on my machine with `.env` ignored from the repo (rightfully so), but I always want the ability to get rid of the repo and clone it and be ready to go. However, when this happens is I don't have the the `.env` file any more, and have to reassemble it from scratch and hunt down all the variables to get the app working again. What if there was a better way?

* With `secure-store store .env` the file gets saved to `Keychain Access` on mac. 
* With `secure-store write .env` you can write the file back to the repo.

This uses the first `commit hash` of the repo as a unique identifier for this repo, so there's no chance of retrieving the wrong `.env`.

```
Stores, retrieves, and manages files with the macOS Keychain

Usage: secure-store <COMMAND>

Commands:
  store       Adds the specified file in the macOS Keychain
  cat         Outputs the specified file content to stdout from the macOS Keychain
  delete      Delete a file from the macOS Keychain.
  write       Writes the specified file content to disk from the macOS Keychain
  list        Lists all the files for this repo that are in macOS Keychain.
  delete-all  Deletes all files for this repo from the macOS Keychain.
  write-all   Writes all files for this repo from the macOS Keychain.
  hash        Prints this repository's hash.
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
