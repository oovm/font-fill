use super::*;

/// methods inherit from [`PointerEvent`]
impl OnClick {
    /// Getter for the `pointerId` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/pointerId)
    #[inline]
    pub fn pointer_id(&self) -> i32 {
        self.0.pointer_id()
    }
    /// Getter for the `width` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/width)
    #[inline]
    pub fn width(&self) -> i32 {
        self.0.width()
    }
    /// Getter for the `height` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/height)
    #[inline]
    pub fn height(&self) -> i32 {
        self.0.height()
    }
    /// Getter for the `pressure` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/pressure)
    #[inline]
    pub fn pressure(&self) -> f32 {
        self.0.pressure()
    }
    /// Getter for the `tangentialPressure` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/tangentialPressure)
    #[inline]
    pub fn tangential_pressure(&self) -> f32 {
        self.0.tangential_pressure()
    }
    /// Getter for the `tiltX` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/tiltX)
    #[inline]
    pub fn tilt_x(&self) -> i32 {
        self.0.tilt_x()
    }
    /// Getter for the `tiltY` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/tiltY)
    #[inline]
    pub fn tilt_y(&self) -> i32 {
        self.0.tilt_y()
    }
    /// Getter for the `twist` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/twist)
    #[inline]
    pub fn twist(&self) -> i32 {
        self.0.twist()
    }
    /// Getter for the `pointerType` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/pointerType)
    #[inline]
    pub fn pointer_type(&self) -> String {
        self.0.pointer_type()
    }
    /// Getter for the `isPrimary` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/isPrimary)
    #[inline]
    pub fn is_primary(&self) -> bool {
        self.0.is_primary()
    }
    /// The `getCoalescedEvents()` method.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/PointerEvent/getCoalescedEvents)
    #[inline]
    pub fn get_coalesced_events(&self) -> Array {
        self.0.get_coalesced_events()
    }
}
