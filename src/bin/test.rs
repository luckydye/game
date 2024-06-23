use piccolo::{Callback, CallbackReturn, Closure, Executor, FunctionPrototype, Lua};
use std::{fs::File, sync::Mutex};

struct Data {
  pub number: i32,
}

fn main() -> anyhow::Result<()> {
  let mut lua = Lua::full();

  let file_name = "scripts/entity.lua";
  let file = File::open(file_name);

  let data = Data { number: 12 };
  let mdata = Mutex::new(data);

  if let Ok(file) = file {
    lua.try_enter(|ctx| {
      ctx.set_global(
        "rotate",
        Callback::from_fn(&ctx, move |_, _, stack| {
          let n = stack.get(0).to_number();

          if let Some(n) = n {
            mdata.lock().unwrap().number = n as i32;
          }

          Ok(CallbackReturn::Return)
        }),
      )?;

      Ok(())
    })?;

    let executor = lua.try_enter(|ctx| {
      let proto = FunctionPrototype::compile(ctx, file_name, file)?;
      let closure = Closure::new(&ctx, proto, Some(ctx.globals()))?;

      Ok(ctx.stash(Executor::start(ctx, closure.into(), ())))
    })?;

    lua.finish(&executor);
  }

  Ok(())
}
