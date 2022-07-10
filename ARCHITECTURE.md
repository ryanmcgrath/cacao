# Cacao Architecture
Cacao is a library to interface with AppKit (macOS) or UIKit (iOS/iPadOS/tvOS). It uses the Objective-C runtime to
handle calling into these frameworks.

Said frameworks typically use an Object Oriented style of programming (subclasses, etc), which can be tricky to
handle with the way that Rust works with regards to ownership. Thankfully, AppKit & UIKit often also use a
delegate pattern - objects registered to receive callbacks. With some creative assumptions, we can get somewhat close
to expected conventions.

This document outlines some of the thinking surrounding the architectural patterns used in this framework. Consider it
to be a somewhat living document - things may change or bend as far as rules go, but hopefully this guide makes looking
through the framework easier.

In general, a tl;dr:

**macOS architecture**
```
App -> Window(s) -> Core Controls
```

**iOS architecture**
```
App -> Window(s) -> UIScene(s) -> Core Controls
```

## Control Setup
A typical control in Cacao has 2-3 main pieces:

- The core control, which can take an optional delegate. More on this below.
- iOS/macOS bridges, which inject a subclass into the Objective-C runtime that forwards methods/callbacks/etc to the Rust
  side of things.
- Extra delegates, which are similar to the above bridges (some classes need this to avoid issues).

## Core Control Contract
A core control is just the Rust interface. It should adhere to the following "contract".

### It should always expose an `objc` field, which holds the underlying Objective-C object.
This is important, as the underlying frameworks can differ in how they handle things, and they get frequent-ish updates
each year. There should _always_ be an escape hatch to make life easier for the end-user.

### Controls should always expose the underlying Layer.
This is technically not "correct" on macOS but I don't care. We explicitly expect there to be a layer on controls, and modern
macOS wants things to be layer backed anyway.

For example, the `View<T>` type has the following field:

### Controls should always expose AutoLayout Anchors.
AutoLayout is the preferred layout engine for Apple's frameworks. Users who need frame-based layouts get them for free as
long as the control also implements the `Layout` trait.

### Interior Mutability makes life easier.
I anticipate this being somewhat divisive, maybe. Not sure. Point is, Rust's model is already hard enough to reason about with
UI frameworks - we can try to ease this with interior mutability on controls.

`utils::properties::ObjcProperty` is a handy wrapper for this, which should ideally be used - it will handle retain counts while
simultaneously making the borrow model feel "correct" on the Rust side.

## Control Example
Let's walk through the `View<T>` type to better understand this architecture.

### Core Control
Since this is our Rust type, we can mostly jump right in. Let's start with the struct definition:

``` rust
#[derive(Debug)]
pub struct View<T = ()> {
    /// An internal flag for whether an instance of a View<T> is a handle. Typically, there's only
    /// one instance that should have this set to `false` - if that one drops, we need to know to
    /// do some extra cleanup.
    pub is_handle: bool,

    /// A pointer to the Objective-C runtime view controller.
    pub objc: ObjcProperty,

    /// References the underlying layer. This is consistent across macOS, iOS and tvOS - on macOS
    /// we explicitly opt in to layer backed views.
    pub layer: Layer,

    /// A pointer to the delegate for this view.
    pub delegate: Option<Box<T>>,

    /// A pointer to the Objective-C runtime top layout constraint.
    pub top: LayoutAnchorY,

    /// A pointer to the Objective-C runtime leading layout constraint.
    pub leading: LayoutAnchorX,

    /// A pointer to the Objective-C runtime left layout constraint.
    pub left: LayoutAnchorX,

    /// A pointer to the Objective-C runtime trailing layout constraint.
    pub trailing: LayoutAnchorX,

    /// A pointer to the Objective-C runtime right layout constraint.
    pub right: LayoutAnchorX,

    /// A pointer to the Objective-C runtime bottom layout constraint.
    pub bottom: LayoutAnchorY,

    /// A pointer to the Objective-C runtime width layout constraint.
    pub width: LayoutAnchorDimension,

    /// A pointer to the Objective-C runtime height layout constraint.
    pub height: LayoutAnchorDimension,

    /// A pointer to the Objective-C runtime center X layout constraint.
    pub center_x: LayoutAnchorX,

    /// A pointer to the Objective-C runtime center Y layout constraint.
    pub center_y: LayoutAnchorY
}
```

A few things to note here!

#### `T` is optional.
We want a user to be able to just slap a `View` onto the screen if they want - they're essential building blocks, after all. We default
this to `()` and provide a designated initializer (see below) for cases where you _want_ a delegate set.

#### `is_handle`
We want to be able to run cleanup on `Drop` of the Rust struct, because we want Rust programmers to be able to think in their assumed lifecycle,
not Objective-C's. We need to be able to clone this into a possible delegate to enable customizing as if it were a class, though; enter `is_handle`.

Essentially, `is_handle` should only be false for the "originating" `View`. Clones should always have `is_handle` set to true; this guard is checked
on Drop, and if it's false, we know we're dropping the original and can clean up.

#### `objc`
This stores the underlying Objective-C object (e.g, `NSView` or `UIView`).

#### Layer backing
Controls should expose a `layer` property. On macOS this technically should be optional, but I'm making the BDFL decision to enforce it being there, because
outside of a shrinking set of cases, you want it there.

#### `delegate`
A `delegate` is our Rust trait impl that receives callbacks from the core control.

For instance, you might have a `View<DragAndDrop>` that calls are forwarded to. It'd look something like the following:

``` rust
pub struct DragAndDrop;

impl cacao::view::ViewDelegate for DragAndDrop {
    const NAME: &'static str = "DragAndDropView";

    fn did_load(&mut self, view: cacao::view::View) {
        // Customize View in here, persist it or something
    }

    // implement various drag and drop handlers
}
```

And the `View<DragAndDrop>` would be constructed like so:

``` rust
let dnd_view = View::with(DragAndDrop);
```

#### AutoLayout Abound
The various Layout Anchors in here are used for AutoLayout (positioning/sizing on the screen). They should always be set.


#### Default
Not every control needs a `Default` impl, but View defaulting is convenient for deep initialization, so we offer it.

``` rust
impl Default for View {
    /// Returns a stock view, for... well, whatever you want.
    fn default() -> Self {
        View::new()
    }
}

```

### Base Initializers
So next on the list is `View::new()`. We also have an internal `init(view)` method to collect some logic that we need in two places.

`register_view_class()` is located in `view/macos.rs` on macOS, and `view/ios.rs` on iOS; this is a _bridge_ that handles class setup.
We'll look at this more in-depth below, but the general idea here is that the method returns a `Class *` that can be used to create a
new Objective-C object.

``` rust
impl View {
    /// An internal initializer method for very common things that we need to do, regardless of
    /// what type the end user is creating.
    ///
    /// This handles grabbing autolayout anchor pointers, as well as things related to layering and
    /// so on. It returns a generic `View<T>`, which the caller can then customize as needed.
    pub(crate) fn init<T>(view: id) -> View<T> {
        unsafe {
            let _: () = msg_send![view, setTranslatesAutoresizingMaskIntoConstraints:NO];

            #[cfg(target_os = "macos")]
            let _: () = msg_send![view, setWantsLayer:YES];
        }

        View {
            is_handle: false,
            delegate: None,
            top: LayoutAnchorY::top(view),
            left: LayoutAnchorX::left(view),
            leading: LayoutAnchorX::leading(view),
            right: LayoutAnchorX::right(view),
            trailing: LayoutAnchorX::trailing(view),
            bottom: LayoutAnchorY::bottom(view),
            width: LayoutAnchorDimension::width(view),
            height: LayoutAnchorDimension::height(view),
            center_x: LayoutAnchorX::center(view),
            center_y: LayoutAnchorY::center(view),

            layer: Layer::wrap(unsafe {
                msg_send![view, layer]
            }),

            objc: ObjcProperty::retain(view),
        }
    }

    /// Returns a default `View`, suitable for customizing and displaying.
    pub fn new() -> Self {
        View::init(unsafe {
            msg_send![register_view_class(), new]
        })
    }
}
```

### Delegate Initializer
For types that accept a delegate, the common pattern we use is for the initializer to be named `with()`. Below, we implement `with()` for `View<T>`:

``` rust
impl<T> View<T> where T: ViewDelegate + 'static {
    /// Initializes a new View with a given `ViewDelegate`. This enables you to respond to events
    /// and customize the view as a module, similar to class-based systems.
    pub fn with(delegate: T) -> View<T> {
        let class = register_view_class_with_delegate(&delegate);
        let mut delegate = Box::new(delegate);

        let view = unsafe {
            let view: id = msg_send![class, new];
            let ptr = Box::into_raw(delegate);
            (&mut *view).set_ivar(VIEW_DELEGATE_PTR, ptr as usize);
            delegate = Box::from_raw(ptr);
            view
        };

        let mut view = View::init(view);
        (&mut delegate).did_load(view.clone_as_handle());
        view.delegate = Some(delegate);
        view
    }
}
```

Note that we use a second view class registration function here, as it performs some extra work to ensure that a unique subclass is created per-Rust-type. We dive
into some `unsafe` here, as we need to set a pointer to our trait object `T` so that the callbacks are able to load and call when coming around from the Objective-C
side - this is explained more in the _bridge_ sections below.

### Drawing the Owl
As this block is likely to still grow, we'll be somewhat brief here - below, we implement a `clone_as_handle()` method, which returns a bland clone of this type (a _handle_).
Notably, this does not have a _delegate_ reference, and `is_handle` is set to `true`. This is passed to the trait implementation in `did_load()`, to enable the trait having access
to the containing Objective-C type.

``` rust
impl<T> View<T> {
    /// An internal method that returns a clone of this object, sans references to the delegate or
    /// callback pointer. We use this in calling `did_load()` - implementing delegates get a way to
    /// reference, customize and use the view but without the trickery of holding pieces of the
    /// delegate - the `View` is the only true holder of those.
    pub(crate) fn clone_as_handle(&self) -> View {
        View {
            delegate: None,
            is_handle: true,
            layer: self.layer.clone(),
            top: self.top.clone(),
            leading: self.leading.clone(),
            left: self.left.clone(),
            trailing: self.trailing.clone(),
            right: self.right.clone(),
            bottom: self.bottom.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
            center_x: self.center_x.clone(),
            center_y: self.center_y.clone(),
            objc: self.objc.clone()
        }
    }

    /// Call this to set the background color for the backing layer.
    pub fn set_background_color<C: AsRef<Color>>(&self, color: C) {
        let color: id = color.as_ref().into();

        #[cfg(target_os = "macos")]
        self.objc.with_mut(|obj| unsafe {
            (&mut *obj).set_ivar(BACKGROUND_COLOR, color);
        });

        #[cfg(target_os = "ios")]
        self.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![&*obj, setBackgroundColor:color];
        });
    }
}
```

We also see a `set_background_color`, which performs different calls depending on the target OS environment: iOS supports background colors by default, and macOS... well, we store it as an `ivar`, and then rely on the
layer painting itself in the _bridge_ implementation. You might see some cases online where code simply just does `layer.backgroundColor.cgColor = ...`, but this doesn't work properly for dark mode support.

### Layout Support
The `layout::Layout` trait implements a slew of commonly needed functions, such as setting frames, handling view adding/removing, hiding and showing, and so on. Controls need only implement one or two `Layout` trait methods
to get most of this for free:

``` rust
impl<T> Layout for View<T> {
    fn with_backing_node<F: Fn(id)>(&self, handler: F) {
        self.objc.with_mut(handler);
    }

    fn get_from_backing_node<F: Fn(&Object) -> R, R>(&self, handler: F) -> R {
        self.objc.get(handler)
    }
}
```

Here, we simply pass handlers into the `objc` field calls. With this setup, we're able to offer relatively sound checks for borrowing the underlying types, and most other `Layout` methods "just work".

### Dropping
Here, we simply check if the dropping item is a handle or not. If it's not (i.e, if it's the top-level original), we can ensure it's removed from the Objective-V view heirarchy and be on our way.

``` rust
impl<T> Drop for View<T> {
    /// If the instance being dropped is _not_ a handle, then we want to go ahead and explicitly
    /// remove it from any super views.
    ///
    /// Why do we do this? It's to try and match Rust's ownership model/semantics. If a Rust value
    /// drops, it (theoretically) makes sense that the View would drop... and not be visible, etc.
    ///
    /// If you're venturing into unsafe code for the sake of custom behavior via the Objective-C
    /// runtime, you can consider flagging your instance as a handle - it will avoid the drop logic here.
    fn drop(&mut self) {
        if !self.is_handle {
            self.remove_from_superview();
        }
    }
}
```

## Bridges(s)
We'll step through an example (abridged) `View` bridge below, for macOS. You should consult the full implementation in `view/` to learn more after reading this.

For our basic `View` type, we want to just map to the corresponding class on the Objective-C side (in this case, `NSView`), and maybe do a bit of tweaking for sanity reasons.

``` rust
pub(crate) fn register_view_class() -> *const Class {
    static mut VIEW_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSView);
        let mut decl = ClassDecl::new("RSTView", superclass).unwrap();

        decl.add_method(sel!(isFlipped), enforce_normalcy as extern "C" fn(&Object, _) -> BOOL);

        decl.add_ivar::<id>(BACKGROUND_COLOR);

        VIEW_CLASS = decl.register();
    });

    unsafe { VIEW_CLASS }
}
```

This function (called inside `View::new()`) creates one reusable `View` subclass, and returns the type on subsequent calls. We're able to add methods to it (`add_method`) which match
Objective-C method signatures, as well as provision space for variable storage (`add_ivar`).

For our _delegate_ types, we need a different class creation method - one that creates a subclass per-unique-type:

``` rust
pub(crate) fn register_view_class_with_delegate<T: ViewDelegate>(instance: &T) -> *const Class {
    load_or_register_class("NSView", instance.subclass_name(), |decl| unsafe {
        decl.add_ivar::<usize>(VIEW_DELEGATE_PTR);
        decl.add_ivar::<id>(BACKGROUND_COLOR);

        decl.add_method(
            sel!(isFlipped),
            enforce_normalcy as extern "C" fn(&Object, _) -> BOOL
        );

        decl.add_method(
            sel!(draggingEntered:),
            dragging_entered::<T> as extern "C" fn (&mut Object, _, _) -> NSUInteger
        );
    })
}
```

Here, we add a method that only makes sense if you're using a delegate (notifying about a drag-enter event). We also provision an extra storage slot, which contains a pointer
to the Rust `ViewDelegate` implementation.

The methods we're setting up can range from simple to complex - take `isFlipped`:

``` rust
extern "C" fn is_flipped(_: &Object, _: Sel) -> BOOL {
    return YES;
}
```

Here, we just want to tell `NSView` to use top,left as the origin point, so we need to respond `YES` in this subclass method.

``` rust
extern "C" fn dragging_entered<T: ViewDelegate>(this: &mut Object, _: Sel, info: id) -> NSUInteger {
    let view = utils::load::<T>(this, VIEW_DELEGATE_PTR);
    view.dragging_entered(DragInfo {
        info: unsafe { Id::from_ptr(info) }
    }).into()
}
```

This is an example of a more complex method: we load the `ViewDelegate` type from the pointer set on the object, and forward the information
into the `dragging_entered` trait method.

## Conclusion
Hopefully this helps newcomers understand the design choices and architecture found in this repository. It can feel odd at first, but ends up lending itself well to UI patterns, and provides
some structure for how things should work.
