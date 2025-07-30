/* Suoritukseen saattaa joutua käyttämään komentoa
 * WINIT_UNIX_BACKEND=x11 cargo run
 * https://github.com/amethyst/bracket-lib/issues/362
 */

mod map;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
}

use prelude::*;

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
