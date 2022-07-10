use std::cell::Cell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::ffi::CString;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Instant;

use lazy_static::lazy_static;
use objc::declare::ClassDecl;
use objc::ffi;
use objc::runtime::Class;

lazy_static! {
    static ref CLASSES: ClassMap = ClassMap::new();
}

thread_local! {
    /// A very simple RNG seed that we use in constructing unique subclass names.
    ///
    /// Why are we doing this? Mainly because I just don't want to bring in another
    /// crate for something that can be done like this; we don't need cryptographically
    /// secure generation or anything fancy, as we're just after a unique dangling bit
    /// for class names.
    static RNG_SEED: Cell<u64> = Cell::new({
        let mut hasher = DefaultHasher::new();
        Instant::now().hash(&mut hasher);
        thread::current().id().hash(&mut hasher);
        let hash = hasher.finish();
        (hash << 1) | 1
    });
}

/// Represents an entry in a `ClassMap`. We store an optional superclass_name for debugging
/// purposes; it's an `Option` to make the logic of loading a class type where we don't need to
/// care about the superclass type simpler.
#[derive(Debug)]
struct ClassEntry {
    pub superclass_name: Option<&'static str>,
    pub ptr: usize
}

/// Represents a key in a `ClassMap`.
type ClassKey = (&'static str, Option<&'static str>);

/// A ClassMap is a general cache for our Objective-C class lookup and registration. Rather than
/// constantly calling into the runtime, we store pointers to Class types here after first lookup
/// and/or creation.
///
/// There may be a way to do this without using HashMaps and avoiding the heap, but working and
/// usable beats ideal for now. Open to suggestions.
#[derive(Debug)]
pub(crate) struct ClassMap(RwLock<HashMap<ClassKey, ClassEntry>>);

impl ClassMap {
    /// Returns a new ClassMap.
    pub fn new() -> Self {
        ClassMap(RwLock::new(HashMap::new()))
    }

    /// A publicly accessible load method that just passes through our global singleton.
    pub fn static_load(class_name: &'static str, superclass_name: Option<&'static str>) -> Option<*const Class> {
        CLASSES.load(class_name, superclass_name)
    }

    /// Attempts to load a previously registered class.
    ///
    /// This checks our internal map first, and then calls out to the Objective-C runtime to ensure
    /// we're not missing anything.
    pub fn load(&self, class_name: &'static str, superclass_name: Option<&'static str>) -> Option<*const Class> {
        {
            let reader = self.0.read().unwrap();
            if let Some(entry) = (*reader).get(&(class_name, superclass_name)) {
                let ptr = &entry.ptr;
                return Some(*ptr as *const Class);
            }
        }

        // If we don't have an entry for the class_name in our internal map, we should still check
        // if we can load it from the Objective-C runtime directly. The reason we need to do this
        // is that there's a use-case where someone might have multiple bundles attempting to
        // use or register the same subclass; Rust doesn't see the same pointers unlike the Objective-C
        // runtime, and we can wind up in a situation where we're attempting to register a Class
        // that already exists but we can't see.
        let objc_class_name = CString::new(class_name).unwrap();
        let class = unsafe { ffi::objc_getClass(objc_class_name.as_ptr() as *const _) };

        // This should not happen for our use-cases, but it's conceivable that this could actually
        // be expected, so just return None and let the caller panic if so desired.
        if class.is_null() {
            return None;
        }

        // If we got here, then this class exists in the Objective-C runtime but is not known to
        // us. For consistency's sake, we'll add this to our store and return that.
        {
            let mut writer = self.0.write().unwrap();
            writer.insert((class_name, superclass_name), ClassEntry {
                superclass_name,
                ptr: class as usize
            });
        }

        Some(class.cast())
    }

    /// Store a newly created subclass type.
    pub fn store(&self, class_name: &'static str, superclass_name: Option<&'static str>, class: *const Class) {
        let mut writer = self.0.write().unwrap();

        writer.insert((class_name, superclass_name), ClassEntry {
            superclass_name,
            ptr: class as usize
        });
    }
}

/// Calls through to `load_or_register_class_with_optional_generated_suffix`, specifying that we
/// should append a random suffix to the generated class name. This is important for situations
/// where we may be loading classes from e.g two different bundles and need to avoid collision.
///
/// Some parts of the codebase (e.g, iOS UIApplication registration) may need to know the name
/// ahead of time and are not concerned about potential duplications. These cases should feel free
/// to call through to `load_or_register_class_with_optional_generated_suffix` directly, as they
/// are comparatively rare in nature.
///
/// > In the future, this indirection may be removed and the return type of
/// > `load_or_register_class_with_optional_generated_suffix` will be altered to return the generated
/// > class name - but most cases do not need this and it would be a larger change to orchestrate at
/// > the moment.
#[inline(always)]
pub fn load_or_register_class<F>(superclass_name: &'static str, subclass_name: &'static str, config: F) -> *const Class
where
    F: Fn(&mut ClassDecl) + 'static
{
    load_or_register_class_with_optional_generated_suffix(superclass_name, subclass_name, true, config)
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
pub fn load_or_register_class_with_optional_generated_suffix<F>(
    superclass_name: &'static str,
    subclass_name: &'static str,
    should_append_random_subclass_name_suffix: bool,
    config: F
) -> *const Class
where
    F: Fn(&mut ClassDecl) + 'static
{
    if let Some(subclass) = CLASSES.load(subclass_name, Some(superclass_name)) {
        return subclass;
    }

    // If we can't find the class anywhere, then we'll attempt to load the superclass and register
    // our new class type.
    if let Some(superclass) = CLASSES.load(superclass_name, None) {
        // When we're generating a new Subclass name, we need to append a random-ish component
        // due to some oddities that can come up in certain scenarios (e.g, various bundler
        // situations appear to have odd rules about subclass name usage/registration, this simply
        // guarantees that we almost always have a unique name to register with the ObjC runtime).
        //
        // For more context, see: https://github.com/ryanmcgrath/cacao/issues/63
        let objc_subclass_name = match should_append_random_subclass_name_suffix {
            true => format!(
                "{}_{}_{}",
                subclass_name,
                superclass_name,
                RNG_SEED.with(|rng| {
                    rng.set(rng.get().wrapping_add(0xa0761d6478bd642f));
                    let s = rng.get();
                    let t = u128::from(s) * (u128::from(s ^ 0xe7037ed1a0b428db));
                    ((t >> 64) as u64) ^ (t as u64)
                })
            ),

            false => format!("{}_{}", subclass_name, superclass_name)
        };

        match ClassDecl::new(&objc_subclass_name, unsafe { &*superclass }) {
            Some(mut decl) => {
                config(&mut decl);

                let class = decl.register();
                CLASSES.store(subclass_name, Some(superclass_name), class);
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
