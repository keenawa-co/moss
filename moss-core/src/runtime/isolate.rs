use mlua::{FromLua, IntoLua, Lua, UserData};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct OutputData {
    pub field1: String,
    pub field2: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct InputData {
    pub some: String,
}

impl UserData for InputData {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("some", |_, this| Ok(this.some.clone()));
        fields.add_field_method_set("some", |_, this, val| {
            this.some = val;
            Ok(())
        });
    }
}

impl UserData for OutputData {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("field1", |_, this| Ok(this.field1.clone()));
        fields.add_field_method_get("field2", |_, this| Ok(this.field2));
    }
}

impl<'lua> FromLua<'lua> for OutputData {
    fn from_lua(lua_value: mlua::Value<'lua>, lua: &'lua Lua) -> mlua::Result<Self> {
        let table = lua_value.as_table().unwrap();
        Ok(OutputData {
            field1: table.get("field1")?,
            field2: table.get("field2")?,
        })
    }
}

pub fn lua_call() -> mlua::Result<()> {
    let lua = Lua::new();
    let scope = lua.globals();

    lua.load(include_str!("../../../testdata/hello.lua"))
        .exec()?;

    let input_data = InputData {
        some: "From Rust to Lua and back".to_string(),
    };

    scope.set("input", input_data.clone())?;

    let hello: mlua::Function = lua.globals().get("Hello")?;

    let greeting: OutputData =
        hello.call(mlua::Value::UserData(lua.create_userdata(input_data)?))?;

    // let greeting = OutputData {
    //     field1: greeting.get("field1")?,
    //     field2: greeting.get("field2")?,
    // };

    println!("{}, {}", greeting.field1, greeting.field2);

    Ok(())
}
