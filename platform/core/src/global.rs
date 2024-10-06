// use crate::context::Context;

// pub trait Global: 'static {}

// pub trait ReadGlobal {
//     fn global(ctx: &Context) -> &Self;
// }

// impl<T: Global> ReadGlobal for T {
//     fn global(ctx: &Context) -> &Self {
//         ctx.global::<T>()
//     }
// }
