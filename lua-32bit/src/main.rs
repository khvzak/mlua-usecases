use mlua::prelude::*;

fn main() -> LuaResult<()> {
    let lua = Lua::new();
    lua.load(r#"
        print("hello")
    "#).exec()
}
