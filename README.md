# find-syntax

An experiment to try and find the ideal syntax highlighter for an arbitrary snippet.

- Given a snippet of code, query the GitHub Markdown API to generate highlighted HTML.
- Determine the complexity and level of color in the generated HTML.
- Report and display the results, showing the "best" syntax highlighter for the snippet.

## Goals

- Memory caching of API results to prevent unnecessary requests.
- Automatic rate limiting and threading optimizations with Tokio, Governor, and Reqwest.
- GitHub Token & User-Agent compliance.
- Heuristics for determining the "best" syntax highlighter for per-request efficiency and overall speed.

### Future

- Provide a server with a nice user intreface for viewing the results using GitHub's syntax.
  - [Light] & [Dark] mode options.
  - OAuth login to allow users to view their own private repos.
- Ability to select parts of the text that should have no color + some color to fine-tune the results.

[light]: https://github.com/primer/github-syntax-light/blob/master/lib/github-light.css
[dark]: https://github.com/primer/github-syntax-dark/blob/master/lib/github-dark.css
