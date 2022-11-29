To run the example in main.rs use `trunk serve` (https://trunkrs.dev/).

## Usage

Use the `css!` macro to create a CssBlock. CssBlock contains a field for every "default" css property (no media queries) and can be used as the style attribute of a yew component.

```rust
let my_style = css! {
    width: 20;
    height: 20;
};

let my_html = html!{
    <div style={my_style}/>
};
```
This will render as
```html
<div style="width: 20px; height: 20px;"></div>
```
The `css!` macro takes any rust expression on the righthand side so
```rust
let my_var = 10;

let my_style = css! {
    width: 20 + 20;
    height: my_var;
};
```
is also valid. 

`i32` is interpreted as `<value>px` while `f64` is interpreted as `<value>%` for fine controll you can use the `Length` enum (e.g. `Length::Em(30.5)`)

`CssBlock` provides a `update(other)` method that merges the current block with another CssBlock and a `with(other)` method that returns a copy of the current block updated with `other`.
```rust
let my_style = css!{
    width: 20;
    height: 20;
};

let my_wider_style = my_style.with(css!{width: 30;});

```
## Status
- All widths, heights, margins, paddings and colors should work. 
- Color fields support `NamedColor(Name)`
- Shorthand (e.g. `border`) fields work mostly but have some type inference problems
- Initial, Inherit and variables are not yet supported 

### Full list of supported fields
- accent-color
- align-content
- align-items
- align-self
- alignment-baseline
- animation-delay
- background-color
- background-image
- border
- border-block
- border-block-color
- border-block-end
- border-block-end-color
- border-block-end-style
- border-block-end-width
- border-block-start
- border-block-start-color
- border-block-start-style
- border-block-start-width
- border-block-style
- border-block-width
- border-bottom
- border-bottom-color
- border-bottom-left-radius
- border-bottom-right-radius
- border-bottom-style
- border-bottom-width
- border-color
- border-end-end-radius
- border-end-start-radius
- border-image
- border-image-outset
- border-image-width
- border-inline
- border-inline-color
- border-inline-end
- border-inline-end-color
- border-inline-end-style
- border-inline-end-width
- border-inline-start
- border-inline-start-color
- border-inline-start-style
- border-inline-start-width
- border-inline-style
- border-inline-width
- border-left
- border-left-color
- border-left-style
- border-left-width
- border-radius
- border-right
- border-right-color
- border-right-style
- border-right-width
- border-start-end-radius
- border-start-start-radius
- border-style
- border-top
- border-top-color
- border-top-left-radius
- border-top-right-radius
- border-top-style
- border-top-width
- border-width
- bottom
- caret-color
- color
- column-rule-color
- column-rule-width
- column-width
- contain-intrinsic-height
- contain-intrinsic-width
- display
- fill-color
- fill-image
- flood-color
- height
- left
- lighting-color
- line-height
- list-style-image
- margin
- margin-block
- margin-block-end
- margin-block-start
- margin-bottom
- margin-break
- margin-inline
- margin-inline-end
- margin-inline-start
- margin-left
- margin-right
- margin-top
- margin-trim
- marker-knockout-left
- marker-knockout-right
- mask-border-width
- mask-image
- max-height
- max-width
- min-height
- min-width
- nav-left
- nav-right
- outline-color
- outline-width
- overflow-clip-margin
- padding
- padding-block
- padding-block-end
- padding-block-start
- padding-bottom
- padding-inline
- padding-inline-end
- padding-inline-start
- padding-left
- padding-right
- padding-top
- right
- scroll-margin
- scroll-margin-bottom
- scroll-margin-left
- scroll-margin-right
- scroll-margin-top
- scroll-padding-bottom
- scroll-padding-left
- scroll-padding-right
- scroll-padding-top
- scrollbar-color
- scrollbar-width
- shape-margin
- stroke-color
- stroke-image
- stroke-width
- text-decoration-color
- text-emphasis-color
- top
- width