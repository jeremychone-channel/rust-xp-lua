use mlua::{Function, IntoLua, Lua, MultiValue, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let lua = Lua::new();

	// -- Load
	let chunk = lua.load(
		r#"
function do_stuff(name)
	name = name or "Super Lua"
	local num = 123		
	print("Hello from " .. name .. " " .. num)
	return num + value_one
end
return do_stuff
		"#,
	);

	let lua_fn: Function = chunk.eval()?;

	// -- with set_environment
	let env = lua.create_table()?;
	env.set("value_one", 111)?;

	let globals = lua.globals();
	for pair in globals.pairs::<Value, Value>() {
		let (key, value) = pair?;
		env.set(key, value)?;
	}
	lua_fn.set_environment(env);

	for i in 0..3 {
		// -- Eval
		let mut args = MultiValue::new();
		args.push_front("Rust".into_lua(&lua)?);
		let res = lua_fn.call::<Value>(args)?;
		println!("->> {res:?}");
	}

	Ok(())
}
