# find-syntax

An experiment to try and find the ideal syntax highlighter for an arbitrary snippet.

- Given a snippet of code, query the GitHub Markdown API to generate highlighted HTML.
- Determine the complexity and level of color in the generated HTML.
- Report and display the results, showing the "best" syntax highlighter for the snippet.

## Analysis

- Colorization can be counting the number of non-whitespace characters inside an element with a classname starting with `pl-` as a percentage of all non-whitespace characters.
  - Some class identifiers represent white colors, and should be manually blacklisted from being counted.
  - The deepest/closest classname should be used to determine the color.
  - CSS analysis may be required if multiple classnames are used to determine the color. This shouldn't happen though.
- Complexity can be determined by analyzing the average coverage for each color within the colorized text.
    - [Distribution calculation](https://gist.github.com/Xevion/1b5c971e88ac51521c133cc7e04ecdff)

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
