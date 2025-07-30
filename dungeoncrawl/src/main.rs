/* Suoritukseen saattaa joutua käyttämään komentoa
 * WINIT_UNIX_BACKEND=x11 cargo run
 * https://github.com/amethyst/bracket-lib/issues/362
 */

use bracket_lib::prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello, Bracket Terminal");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawl")
        .build()?;

    main_loop(context, State{})
}
