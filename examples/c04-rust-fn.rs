use mlua::{IntoLua, Lua, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let lua = Lua::new();

	// -- Utils module
	let utils = lua.create_table()?;
	let compute_stuff_fn = lua.create_function(|lua: &Lua, arg_0: i64| {
		let res = arg_0 * 2;
		res.into_lua(lua)
	})?;
	utils.set("compute_stuff", compute_stuff_fn)?;

	// -- globals
	lua.globals().set("utils", utils)?;

	// -- Load
	let chunk = lua.load(
		r#"
local num = 123

local stuff = utils.compute_stuff(1000)		
local sum = 0   -- utils.add_stuff(10000, 30000)

return "" .. num .. " " .. stuff .. " " .. sum
	"#,
	);

	// -- Eval
	let res = chunk.eval::<Value>()?; // with returns

	println!("->> {res:?}");

	Ok(())
}
