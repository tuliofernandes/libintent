fs.list(1) "" "libintent"

# NAME

fs.list - list files and directories at a path

# SYNOPSIS

*fs.list* [_path_]

# DESCRIPTION

*fs.list* reads the contents of a directory and returns two sorted lists: one
for regular files and one for subdirectories.

The target path is supplied via the intent's *data* field. When *data* is
absent or empty, the current working directory (*.*) is used.

Hidden entries (those beginning with a dot) are included in the output — no
filtering is applied. Symbolic links are classified according to their
metadata: a symlink pointing to a directory is reported under *dirs*, all
others under *files*.

# INPUT

*data* (string, optional)
	Path to the directory to list. Defaults to *.* when omitted.

# OUTPUT

On success, a JSON object is returned:

```
{
  "status": "Ok",
  "result": {
    "data": {
      "files": ["file_a.txt", "file_b.rs"],
      "dirs":  ["src", "tests"]
    }
  }
}
```

Both *files* and *dirs* are sorted in ascending lexicographic order.
Entry names are bare names, not full paths.

On error:

```
{
  "status": "Error",
  "error": "<message>"
}
```

# ERRORS

*Failed to read directory: ...*
	The path does not exist, is not a directory, or the process lacks
	permission to read it. The underlying OS error is appended.

# EXAMPLES

List the current directory:

```
octopus fs.list
```

List a specific path:

```
octopus fs.list /etc/apt
```

Pass the path via data (host-binary syntax may vary):

```
echo "/etc/apt" | octopus fs.list
```

# SEE ALSO

*web.search*(1)
