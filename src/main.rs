use html_node::{html, text, unsafe_text};
use minimad;
use minimad::CompositeStyle::*;
use minimad::Line::*;
use minimad::{Composite, Compound, Line};

// project uses tailwindcss classes

fn main() {
    let md = r#"
# Title

## Subtitle

### Subsubtitle

#### Subsubsubtitle

##### Sub**subsubsubtitle**

###### Subsubsubsubsubtitle

This is a paragraph.

This is a paragraph with
a line break.

This is a **bold** word.

This is a *italic* word.

This is a `code` word.

This is a [link](https://www.google.com).

This is a ![image](https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png).

This is a list:
- item 1
- item 2
- item 3

This is a table:
| Header 1 | Header 2 |
|----------|----------|
| cell 1   | cell 2   |
| cell 3   | cell 4   |

This is a blockquote:
> This is a blockquote.

This is a horizontal rule:
---

This is a footnote[^1].

[^1]: This is a footnote.

This is a math:
$$
\int_0^\infty e^{-x^2} dx = \frac{\sqrt{\pi}}{2}
$$

This is a code block:

```rust
fn main() {
    println!("Hello, world!");
}
```

This is a definition list:
term 1
: definition 1
term 2
: definition 2
term 3
: definition 3
"#;

    let md = md.trim();
    let parsed = minimad::parse_text(md, minimad::Options::default());

    // TODO
    // Lines doit être un vec mutable
    // On doit pouvoir pop les lignes une par une

    // Une table ou un block de code doit être traité différemment, il va pop une ligne tant que le type de la ligne est pareil


    for line in parsed.lines {
        let content = parse_line(line);
        if !content.is_empty() {
            println!("{}", content);
        }
    }
}

fn parse_line(line: Line) -> String {

    match line {
        Normal(text) => parse_normal(text),
        _ => "".into(),
    }
}

fn parse_normal(composite: Composite) -> String {
    println!("{:?}", composite);



    match composite.style {
        Header(level) => parse_header(level, composite.compounds),
        Paragraph => parse_string(composite.compounds),
        _ => "".into(),
    }
}

fn parse_header(level: u8, compounds: Vec<Compound>) -> String {
    let text = compounds
        .iter()
        .map(|c| parse_compound(c))
        .collect::<Vec<String>>();
    let text = text.join("");

    let level = format!("h{}", level);
    let css = match level.as_str() {
        "h1" => "text-4xl font-bold",
        "h2" => "text-3xl font-bold",
        "h3" => "text-2xl font-bold",
        "h4" => "text-xl font-bold",
        "h5" => "text-lg font-bold",
        "h6" => "text-base font-bold",
        _ => "",
    };

    let html = html! {
        <{level} class={css}>
            {unsafe_text!("{text}")}
        </{level}>
    };

    html.to_string()
}

fn parse_string(compounds: Vec<Compound>) -> String {
    let text = compounds
        .iter()
        .map(|c| parse_compound(c))
        .collect::<Vec<String>>();
    let text = text.join("");

    if text.is_empty() {
        return text;
    }

    let html = html! {
        <p>
            {unsafe_text!("{text}")}
        </p>
    };

    html.to_string()
}

fn parse_compound(compound: &Compound) -> String {
    if compound.bold {
        let html = html! {
             <span class="font-bold">
                {unsafe_text!("{compound}")}
            </span>
        };

        html.to_string()
    } else if compound.italic {
        let html = html! {
            <span class="italic">
               {unsafe_text!("{compound}")}
            </span>
        };

        html.to_string()
    } else if compound.code {
        let html = html! {
            <code>
                {unsafe_text!("{compound}")}
            </code>
        };

        html.to_string()
    } else if compound.strikeout {
        format!("<strike>{}</strike>", compound)
    } else {
        format!("{}", compound)
    }
}
