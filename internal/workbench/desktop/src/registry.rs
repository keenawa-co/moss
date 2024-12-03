use anyhow::Result;
use desktop_models::{
    actions::MenuItem,
    view::{GroupId, TreeViewDescriptor, TreeViewGroup, TreeViewGroupLocation},
};
use hashbrown::HashMap;
use moss_text::ReadOnlyStr;
use parking_lot::RwLock;
use std::sync::Arc;
use std::{any::TypeId, fmt::Debug};

pub struct MenuRegistry {
    menus: HashMap<ReadOnlyStr, Vec<MenuItem>>,
}

impl MenuRegistry {
    pub fn new() -> Self {
        Self {
            menus: HashMap::new(),
        }
    }

    pub fn append_menu_item(&mut self, menu_id: ReadOnlyStr, item: MenuItem) {
        self.menus
            .entry(menu_id.into())
            .or_insert_with(Vec::new)
            .push(item);
    }

    pub fn append_menu_items<I>(&mut self, items: I)
    where
        I: IntoIterator<Item = (ReadOnlyStr, MenuItem)>,
    {
        for (menu_id, item) in items {
            self.append_menu_item(menu_id, item);
        }
    }

    pub fn get_menu_items_by_namespace(&self, namespace: &ReadOnlyStr) -> Option<&Vec<MenuItem>> {
        self.menus.get(namespace)
    }
}

#[derive(Debug)]
pub struct ViewsRegistry {
    groups: HashMap<TreeViewGroupLocation, Vec<TreeViewGroup>>,
    views: HashMap<GroupId, Vec<TreeViewDescriptor>>,
}

impl ViewsRegistry {
    pub fn new() -> Self {
        ViewsRegistry {
            groups: HashMap::new(),
            views: HashMap::new(),
        }
    }

    pub(crate) fn append_view_group(
        &mut self,
        location: TreeViewGroupLocation,
        group: TreeViewGroup,
    ) {
        self.groups
            .entry(location)
            .or_insert_with(Vec::new)
            .push(group);
    }

    pub(crate) fn register_views(
        &mut self,
        id: ReadOnlyStr,
        batch: impl IntoIterator<Item = TreeViewDescriptor>,
    ) {
        self.views.entry(id).or_insert_with(Vec::new).extend(batch);
    }

    pub(crate) fn get_view_descriptors_by_group_id(
        &self,
        id: &ReadOnlyStr,
    ) -> Option<&Vec<TreeViewDescriptor>> {
        self.views.get(id)
    }

    pub(crate) fn get_view_model<T: Send + Sync + Debug + 'static>(
        &self,
        group_id: impl Into<ReadOnlyStr>,
        view_id: String,
    ) -> Option<Arc<T>> {
        self.views
            .get(&group_id.into())?
            .iter()
            .find(|item| item.id == view_id)
            .and_then(|item| Arc::downcast::<T>(Arc::clone(&item.model)).ok())
    }

    pub(crate) fn get_groups_by_location(
        &self,
        location: &TreeViewGroupLocation,
    ) -> Option<&Vec<TreeViewGroup>> {
        self.groups.get(location)
    }
}

use std::marker::PhantomData;
use std::ptr::NonNull;
use std::rc::Rc;

/// Marker structure for type erasure.
struct Abstract(());

/// Represents a saved command with type information erased to `Abstract`.
struct Command {
    /// Pointer to the stored function.
    func_ptr: NonNull<Abstract>,
    /// Function to invoke the stored function.
    invoke_fn: unsafe fn(
        NonNull<Abstract>, // func_ptr
        *mut (),           // args_ptr
        *mut (),           // result_ptr
    ),
    /// Function to release the stored function.
    drop_fn: unsafe fn(NonNull<Abstract>),
    /// Prevents implementation of `Send` and `Sync`.
    _not_send_sync: PhantomData<Rc<()>>,
}

impl Drop for Command {
    fn drop(&mut self) {
        unsafe {
            (self.drop_fn)(self.func_ptr);
        }
    }
}

pub struct CommandRegistry {
    commands: HashMap<u32, Command>,
}

impl CommandRegistry {
    fn new() -> Self {
        CommandRegistry {
            commands: HashMap::new(),
        }
    }

    /// Registers a new command with a unique identifier.
    pub fn register<F, Args, Ret>(&mut self, id: u32, func: F)
    where
        F: Fn(Args) -> Ret + 'static,
        Args: 'static,
        Ret: 'static,
    {
        // Wrap the function in a `Box` and obtain a raw pointer.
        let boxed_func = Box::new(func);
        let func_ptr =
            unsafe { NonNull::new_unchecked(Box::into_raw(boxed_func) as *mut Abstract) };

        // Unsafe function to invoke the stored function.
        unsafe fn invoke_fn<F, Args, Ret>(
            func_ptr: NonNull<Abstract>,
            args_ptr: *mut (),
            result_ptr: *mut (),
        ) where
            F: Fn(Args) -> Ret,
            Args: 'static,
            Ret: 'static,
        {
            let func = &*(func_ptr.as_ptr() as *const F);
            // Restore arguments and free memory.
            let args_box = Box::from_raw(args_ptr as *mut Args);
            let args = *args_box;
            // Invoke the function.
            let result = func(args);
            // Store the result.
            std::ptr::write(result_ptr as *mut Ret, result);
        }

        // Unsafe function to release the stored function.
        unsafe fn drop_fn<F>(func_ptr: NonNull<Abstract>) {
            let _ = Box::from_raw(func_ptr.as_ptr() as *mut F);
        }

        let command = Command {
            func_ptr,
            invoke_fn: invoke_fn::<F, Args, Ret>,
            drop_fn: drop_fn::<F>,
            _not_send_sync: PhantomData,
        };

        self.commands.insert(id, command);
    }

    /// Invokes a registered command with the given identifier and arguments.
    pub fn invoke<Args, Ret>(&self, id: u32, args: Args) -> Result<Ret, &'static str>
    where
        Args: 'static,
        Ret: 'static,
    {
        let command = self.commands.get(&id).ok_or("Command not found")?;

        // Wrap the arguments and obtain a raw pointer.
        let boxed_args = Box::new(args);
        let args_ptr = Box::into_raw(boxed_args) as *mut ();

        // Allocate space for the result.
        let mut result = std::mem::MaybeUninit::<Ret>::uninit();

        // Invoke the function.
        unsafe {
            (command.invoke_fn)(command.func_ptr, args_ptr, result.as_mut_ptr() as *mut ());
        }

        // Restore the result.
        let result = unsafe { result.assume_init() };

        Ok(result)
    }
}

pub struct RegistryManager {
    pub views: Arc<RwLock<ViewsRegistry>>,
    pub menus: Arc<RwLock<MenuRegistry>>,
    pub commands: Arc<RwLock<CommandRegistry>>,
}

impl RegistryManager {
    pub fn new() -> Self {
        let views = Arc::new(RwLock::new(ViewsRegistry::new()));
        let menus = Arc::new(RwLock::new(MenuRegistry::new()));
        let commands = Arc::new(RwLock::new(CommandRegistry::new()));

        Self {
            views,
            menus,
            commands,
        }
    }
}
