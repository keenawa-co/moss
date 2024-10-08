// Copyright 2019 Google LLC
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![no_std]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "std")]
extern crate std;

#[doc(hidden)]
pub extern crate alloc;
#[doc(hidden)]
pub extern crate spin;

macro_rules! reverse_apply {
    ($m: ident [] $($reversed:tt)*) => {
        $m!{$($reversed),*}  // base case
    };
    ($m: ident [$first:tt $($rest:tt)*] $($reversed:tt)*) => {
        reverse_apply!{$m [$($rest)*] $first $($reversed)*}
    };
}

/// Imagine macro parameters, but more like those Russian dolls.
///
/// Calls m!(), m!(A), m!(A, B), and m!(A, B, C) for i.e. (m, A, B, C)
/// where m is any macro, for any number of parameters.
macro_rules! smaller_tuples_too {
    ($m: ident, $next: tt) => {
        $m!{}
        $m!{$next}
    };
    ($m: ident, $next: tt, $($rest: tt),*) => {
        smaller_tuples_too!{$m, $($rest),*}
        reverse_apply!{$m [$next $($rest)*]}
    };
}

mod archetype;
mod batch;
mod borrow;
mod bundle;
mod change_tracker;
mod command_buffer;
mod entities;
mod entity_builder;
mod entity_ref;
mod frame;
mod query;
mod query_one;
#[cfg(any(feature = "row-serialize", feature = "column-serialize"))]
pub mod serialize;
mod take;

pub use archetype::{Archetype, ArchetypeColumn, ArchetypeColumnMut, TypeIdMap, TypeInfo};
pub use batch::{BatchIncomplete, BatchWriter, ColumnBatch, ColumnBatchBuilder, ColumnBatchType};
pub use bundle::{
    bundle_satisfies_query, dynamic_bundle_satisfies_query, Bundle, DynamicBundle,
    DynamicBundleClone, MissingComponent,
};
pub use change_tracker::{ChangeTracker, Changes};
pub use command_buffer::CommandBuffer;
pub use entities::{Entity, NoSuchEntity};
pub use entity_builder::{BuiltEntity, BuiltEntityClone, EntityBuilder, EntityBuilderClone};
pub use entity_ref::{ComponentRef, ComponentRefShared, EntityRef, Ref, RefMut};
pub use frame::{
    ArchetypesGeneration, Component, ComponentError, Frame, Iter, QueryOneError, SpawnBatchIter,
    SpawnColumnBatchIter,
};
pub use query::{
    Access, Batch, BatchedIter, Or, PreparedQuery, PreparedQueryBorrow, PreparedQueryIter,
    PreparedView, Query, QueryBorrow, QueryIter, QueryMut, QueryShared, Satisfies, View,
    ViewBorrow, With, Without,
};
pub use query_one::QueryOne;
pub use take::TakenEntity;

// Unstable implementation details needed by the macros
#[doc(hidden)]
pub use bundle::DynamicClone;
#[doc(hidden)]
pub use query::Fetch;

#[cfg(feature = "macros")]
pub use crater_macros::{Bundle, DynamicBundleClone, Query};

fn align(x: usize, alignment: usize) -> usize {
    debug_assert!(alignment.is_power_of_two());
    (x + alignment - 1) & (!alignment + 1)
}
