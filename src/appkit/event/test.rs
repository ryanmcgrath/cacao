#[cfg(test)] #[rustfmt::skip]
mod event_test {
    use crate::appkit::EventModifierBitFlag;
    #[test]
    fn test_event_modifier_bit_flag_display() {
        assert_eq!("‹⇧"           ,format!("{}"  ,EventModifierBitFlag::LeftShift  ));
        assert_eq!("‹⇧›"          ,format!("{}"  ,EventModifierBitFlag::Shift      ));
        assert_eq!("⇧›"           ,format!("{}"  ,EventModifierBitFlag::RightShift ));
        assert_eq!("‹⇧‹⌃‹⌥‹⌘"    ,format!("{}"  ,EventModifierBitFlag::LeftModi   ));
        assert_eq!("⇧›⌃›⌥›⌘›"    ,format!("{}"  ,EventModifierBitFlag::RightModi  ));
        assert_eq!("‹⇧›‹⌃›‹⌥›‹⌘›",format!("{}"  ,EventModifierBitFlag::LRModi     ));
        assert_eq!("🌐"           ,format!("{:#}",EventModifierBitFlag::Function   )); // when it's a modifier key
        assert_eq!("ƒ"            ,format!("{}"  ,EventModifierBitFlag::Function   ));
        assert_eq!("⇭"            ,format!("{:#}",EventModifierBitFlag::Numpad     )); // when it's a modifier key
        assert_eq!("🔢"           ,format!("{}"  ,EventModifierBitFlag::Numpad     ));
        assert_eq!("ℹ"           ,format!("{}"  ,EventModifierBitFlag::Help       ));
        assert_eq!("!🖮"           ,format!("{}"  ,EventModifierBitFlag::DeviceIndependentFlagsMask));
        // Shift ignored, only flagmask remains
        assert_eq!("!🖮"           ,format!("{}"  ,EventModifierBitFlag::DeviceIndependentFlagsMask | EventModifierBitFlag::RightShift));
    }
}
