use piccolo::{Callback, CallbackReturn, Closure, Executor, FunctionPrototype, Lua, Value};
use std::fs::File;

struct Data {
  pub number: i32,
}

fn main() -> anyhow::Result<()> {
  let mut lua = Lua::full();

  let file_name = "scripts/entity.lua";
  let file = File::open(file_name);

  let mut data = Data { number: 12 };

  if let Ok(file) = file {
    lua.try_enter(|ctx| {
      ctx.set_global(
        "delta",
        Callback::from_fn(&ctx, move |_, _, mut stack| {
          stack.push_back(Value::Number(12 as f64));
          Ok(CallbackReturn::Return)
        }),
      )?;

      ctx.set_global(
        "rotate",
        Callback::from_fn(&ctx, move |_, _, stack| {
          let n = stack.get(0).to_number();

          if let Some(n) = n {
            data.number = n as i32;
          }

          Ok(CallbackReturn::Return)
        }),
      )?;

      Ok(())
    })?;

    let executor = lua.try_enter(|ctx| {
      let proto = FunctionPrototype::compile(ctx, file_name, file)?;
      let closure = Closure::new(&ctx, proto, Some(ctx.globals()))?;

      let stash = ctx.stash(Executor::start(ctx, closure.into(), ()));
      Ok(stash)
    })?;

    lua.finish(&executor);
  }

  Ok(())
}
