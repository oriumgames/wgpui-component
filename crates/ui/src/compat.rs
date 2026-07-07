//! Compatibility shims for building against **wgpui**
//! (<https://github.com/Far-Beyond-Pulsar/wgpui>), a single-backend
//! (wgpu/winit/taffy) fork of gpui-ce.
//!
//! gpui-component targets the bleeding-edge `gpui` from zed, which has grown a
//! full accessibility layer (`Role`, aria element methods, `on_a11y_action`,
//! …) and a handful of newer helpers (`Pixels::as_f32`). wgpui, forked from the
//! gpui-ce lineage, does not yet expose these. This module provides drop-in
//! replacements so the component library compiles and behaves identically,
//! with the accessibility metadata reduced to inert no-ops until wgpui grows
//! the corresponding APIs.
//!
//! Everything here is re-exported from the crate root so downstream modules can
//! `use crate::compat::...` (or rely on the prelude glob).

use gpui::{Bounds, GradientStop, LinearColorStop, Pixels, Point, SharedString, Size, Styled};

/// zed gpui accepted a `LinearColorStop` anywhere a gradient stop was expected
/// (via `From<LinearColorStop> for GradientStop`). wgpui keeps the two types
/// distinct and provides no such conversion, so this bridges them explicitly.
pub trait LinearColorStopExt {
    fn to_gradient_stop(self) -> GradientStop;
}

impl LinearColorStopExt for LinearColorStop {
    #[inline]
    fn to_gradient_stop(self) -> GradientStop {
        GradientStop {
            color: self.color,
            position: self.percentage,
        }
    }
}

/// Accessibility role for an element.
///
/// Mirrors the variants used across gpui-component. On wgpui these carry no
/// runtime effect yet — they exist so the component code type-checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Role {
    Alert,
    AlertDialog,
    Button,
    Cell,
    CheckBox,
    ColumnHeader,
    ComboBox,
    DateInput,
    DateTimeInput,
    Dialog,
    EmailInput,
    Link,
    List,
    ListItem,
    Menu,
    MenuBar,
    MenuItem,
    MultilineTextInput,
    PasswordInput,
    PhoneNumberInput,
    ProgressIndicator,
    RadioButton,
    RadioGroup,
    Row,
    RowGroup,
    Slider,
    SpinButton,
    Tab,
    TabList,
    Table,
    TextInput,
    Toolbar,
    UrlInput,
}

/// Tri-state toggle value for `aria-checked` / `aria-pressed` semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Toggled {
    True,
    False,
}

/// Orientation for composite widgets (sliders, toolbars, …).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

/// Accessibility actions a user agent can request on an element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AccessibleAction {
    Increment,
    Decrement,
}

/// Extension trait supplying the accessibility builder methods that zed's gpui
/// exposes on elements. On wgpui each method is a no-op that returns `self`
/// unchanged, so existing call chains keep working.
///
/// Implemented for every type via a blanket impl; the methods only apply where
/// gpui-component chains them onto elements.
pub trait Accessible: Sized {
    #[inline]
    fn role(self, _role: Role) -> Self {
        self
    }
    #[inline]
    fn aria_label(self, _label: impl Into<SharedString>) -> Self {
        self
    }
    #[inline]
    fn aria_selected(self, _selected: bool) -> Self {
        self
    }
    #[inline]
    fn aria_expanded(self, _expanded: bool) -> Self {
        self
    }
    #[inline]
    fn aria_toggled(self, _toggled: Toggled) -> Self {
        self
    }
    #[inline]
    fn aria_orientation(self, _orientation: Orientation) -> Self {
        self
    }
    #[inline]
    fn aria_numeric_value<T>(self, _value: T) -> Self {
        self
    }
    #[inline]
    fn aria_min_numeric_value<T>(self, _value: T) -> Self {
        self
    }
    #[inline]
    fn aria_max_numeric_value<T>(self, _value: T) -> Self {
        self
    }
    #[inline]
    fn aria_position_in_set<T>(self, _pos: T) -> Self {
        self
    }
    #[inline]
    fn aria_size_of_set<T>(self, _size: T) -> Self {
        self
    }
    #[inline]
    fn aria_column_index<T>(self, _index: T) -> Self {
        self
    }
    #[inline]
    fn aria_row_index<T>(self, _index: T) -> Self {
        self
    }
    #[inline]
    fn on_a11y_action<F>(self, _action: AccessibleAction, _handler: F) -> Self
    where
        F: Fn(&(), &mut gpui::Window, &mut gpui::App) + 'static,
    {
        self
    }
}

impl<T> Accessible for T {}

/// `Pixels::as_f32`, present in zed's gpui but not in wgpui.
pub trait PixelsExt {
    fn as_f32(&self) -> f32;
}

impl PixelsExt for Pixels {
    #[inline]
    fn as_f32(&self) -> f32 {
        f32::from(*self)
    }
}

/// Reference point on a [`gpui::Bounds`] rectangle. Mirrors zed gpui's `Anchor`
/// enum, which wgpui does not expose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Anchor {
    TopLeft,
    TopCenter,
    TopRight,
    LeftCenter,
    RightCenter,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Anchor {
    /// Convert to wgpui's 4-corner [`gpui::Corner`], used by `anchored().anchor()`.
    /// The center variants (which zed's `Corner` lacks) collapse onto the
    /// nearest left/top corner.
    pub fn to_corner(self) -> gpui::Corner {
        use gpui::Corner;
        match self {
            Anchor::TopLeft | Anchor::TopCenter | Anchor::LeftCenter => Corner::TopLeft,
            Anchor::TopRight | Anchor::RightCenter => Corner::TopRight,
            Anchor::BottomLeft | Anchor::BottomCenter => Corner::BottomLeft,
            Anchor::BottomRight => Corner::BottomRight,
        }
    }

    /// Mirror of `Corner::other_side_corner_along` for the corner variants.
    pub fn other_side_along(self, axis: gpui::Axis) -> Self {
        use gpui::Axis;
        match axis {
            Axis::Vertical => match self {
                Anchor::TopLeft => Anchor::BottomLeft,
                Anchor::TopRight => Anchor::BottomRight,
                Anchor::BottomLeft => Anchor::TopLeft,
                Anchor::BottomRight => Anchor::TopRight,
                other => other,
            },
            Axis::Horizontal => match self {
                Anchor::TopLeft => Anchor::TopRight,
                Anchor::TopRight => Anchor::TopLeft,
                Anchor::BottomLeft => Anchor::BottomRight,
                Anchor::BottomRight => Anchor::BottomLeft,
                other => other,
            },
        }
    }
}

/// [`gpui::Bounds`] helpers present in zed gpui but missing from wgpui.
pub trait BoundsExt {
    /// The middle of the top edge.
    fn top_center(&self) -> Point<Pixels>;
    /// Build bounds from the position of one of its anchor points and a size.
    fn from_anchor_and_size(
        anchor: Anchor,
        anchor_point: Point<Pixels>,
        size: Size<Pixels>,
    ) -> Self;
}

impl BoundsExt for Bounds<Pixels> {
    #[inline]
    fn top_center(&self) -> Point<Pixels> {
        Point {
            x: self.origin.x + self.size.width / 2.,
            y: self.origin.y,
        }
    }

    fn from_anchor_and_size(
        anchor: Anchor,
        anchor_point: Point<Pixels>,
        size: Size<Pixels>,
    ) -> Self {
        let Point { x, y } = anchor_point;
        let (w, h) = (size.width, size.height);
        let half_w = w / 2.;
        let half_h = h / 2.;
        let origin = match anchor {
            Anchor::TopLeft => Point { x, y },
            Anchor::TopCenter => Point { x: x - half_w, y },
            Anchor::TopRight => Point { x: x - w, y },
            Anchor::LeftCenter => Point { x, y: y - half_h },
            Anchor::RightCenter => Point {
                x: x - w,
                y: y - half_h,
            },
            Anchor::BottomLeft => Point { x, y: y - h },
            Anchor::BottomCenter => Point {
                x: x - half_w,
                y: y - h,
            },
            Anchor::BottomRight => Point { x: x - w, y: y - h },
        };
        Bounds { origin, size }
    }
}

/// Tailwind-style flex helpers (`flex_grow_1`, `flex_shrink_1`, …) that zed's
/// gpui generates on [`Styled`] but wgpui does not.
pub trait FlexExt: Styled + Sized {
    #[inline]
    fn flex_grow_1(mut self) -> Self {
        self.style().flex_grow = Some(1.);
        self
    }
    #[inline]
    fn flex_shrink_1(mut self) -> Self {
        self.style().flex_shrink = Some(1.);
        self
    }
    /// zed gpui's `flex_grow(f32)` took an explicit grow factor; wgpui's
    /// `flex_grow()` is fixed at 1. This preserves the explicit-factor variant.
    #[inline]
    fn flex_grow_amount(mut self, factor: f32) -> Self {
        self.style().flex_grow = Some(factor);
        self
    }
}

impl<T: Styled> FlexExt for T {}
