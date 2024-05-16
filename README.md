 # Leptos Remix Icon
 Integrate Open Source [Remix Icon](https://remixicon.com/) into [Leptos](https://leptos.dev) Application




## Getting Started 
Include Remix CDN in your application's `index.html`

```html

<link
    href="https://cdn.jsdelivr.net/npm/remixicon@4.2.0/fonts/remixicon.css"
    rel="stylesheet"
/>
```

Then in your Leptos Components
```rust
use leptos_remix_icon::Icon;
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



## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details

## Acknowledgments
[Remix Icon Team](https://github.com/Remix-Design/RemixIcon)
        