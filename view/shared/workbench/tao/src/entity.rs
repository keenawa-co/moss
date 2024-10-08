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

    pub fn id(&self) -> ComponentId {
        self.id
    }
}

pub struct ComponentSet {
    metadata: HashMap<ComponentId, ComponentMetadata>,
    type_to_id: HashMap<TypeId, ComponentId>,
}

impl ComponentSet {
    pub fn new() -> Self {
        Self {
            metadata: HashMap::new(),
            type_to_id: HashMap::new(),
        }
    }

    pub fn register<T: 'static>(&mut self) -> ComponentId {
        let type_id = TypeId::of::<T>();
        if let Some(&id) = self.type_to_id.get(&type_id) {
            return id;
        }

        let metadata = ComponentMetadata::new::<T>();
        let id = metadata.id();

        self.metadata.insert(id, metadata);
        self.type_to_id.insert(type_id, id);

        id
    }

    pub fn get_metadata(&self, id: &ComponentId) -> Option<&ComponentMetadata> {
        self.metadata.get(id)
    }

    pub fn get_id<T: 'static>(&self) -> Option<ComponentId> {
        self.type_to_id.get(&TypeId::of::<T>()).cloned()
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

    unsafe fn resize(&mut self, new_capacity: usize) {
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

    fn get_row(&self, ident: &str) -> Option<TableRow> {
        self.view_indices
            .get(ident)
            .map(|&index| TableRow::new(index as u32))
    }
}

// --------------

pub struct Storage {
    components: ComponentSet,
    tables: HashMap<TableId, Table>,

    next_table_id: u32,
    next_view_id: u32,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            components: ComponentSet::new(),
            tables: HashMap::new(),
            next_table_id: 0,
            next_view_id: 0,
        }
    }

    pub fn register_component<T: 'static>(&mut self) -> ComponentId {
        self.components.register::<T>()
    }

    pub fn create_table(&mut self, components_ids: &[ComponentId], capacity: usize) -> TableId {
        let table_id = TableId::new(self.next_table_id);
        self.next_table_id += 1;

        let table = Table::new(components_ids, &self.components, capacity);
        self.tables.insert(table_id, table);

        table_id
    }

    pub fn create_view(&mut self, table_id: TableId, ident: String) -> Option<(View, TableRow)> {
        let table = self.tables.get_mut(&table_id)?;
        let view = View::new(self.next_view_id, ident);

        self.next_view_id += 1;

        let row = table.add_view(view.clone());

        Some((view, row))
    }

    pub fn set_component<T: 'static>(
        &mut self,
        table_id: TableId,
        row: TableRow,
        value: T,
    ) -> Result<(), String> {
        let component_id = self
            .components
            .get_id::<T>()
            .ok_or("Component not registered")?;
        let table = self.tables.get_mut(&table_id).ok_or("Table not found")?;
        let column = table
            .columns
            .get_mut(&component_id)
            .ok_or("Component not found in table")?;

        let index = row.0 as usize;

        unsafe {
            if index >= column.cap {
                column.resize(index + 1);
            }

            let dst = column.get_ptr(index) as *mut T;
            ptr::write(dst, value);
            if index >= column.len {
                column.len = index + 1;
            }
        }

        Ok(())
    }

    fn get_component<T: 'static>(&self, table_id: TableId, row: TableRow) -> Option<&T> {
        let component_id = self.components.get_id::<T>()?;
        let table = self.tables.get(&table_id)?;
        let column = table.columns.get(&component_id)?;

        let index = row.0 as usize;
        if index >= column.len {
            return None;
        }

        unsafe {
            let ptr = column.get_ptr(index) as *const T;
            Some(&*ptr)
        }
    }

    fn get_row(&self, table_id: TableId, ident: &str) -> Option<TableRow> {
        self.tables.get(&table_id)?.get_row(ident)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestTooltipComponent(String);
    struct TestOrderComponent(u32);

    #[test]
    fn simple_test() {
        let mut storages = Storage::new();

        let tooltip_id = storages.register_component::<TestTooltipComponent>();
        let order_id = storages.register_component::<TestOrderComponent>();

        let component_ids = vec![tooltip_id, order_id];
        let table_id = storages.create_table(&component_ids, 10);

        let identifier = "activityBar.launchpad".to_string();
        let (entity, row) = storages.create_view(table_id, identifier.clone()).unwrap();

        storages
            .set_component(table_id, row, TestTooltipComponent("Launchpad".to_string()))
            .unwrap();
        storages
            .set_component(table_id, row, TestOrderComponent(1))
            .unwrap();

        let row = storages.get_row(table_id, &identifier).unwrap();

        if let Some(tooltip) = storages.get_component::<TestTooltipComponent>(table_id, row) {
            println!("Tooltip: {}", tooltip.0);
        }
    }
}
