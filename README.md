# rasciigraph
Tiny Rust library to draw pretty line graphs using ascii characters.

# Usage
Add this to your Cargo.toml
``` toml
[dependencies]
rasciigraph = "0.1.1"
```
Add this line of code to top of your source code
``` rust
extern crate rasciigraph;
```

If you prefer to use Rust 2018 edition you may prefer to add this line to your source code
``` rust
use rasciigraph::{plot, Config}
```

# Examples
This code 
``` rust
extern crate rasciigraph;

use rasciigraph::{plot, Config};

fn main() {
    println!(
        "{}",
        plot(
            vec![
                0.0, 0.0, 0.0, 0.0, 1.5, 0.0, 0.0, -0.5, 9.0, -3.0, 0.0, 0.0, 1.0, 2.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.5, 0.0, 0.0, -0.5, 8.0, -3.0, 0.0, 0.0, 1.0,
                2.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.5, 0.0, 0.0, -0.5, 10.0, -3.0,
                0.0, 0.0, 1.0, 2.0, 1.0, 0.0, 0.0, 0.0, 0.0
            ],
            Config::default()
                .with_offset(10)
                .with_height(10)
                .with_caption("I'm a doctor, not an engineer.".to_string())
        )
    );
}

```
Produces an output like this
```
  10.00   ┤                                             ╭╮          
  8.70    ┤       ╭╮                                    ││          
  7.40    ┼       ││                 ╭╮                 ││          
  6.10    ┤       ││                 ││                 ││          
  4.80    ┤       ││                 ││                 ││          
  3.50    ┤       ││                 ││                 ││          
  2.20    ┤       ││   ╭╮            ││   ╭╮            ││   ╭╮     
  0.90    ┤   ╭╮  ││  ╭╯╰╮       ╭╮  ││  ╭╯╰╮       ╭╮  ││  ╭╯╰╮    
 -0.40    ┼───╯╰──╯│╭─╯  ╰───────╯╰──╯│╭─╯  ╰───────╯╰──╯│╭─╯  ╰─── 
 -1.70    ┤        ││                 ││                 ││         
 -3.00    ┤        ╰╯                 ╰╯                 ╰╯        
             I'm a doctor, not an engineer.
```

# Acknowledgement
This crat is rustlang port of library [asciigraph](https://github.com/guptarohit/asciigraph) written by [@guptarohit](https://github.com/guptarohit).

Above library is also port of library [asciichart](https://github.com/kroitor/asciichart) written by [@kroitor](https://github.com/kroitor).
