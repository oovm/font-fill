use super::*;

impl MouseEventWrapper {
    /// Getter for the `screenX` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/screenX)
    #[inline]
    pub fn screen_x(&self) -> i32 {
        self.0.screen_x()
    }

    /// Getter for the `screenY` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/screenY)
    #[inline]
    pub fn screen_y(&self) -> i32 {
        self.0.screen_y()
    }
    /// Getter for the `clientX` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/clientX)
    #[inline]
    pub fn client_x(&self) -> i32 {
        self.0.client_x()
    }

    /// Getter for the `clientY` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/clientY)
    #[inline]
    pub fn client_y(&self) -> i32 {
        self.0.client_y()
    }
    /// Getter for the `x` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/x)
    #[inline]
    pub fn x(&self) -> i32 {
        self.0.x()
    }

    /// Getter for the `y` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/y)
    #[inline]
    pub fn y(&self) -> i32 {
        self.0.y()
    }

    /// Getter for the `offsetX` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/offsetX)
    #[inline]
    pub fn offset_x(&self) -> i32 {
        self.0.offset_x()
    }

    /// Getter for the `offsetY` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/offsetY)
    #[inline]
    pub fn offset_y(&self) -> i32 {
        self.0.offset_y()
    }

    /// Getter for the `ctrlKey` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/ctrlKey)
    #[inline]
    pub fn ctrl_key(&self) -> bool {
        self.0.ctrl_key()
    }

    /// Getter for the `shiftKey` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/shiftKey)
    #[inline]
    pub fn shift_key(&self) -> bool {
        self.0.shift_key()
    }

    /// Getter for the `altKey` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/altKey)
    pub fn alt_key(&self) -> bool {
        self.0.alt_key()
    }

    /// Getter for the `metaKey` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/metaKey)
    #[inline]
    pub fn meta_key(&self) -> bool {
        self.0.meta_key()
    }

    /// Getter for the `button` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/button)
    #[inline]
    pub fn button(&self) -> i16 {
        self.0.button()
    }

    /// Getter for the `buttons` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/buttons)
    #[inline]
    pub fn buttons(&self) -> u16 {
        self.0.buttons()
    }
    /// Getter for the `relatedTarget` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/relatedTarget)
    #[inline]
    pub fn related_target(&self) -> Option<EventTarget> {
        self.0.related_target()
    }

    /// Getter for the `region` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/region)
    #[inline]
    pub fn region(&self) -> Option<String> {
        self.0.region()
    }

    /// Getter for the `movementX` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/movementX)
    #[inline]
    pub fn movement_x(&self) -> i32 {
        self.0.movement_x()
    }

    /// Getter for the `movementY` field of this object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/movementY)
    #[inline]
    pub fn movement_y(&self) -> i32 {
        self.0.movement_y()
    }
}
