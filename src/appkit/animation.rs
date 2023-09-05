use block::ConcreteBlock;
use objc::{class, msg_send, sel};

use crate::foundation::id;

/// A very, very basic wrapper around NSAnimationContext. 100% subject to change.
#[derive(Debug)]
pub struct AnimationContext(id);

impl AnimationContext {
    /// Wraps an NSAnimationContext pointer.
    pub fn new(ctx: id) -> Self {
        Self(ctx)
    }

    /// Sets the animation duration.
    pub fn set_duration(&mut self, duration: f64) {
        unsafe {
            let _: () = msg_send![self.0, setDuration: duration];
        }
    }

    /// Pass it a block, and the changes in that block will be animated, provided they're
    /// properties that support animation.
    ///
    /// [https://developer.apple.com/documentation/appkit/nsanimationcontext?language=objc]
    ///
    /// For more information, you should consult the documentation for NSAnimationContext, then skim
    /// the supported methods here.
    pub fn run<F>(animation: F)
    where
        F: Fn(&mut AnimationContext) + Send + Sync + 'static
    {
        let block = ConcreteBlock::new(move |ctx| {
            let mut context = AnimationContext(ctx);
            animation(&mut context);
        });
        let block = block.copy();

        unsafe {
            //let context: id = msg_send![class!(NSAnimationContext), currentContext];
            let _: () = msg_send![class!(NSAnimationContext), runAnimationGroup: block];
        }
    }

    /// Pass it a block, and the changes in that block will be animated, provided they're
    /// properties that support animation.
    ///
    /// [https://developer.apple.com/documentation/appkit/nsanimationcontext?language=objc]
    ///
    /// For more information, you should consult the documentation for NSAnimationContext, then skim
    /// the supported methods here.
    pub fn run_with_completion_handler<F, C>(animation: F, completion_handler: C)
    where
        F: Fn(&mut AnimationContext) + Send + Sync + 'static,
        C: Fn() + Send + Sync + 'static
    {
        let block = ConcreteBlock::new(move |ctx| {
            let mut context = AnimationContext(ctx);
            animation(&mut context);
        });
        let block = block.copy();

        let completion_block = ConcreteBlock::new(completion_handler);
        let completion_block = completion_block.copy();

        unsafe {
            //let context: id = msg_send![class!(NSAnimationContext), currentContext];
            let _: () = msg_send![class!(NSAnimationContext), runAnimationGroup:block
                completionHandler:completion_block];
        }
    }
}
