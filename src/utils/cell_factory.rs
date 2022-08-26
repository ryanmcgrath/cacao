use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::view::ViewDelegate;

type CellFactoryMap = HashMap<&'static str, Box<dyn Fn() -> Box<dyn Any>>>;

/// A CellFactory is an struct that stores closures that instantiate view types.
///
/// This is a pattern used in certain view types (e.g, `ListView`). This factory exists to enable
/// dynamic view registration and dequeueing. It stores a closure and erases the type to `Any`, and
/// supports querying for that time to get it back.
///
/// It is explicitly designed to panic if it's unable to retrieve a stored item with the specified
/// type, as the views that use this would cease to function if the type can't be retrieved, and
/// it's better to blow up early.
#[derive(Clone)]
pub struct CellFactory(pub Rc<RefCell<CellFactoryMap>>);

impl CellFactory {
    /// Creates and returns a new CellFactory.
    pub fn new() -> Self {
        CellFactory(Rc::new(RefCell::new(HashMap::new())))
    }

    /// Store a closure for the given identifier.
    // @TODO: We might not need to do anything with this being `ViewDelegate`...
    pub fn insert<F, T>(&self, identifier: &'static str, vendor: F)
    where
        F: Fn() -> T + 'static,
        T: ViewDelegate + 'static,
    {
        let mut lock = self.0.borrow_mut();
        lock.insert(
            identifier,
            Box::new(move || {
                let cell = vendor();
                Box::new(cell) as Box<dyn Any>
            }),
        );
    }

    /// Attempts to retrieve the closure, downcasted to the specified type. This will panic if it's
    /// unable to retrieve the closure with the requested type.
    pub fn get<R>(&self, identifier: &'static str) -> Box<R>
    where
        R: ViewDelegate + 'static,
    {
        let lock = self.0.borrow();
        let vendor = match lock.get(identifier) {
            Some(v) => v,
            None => {
                panic!("Unable to dequeue cell for {}: did you forget to register it?", identifier);
            },
        };

        let view = vendor();

        if let Ok(view) = view.downcast::<R>() {
            view
        } else {
            panic!("Asking for cell of type {}, but failed to match the type!", identifier);
        }
    }
}

impl std::fmt::Debug for CellFactory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CellFactory").finish()
    }
}
