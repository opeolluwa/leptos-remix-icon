 # Leptos Remix Icon

 Integrate Open Source [Remix Icon](https://remixicon.com/) into [Leptos](https://leptos.dev) Application


## Getting Started 

Include Remix CDN in your application's `index.html`

```html
<!--index.html-->
<head>
  ...
<link
    href="https://cdn.jsdelivr.net/npm/remixicon@4.2.0/fonts/remixicon.css"
    rel="stylesheet"
/>

</head>
```

Then in your Leptos Components

```rust
use leptos_remix_icon::Icon;
use leptos::{view, IntoView, component};

#[component]
pub fb Button(){

  view!{
<button> Star on Github 
  <Icon icon="github-fill" class="text-gray-500"/> 

  // with size 
    <Icon icon="github-fill" size="xxs" /> 
</button>
  }
}

```


## Development 

To use Remix Icon locally

1. Download the CSS file from <https://cdn.jsdelivr.net/npm/remixicon@2.5.0/fonts/remixicon.css> 

2. Add the downloaded file `remix.css` to `public` folder 

3. Add the following line to your `index.html`

```html
<!--index.html-->
<head>
  ...
<link data-trunk rel="copy-dir" href="./public/" />

<head/>
```

see the [CDN](./examples/tailwind_csr_cdn/) and [local assets](./examples/tailwind_csr_local/) examples 

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details

## Acknowledgments

[Remix Icon Team](https://github.com/Remix-Design/RemixIcon)
        