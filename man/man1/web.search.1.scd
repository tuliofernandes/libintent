web.search(1) "" "libintent"

# NAME

web.search - search the web and return a Markdown summary

# SYNOPSIS

*web.search* [--pages _N_] _query_

# DESCRIPTION

*web.search* queries Brave Search for the given _query_, fetches the top
results, and returns a Markdown-formatted summary buffer containing the title,
URL, and extracted article text for each result.

The query can be supplied either as the first positional argument or via the
intent's *data* field (pipe or standard input depending on the host binary).
When both are present, *data* takes precedence.

Results from social networks, media platforms, and encyclopaedias are excluded
by default (see *FILTERED DOMAINS* below).

# OPTIONS

*--pages* _N_
	Return at most _N_ results. Must be a positive integer. Defaults to *5*.

# INPUT

*data* (string, optional)
	The search query. Takes precedence over the positional argument.

*args[0]* (string, optional)
	The search query, used when *data* is absent or empty.

At least one of *data* or *args[0]* must be provided.

# OUTPUT

On success, a JSON object is returned:

```
{
  "status": "Ok",
  "result": {
    "data": "<markdown string>"
  }
}
```

The *data* field contains a Markdown document with one entry per result,
formatted as:

```
N. [Title](URL)

# Page title

## Page subtitle (if present)

Extracted article paragraphs...

---
```

On error:

```
{
  "status": "Error",
  "error": "<message>"
}
```

# FILTERED DOMAINS

The following domains are excluded from results regardless of the query:

- youtube.com, youtu.be
- twitter.com
- facebook.com
- instagram.com
- linkedin.com
- amazon.com
- spotify.com
- apple.com
- wikipedia.org

# ERRORS

*Missing query for web.search intent*
	Neither *data* nor *args[0]* was provided, or both were empty.

*--pages requires a numeric value*
	The *--pages* flag was given without a following argument.

*--pages value must be a positive number, got 'X'*
	The value after *--pages* could not be parsed as a positive integer.

*Failed to execute web.search: ...*
	A network error occurred while contacting Brave Search or fetching a result
	page. The underlying OS or HTTP error is appended.

# EXAMPLES

Search for the top 5 results (default):

```
octopus web.search "rust async runtimes"
```

Limit results to 3:

```
octopus web.search --pages 3 "rust async runtimes"
```

Pass the query via data (host-binary syntax may vary):

```
echo "rust async runtimes" | octopus web.search
```

# SEE ALSO

*fs.list*(1)
