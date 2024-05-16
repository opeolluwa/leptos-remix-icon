 # Leptos Remix Icon

- [Description](#description)
- [Getting Started](#getting-started)
  - [Dependencies](#dependencies)
  - [Installing](#installing)
  - [Executing program](#executing-program)
- [Documentation](#documentation)
- [Help](#help)
- [Authors](#authors)
- [Version History](#version-history)
- [License](#license)
- [Acknowledgments](#acknowledgments)

## Description

Integrate Remix Icon into Leptos application

## Getting Started

### Dependencies

- Describe any prerequisites, libraries, OS version, etc., needed before installing program.
- ex. Windows 10

### Installing

- How/where to download your program
- Any modifications needed to be made to files/folders

### Executing program
- Include Remix in your application 
```html

<link
    href="https://cdn.jsdelivr.net/npm/remixicon@4.2.0/fonts/remixicon.css"
    rel="stylesheet"
/>
```

```rust
use rmi::Icon;
use leptos::{view, IntoView, component};

#[component]
pub fb Button(){

  view!{
<button> Star on Github 
  <Icon icon="github-fill" class=""/> 
</button>
  }
}

```

## Documentation

Describe any special instructions that are necessary to install a software package on your computer (if applicable).

## Help

Any advise for common problems or issues.

```
command to run if program contains helper info
```

## Authors

Contributors names and contact info

ex. Dominique Pizzie  
ex. [@DomPizzie](https://twitter.com/dompizzie)

## Version History

- 0.2
  - Various bug fixes and optimizations
  - See [commit change]() or See [release history]()
- 0.1
  - Initial Release

## License

This project is licensed under the [NAME HERE] License - see the LICENSE.md file for details

## Acknowledgments

Inspiration, code snippets, etc.
        