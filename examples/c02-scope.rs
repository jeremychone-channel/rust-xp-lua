use mlua::{Lua, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let lua = Lua::new();

	// -- Load
	let chunk = lua.load(
		r#"
local num = 123		
print("Hello from Lua " .. num)
return num + value_one
		"#,
	);

	// -- Option 1 - with globals
	// let globals = lua.globals();
	// globals.set("value_one", 111)?;

	// -- Option 2 - with set_environment
	let env = lua.create_table()?;
	env.set("value_one", 111)?;
	let globals = lua.globals();
	for pair in globals.pairs::<Value, Value>() {
		let (key, value) = pair?;
		env.set(key, value)?;
	}

	let chunk = chunk.set_environment(env);

	// -- Eval
	let res = chunk.eval::<Value>()?;
	println!("->> {res:?}");

	Ok(())
}
