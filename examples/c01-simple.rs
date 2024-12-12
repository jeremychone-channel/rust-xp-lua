use mlua::{Lua, Value};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let lua = Lua::new();

	// -- Load
	let chunk = lua.load(
		r#"
local num = 123		
print("Hello from Lua " .. num)
return num + 1
		"#,
	);

	// -- Exec
	// chunk.exec()?; // no return

	// -- Eval
	let res = chunk.eval::<Value>()?;
	println!("->> {res:?}");

	Ok(())
}
