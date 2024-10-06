use hashbrown::HashMap;
use std::{
    alloc::{alloc, dealloc, handle_alloc_error, realloc, Layout},
    any::{Any, TypeId},
    borrow::Cow,
    cell::UnsafeCell,
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem::{align_of, size_of},
    ptr::{self, NonNull},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct View {
    id: u32,
    ident: String,
}

impl View {
    pub fn new(id: u32, ident: String) -> Self {
        Self { id, ident }
    }
}

/// Unique identifier for a component
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComponentId(u32);

impl ComponentId {
    pub fn new(id: u32) -> Self {
        ComponentId(id)
    }
}

pub struct ComponentMetadata {
    id: ComponentId,
    type_id: TypeId,
    layout: Layout,
    drop_fn: unsafe fn(*mut u8),
}

impl ComponentMetadata {
    fn new<T: 'static>() -> Self {
        unsafe fn drop_ptr<T>(ptr: *mut u8) {
            ptr::drop_in_place(ptr as *mut T);
        }

        let type_id = TypeId::of::<T>();
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        type_id.hash(&mut hasher);

        let hash = hasher.finish();

        Self {
            id: ComponentId::new((hash & 0xFFFF_FFFF) as u32),
            type_id,
            layout: Layout::new::<T>(),
            drop_fn: drop_ptr::<T>,
        }
    }
}

pub struct ComponentSet {
    infos: HashMap<ComponentId, ComponentMetadata>,
    type_to_id: HashMap<TypeId, ComponentId>,
}

impl ComponentSet {
    pub fn new() -> Self {
        Self {
            infos: HashMap::new(),
            type_to_id: HashMap::new(),
        }
    }

    pub fn get_metadata(&self, id: &ComponentId) -> Option<&ComponentMetadata> {
        self.infos.get(id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TableId(u32);

impl TableId {
    pub fn new(id: u32) -> Self {
        TableId(id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TableRow(u32);

impl TableRow {
    pub fn new(index: u32) -> Self {
        TableRow(index)
    }
}

pub struct Column {
    data: NonNull<u8>,
    layout: Layout,
    cap: usize,
    len: usize,
    drop_fn: unsafe fn(*mut u8),
}

impl Column {
    fn new(component_metadata: &ComponentMetadata, capacity: usize) -> Self {
        let layout = component_metadata.layout;
        let size = layout.size() * capacity;
        let align = layout.align();

        let ptr = if size > 0 {
            let layout = Layout::from_size_align(size, align).unwrap();
            unsafe {
                let raw_ptr = alloc(layout);
                if raw_ptr.is_null() {
                    handle_alloc_error(layout);
                }

                NonNull::new_unchecked(raw_ptr)
            }
        } else {
            NonNull::dangling()
        };

        Column {
            data: ptr,
            layout,
            cap: capacity,
            len: 0,
            drop_fn: component_metadata.drop_fn,
        }
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.cap == 0 {
            4 // TODO: use this value as a const
        } else {
            self.cap * 2 // TODO: use this value as a const
        };

        let new_size = self.layout.size() * new_capacity;
        let new_layout = Layout::from_size_align(new_size, self.layout.align()).unwrap();

        let new_ptr = if self.cap == 0 {
            alloc(new_layout)
        } else {
            realloc(
                self.data.as_ptr(),
                Layout::from_size_align(self.layout.size() * self.cap, self.layout.align())
                    .unwrap(),
                new_size,
            )
        };

        if new_ptr.is_null() {
            handle_alloc_error(new_layout);
        }

        self.data = NonNull::new_unchecked(new_ptr);
        self.cap = new_capacity;
    }

    unsafe fn get_ptr(&self, index: usize) -> *mut u8 {
        self.data.as_ptr().add(index * self.layout.size())
    }
}

impl Drop for Column {
    fn drop(&mut self) {
        unsafe {
            for i in 0..self.len {
                let ptr = self.get_ptr(i);
                (self.drop_fn)(ptr);
            }

            if self.cap > 0 {
                let layout =
                    Layout::from_size_align(self.layout.size() * self.cap, self.layout.align())
                        .unwrap();

                dealloc(self.data.as_ptr(), layout);
            }
        }
    }
}

pub struct Table {
    columns: HashMap<ComponentId, Column>,
    views: Vec<View>,                     // OPTIMIZE: consider to use SmallVec
    view_indices: HashMap<String, usize>, // Map from identifier to index in entities vector
    cap: usize,
}

impl Table {
    fn new(components_ids: &[ComponentId], component_set: &ComponentSet, capacity: usize) -> Self {
        let mut columns = HashMap::new();

        for &component_id in components_ids {
            if let Some(metadata) = component_set.get_metadata(&component_id) {
                let column = Column::new(metadata, capacity);
                columns.insert(component_id, column);
            }
        }

        Table {
            columns,
            views: Vec::with_capacity(capacity),
            view_indices: HashMap::new(),
            cap: capacity,
        }
    }

    fn grow(&mut self) {
        self.cap *= 2;
        self.views.reserve(self.cap);

        for column in self.columns.values_mut() {
            unsafe { column.grow() };
        }
    }

    fn add_view(&mut self, view: View) -> TableRow {
        let index = self.views.len();
        if index >= self.cap {
            self.grow();
        }

        self.views.push(view.clone());
        self.view_indices.insert(view.ident.clone(), index);

        TableRow::new(index as u32)
    }
}
