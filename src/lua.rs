use bevy::prelude::*;
use piccolo::{Callback, CallbackReturn, Closure, Executor, FunctionPrototype, Lua, Value};
use std::{
  fs::File,
  sync::{Arc, Mutex},
  time::{SystemTime, UNIX_EPOCH},
};

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Script {
  pub file: String,
}

pub fn init_lua(
  mut commands: Commands,
  mut query: Query<(Entity, &mut Transform, &Script)>,
  time: Res<Time>,
) {
  let t = time.delta().as_millis().clone();

  for (entity, mut transform, script) in &mut query {
    let mut lua = Lua::full();

    let file_name = script.file.as_str();
    let file = File::open(file_name);

    // TODO: how to not arc mutex?
    let _transform = Arc::new(Mutex::new(transform.clone()));

    if let Ok(file) = file {
      lua
        .try_enter(|ctx| {
          let transform = _transform.clone();
          let translate = _transform.clone();

          ctx.set_global(
            "time",
            Callback::from_fn(&ctx, move |_, _, mut stack| {
              let start = SystemTime::now();
              let t = start
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");

              stack.push_back(Value::Number(t.as_millis() as f64));
              Ok(CallbackReturn::Return)
            }),
          )?;

          ctx.set_global(
            "delta",
            Callback::from_fn(&ctx, move |_, _, mut stack| {
              stack.push_back(Value::Number(t as f64));
              Ok(CallbackReturn::Return)
            }),
          )?;

          ctx.set_global(
            "rotate",
            Callback::from_fn(&ctx, move |_, _, stack| {
              let n = stack.get(0).to_number();

              if let Some(n) = n {
                transform.lock().unwrap().rotate_y(n as f32);
              }

              Ok(CallbackReturn::Return)
            }),
          )?;

          ctx.set_global(
            "move",
            Callback::from_fn(&ctx, move |_, _, stack| {
              let x = stack.get(0).to_number();
              let y = stack.get(1).to_number();

              if let Some((x, y)) = x.zip(y) {
                translate.lock().unwrap().translation.x += x as f32;
                translate.lock().unwrap().translation.y += y as f32;
              }

              Ok(CallbackReturn::Return)
            }),
          )?;

          Ok(())
        })
        .ok();

      let executor = lua
        .try_enter(|ctx| {
          let proto = FunctionPrototype::compile(ctx, file_name, file)?;
          let closure = Closure::new(&ctx, proto, Some(ctx.globals()))?;

          let stash = ctx.stash(Executor::start(ctx, closure.into(), ()));
          Ok(stash)
        })
        .ok();

      if let Some(executor) = executor {
        lua.finish(&executor);
      }

      transform.rotation = _transform.lock().unwrap().rotation.clone();
      transform.translation = _transform.lock().unwrap().translation.clone();
    }
  }
}
