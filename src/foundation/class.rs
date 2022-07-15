use std::collections::HashMap;
use std::ffi::CString;
use std::sync::{Arc, RwLock};

use lazy_static::lazy_static;

use objc::declare::ClassDecl;
use objc::runtime::{objc_getClass, Class};

lazy_static! {
    static ref CLASSES: ClassMap = ClassMap::new();
}

/// A ClassMap is a general cache for our Objective-C class lookup and registration. Rather than
/// constantly calling into the runtime, we store pointers to Class types here after first lookup
/// and/or creation. The general store format is (roughly speaking) as follows:
///
/// ```no_run
/// {
///     "subclass_type": {
///         "superclass_type": *const Class as usize
///     }
/// }
/// ```
///
/// The reasoning behind the double map is that it allows for lookup without allocating a `String`
/// on each hit; allocations are only required when creating a Class to inject, purely for naming
/// and debugging reasons.
///
/// There may be a way to do this without using HashMaps and avoiding the heap, but working and
/// usable beats ideal for now. Open to suggestions.
#[derive(Debug)]
pub(crate) struct ClassMap(RwLock<HashMap<&'static str, HashMap<&'static str, usize>>>);

impl ClassMap {
    /// Returns a new ClassMap.
    pub fn new() -> Self {
        ClassMap(RwLock::new({
            let mut map = HashMap::new();

            // Top-level classes, like `NSView`, we cache here. The reasoning is that if a subclass
            // is being created, we can avoid querying the runtime for the superclass - i.e, many
            // subclasses will have `NSView` as their superclass.
            map.insert("_supers", HashMap::new());

            map
        }))
    }

    /// Attempts to load a previously registered subclass.
    pub fn load_subclass(&self, subclass_name: &'static str, superclass_name: &'static str) -> Option<*const Class> {
        let reader = self.0.read().unwrap();

        if let Some(inner) = (*reader).get(subclass_name) {
            if let Some(class) = inner.get(superclass_name) {
                return Some(*class as *const Class);
            }
        }

        None
    }

    /// Store a newly created subclass type.
    pub fn store_subclass(&self, subclass_name: &'static str, superclass_name: &'static str, class: *const Class) {
        let mut writer = self.0.write().unwrap();

        if let Some(map) = (*writer).get_mut(subclass_name) {
            map.insert(superclass_name, class as usize);
        } else {
            let mut map = HashMap::new();
            map.insert(superclass_name, class as usize);
            (*writer).insert(subclass_name, map);
        }
    }

    /// Attempts to load a Superclass. This first checks for the cached pointer; if not present, it
    /// will load the superclass from the Objective-C runtime and cache it for future lookup. This
    /// assumes that the class is one that should *already* and *always* exist in the runtime, and
    /// by design will panic if it can't load the correct superclass, as that would lead to very
    /// invalid behavior.
    pub fn load_superclass(&self, name: &'static str) -> Option<*const Class> {
        {
            let reader = self.0.read().unwrap();
            if let Some(superclass) = (*reader)["_supers"].get(name) {
                return Some(*superclass as *const Class);
            }
        }

        let objc_superclass_name = CString::new(name).unwrap();
        let superclass = unsafe { objc_getClass(objc_superclass_name.as_ptr() as *const _) };

        // This should not happen, for our use-cases, but it's conceivable that this could actually
        // be expected, so just return None and let the caller panic if so desired.
        if superclass.is_null() {
            return None;
        }

        {
            let mut writer = self.0.write().unwrap();
            if let Some(supers) = (*writer).get_mut("_supers") {
                supers.insert(name, superclass as usize);
            }
        }

        Some(superclass)
    }
}

/// Attempts to load a subclass, given a `superclass_name` and subclass_name. If
/// the subclass cannot be loaded, it's dynamically created and injected into
/// the runtime, and then returned. The returned value can be used for allocating new instances of
/// this class in the Objective-C runtime.
///
/// The `config` block can be used to customize the Class declaration before it's registered with
/// the runtime. This is useful for adding method handlers and ivar storage.
///
/// If the superclass cannot be loaded, this will panic. If the subclass cannot be
/// created, this will panic. In general, this is expected to work, and if it doesn't,
/// the entire framework will not really work.
///
/// There's definitely room to optimize here, but it works for now.
#[inline(always)]
pub fn load_or_register_class<F>(superclass_name: &'static str, subclass_name: &'static str, config: F) -> *const Class
where
    F: Fn(&mut ClassDecl) + 'static
{
    if let Some(subclass) = CLASSES.load_subclass(subclass_name, superclass_name) {
        return subclass;
    }

    if let Some(superclass) = CLASSES.load_superclass(superclass_name) {
        let objc_subclass_name = format!("{}_{}", subclass_name, superclass_name);

        match ClassDecl::new(&objc_subclass_name, unsafe { &*superclass }) {
            Some(mut decl) => {
                config(&mut decl);

                let class = decl.register();
                CLASSES.store_subclass(subclass_name, superclass_name, class);
                return class;
            },

            None => {
                panic!(
                    "Subclass of type {}_{} could not be allocated.",
                    subclass_name, superclass_name
                );
            }
        }
    }

    panic!(
        "Attempted to create subclass for {}, but unable to load superclass of type {}.",
        subclass_name, superclass_name
    );
}
