use std::convert::TryFrom;

use crate::id_shim::ShareId;
use objc::{class, msg_send, runtime::Object, sel};

use crate::foundation::NSUInteger;

#[derive(Clone, Debug)]
pub struct HapticFeedbackPerformer(pub ShareId<Object>);

impl HapticFeedbackPerformer {
    pub fn perform(&self, pattern: FeedbackPattern, performance_time: PerformanceTime) {
        unsafe {
            let _: () = msg_send![&*self.0, performFeedbackPattern: pattern as isize performanceTime: performance_time as usize];
        }
    }
}

impl Default for HapticFeedbackPerformer {
    /// Returns the default haptic feedback performer.
    fn default() -> Self {
        HapticFeedbackPerformer(unsafe {
            let manager = msg_send![class!(NSHapticFeedbackManager), defaultPerformer];
            ShareId::from_ptr(manager)
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PerformanceTime {
    Default = 0,
    Now = 1,
    DrawCompleted = 2
}

impl Default for PerformanceTime {
    fn default() -> Self {
        Self::Default
    }
}

impl TryFrom<f64> for PerformanceTime {
    type Error = &'static str;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        match value as u8 {
            0 => Ok(Self::Default),
            1 => Ok(Self::Now),
            2 => Ok(Self::DrawCompleted),
            _ => Err("Invalid performance time")
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum FeedbackPattern {
    Generic = 0,
    Alignment = 1,
    LevelChange = 2
}

impl Default for FeedbackPattern {
    fn default() -> Self {
        Self::Generic
    }
}

impl TryFrom<f64> for FeedbackPattern {
    type Error = &'static str;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        match value as u8 {
            0 => Ok(Self::Generic),
            1 => Ok(Self::Alignment),
            2 => Ok(Self::LevelChange),
            _ => Err("Invalid feedback pattern")
        }
    }
}
