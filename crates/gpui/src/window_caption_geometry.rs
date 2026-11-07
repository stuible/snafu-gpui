//! Pure window-control region geometry for the Windows AppWindow caption
//! host: the translation of a completed frame's logical control layout into
//! physical client-coordinate region rectangles, the caption/Passthrough
//! planning decision (title-band clipping, measured inset reservation,
//! SDK-owned rectangle exclusion, kind disjointness), the overlay suppression
//! rule and the payload freshness check. Plain data — std only, no SDK, no
//! Win32 — so the rules the native caption depends on are unit-testable
//! off-Windows. The host drives the SDK setters through the attach
//! transaction's stages; nothing here keeps state. Compiled on Windows and in
//! test builds, exactly like `window_attach_txn`, so the default-feature lint
//! builds of other platforms never see it.

use crate::WindowControlLayout;
use crate::{Bounds, Pixels};

/// One physical client-coordinate rectangle for an SDK region set — the
/// plain form of the bindings' `RectInt32`, kept SDK-free so the planning
/// rules run off-Windows.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RegionRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

/// Physical window metrics for region planning, measured through the SDK
/// and Win32 at the current size/DPI. Never cached across metric
/// invalidations by the caller.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RegionMetrics {
    pub caption_height: i32,
    pub left_inset: i32,
    pub right_inset: i32,
}

impl RegionMetrics {
    /// Whether the measured metrics may shape native geometry at all. A
    /// non-positive title height means the SDK supplied nothing usable;
    /// insets may legitimately be zero, which reserves no strip.
    pub fn usable(&self) -> bool {
        self.caption_height > 0 && self.left_inset >= 0 && self.right_inset >= 0
    }
}

impl RegionRect {
    fn valid(&self) -> bool {
        self.width > 0 && self.height > 0
    }

    fn intersect(&self, other: &RegionRect) -> Option<RegionRect> {
        let left = self.x.max(other.x);
        let top = self.y.max(other.y);
        let right = (self.x + self.width).min(other.x + other.width);
        let bottom = (self.y + self.height).min(other.y + other.height);
        let rect = RegionRect {
            x: left,
            y: top,
            width: right - left,
            height: bottom - top,
        };
        rect.valid().then_some(rect)
    }

    /// The pieces of `self` that remain after removing `cut`: at most the
    /// four strips around it. Small-rectangle clipping for the explicit
    /// rectangles this module plans — never a general hitbox partitioner.
    fn subtract(&self, cut: &RegionRect) -> Vec<RegionRect> {
        let Some(core) = self.intersect(cut) else {
            return [*self].to_vec();
        };
        [
            RegionRect {
                x: self.x,
                y: self.y,
                width: self.width,
                height: core.y - self.y,
            },
            RegionRect {
                x: self.x,
                y: core.y + core.height,
                width: self.width,
                height: (self.y + self.height) - (core.y + core.height),
            },
            RegionRect {
                x: self.x,
                y: core.y,
                width: core.x - self.x,
                height: core.height,
            },
            RegionRect {
                x: core.x + core.width,
                y: core.y,
                width: (self.x + self.width) - (core.x + core.width),
                height: core.height,
            },
        ]
        .into_iter()
        .filter(|rect| rect.valid())
        .collect()
    }
}

/// Rounds a logical rectangle to whole physical pixels INWARD (it only
/// shrinks), so a published caption rectangle never claims a pixel the
/// frame did not draw as drag. Returns `None` for an empty remainder.
pub fn logical_to_physical_inward(
    bounds: Bounds<Pixels>,
    scale_factor: f32,
) -> Option<RegionRect> {
    let x = (bounds.left().0 * scale_factor).ceil() as i32;
    let y = (bounds.top().0 * scale_factor).ceil() as i32;
    let right = (bounds.right().0 * scale_factor).floor() as i32;
    let bottom = (bounds.bottom().0 * scale_factor).floor() as i32;
    let rect = RegionRect {
        x,
        y,
        width: right - x,
        height: bottom - y,
    };
    rect.valid().then_some(rect)
}

/// Rounds a logical rectangle to whole physical pixels OUTWARD (it only
/// grows), so a published client-passthrough rectangle always covers every
/// pixel the frame routed as client. Returns `None` for an empty
/// remainder after the caller clips to the client bounds.
pub fn logical_to_physical_outward(
    bounds: Bounds<Pixels>,
    scale_factor: f32,
) -> Option<RegionRect> {
    let x = (bounds.left().0 * scale_factor).floor() as i32;
    let y = (bounds.top().0 * scale_factor).floor() as i32;
    let right = (bounds.right().0 * scale_factor).ceil() as i32;
    let bottom = (bounds.bottom().0 * scale_factor).ceil() as i32;
    let rect = RegionRect {
        x,
        y,
        width: right - x,
        height: bottom - y,
    };
    rect.valid().then_some(rect)
}

/// A drag rectangle with its hitbox's position in the frame's back-to-
/// front hitbox list.
pub struct OrderedDragRect {
    pub rect: Bounds<Pixels>,
    pub order: usize,
}

/// One frame hitbox: its mask-clipped bounds and whether it blocks mouse
/// interaction behind it.
pub struct OrderedHitbox {
    pub clipped: Bounds<Pixels>,
    pub blocks_mouse: bool,
}

/// The overlay suppression rule: a drag rectangle any LATER mouse-blocking
/// hitbox intersects is suppressed WHOLE — the frame drew an interactive
/// overlay over (part of) it, so dragging must yield entirely rather than
/// partition the rectangle around every overlay hitbox. Non-blocking
/// hitboxes never suppress; rectangles behind nothing stay published.
pub fn suppress_covered_drag_rects(
    drags: &[OrderedDragRect],
    hitboxes: &[OrderedHitbox],
) -> Vec<Bounds<Pixels>> {
    drags
        .iter()
        .filter(|drag| {
            hitboxes
                .get(drag.order + 1..)
                .map(|later| {
                    !later
                        .iter()
                        .any(|hitbox| hitbox.blocks_mouse && hitbox.clipped.intersects(&drag.rect))
                })
                .unwrap_or(true)
        })
        .map(|drag| drag.rect)
        .collect()
}

/// Why a stored layout payload may not be applied to the live window.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StaleLayoutReason {
    /// The payload names another native window identity: an HWND value
    /// reuse never makes a previous identity's payload applicable.
    Identity,
    /// The payload was stamped before this window's latest metric
    /// invalidation (a resize, DPI change, activation or fullscreen
    /// toggle).
    Revision,
    /// The payload's logical extent, at its own scale factor, does not
    /// match the window's ACTUAL physical client extent: the frame
    /// completed under a different size, possibly a reentrant size
    /// change that landed mid-draw.
    Extent,
    /// The payload's scale factor does not match the window's actual DPI
    /// scale.
    Scale,
    /// The payload's frame is OLDER than the frame whose plan is already
    /// applied; region sets never regress to an earlier frame.
    Frame,
}

impl StaleLayoutReason {
    /// Short reason text for the rejection log.
    pub fn text(self) -> &'static str {
        match self {
            StaleLayoutReason::Identity => "identity",
            StaleLayoutReason::Revision => "metric revision",
            StaleLayoutReason::Extent => "client extent",
            StaleLayoutReason::Scale => "scale factor",
            StaleLayoutReason::Frame => "frame order",
        }
    }
}

/// Whether a stored layout payload may still be applied to the live
/// window. Every currency fact is checked at application time against the
/// window's ACTUAL native state — exact native identity (a reused HWND
/// value never matches), the current metric revision (a payload stamped
/// before a resize, DPI change, activation or fullscreen toggle is
/// stale), the actual physical client extent and DPI scale (the frame
/// must have been drawn under the window's present geometry — never a
/// cached or self-declared measurement), and frame ordering (an older
/// frame never replaces the plan of a newer applied one).
pub fn payload_currency(
    payload: &WindowControlLayout,
    session_identity: u64,
    current_revision: u64,
    actual_client: RegionRect,
    actual_scale_factor: f32,
    last_applied_frame: Option<u64>,
) -> Result<(), StaleLayoutReason> {
    if payload.identity != session_identity {
        return Err(StaleLayoutReason::Identity);
    }
    if payload.metric_revision != current_revision {
        return Err(StaleLayoutReason::Revision);
    }
    // The logical viewport at the frame's own scale factor must still be
    // the window's physical client extent, within half a physical pixel
    // of rounding from the logical/physical conversion.
    let width = payload.client_size.width.0 * payload.scale_factor;
    let height = payload.client_size.height.0 * payload.scale_factor;
    if (width - actual_client.width as f32).abs() > 0.5
        || (height - actual_client.height as f32).abs() > 0.5
    {
        return Err(StaleLayoutReason::Extent);
    }
    // Windows DPI scales are integral DPI over 96 on both sides, so the
    // values agree exactly when current; the tolerance only absorbs
    // float representation noise.
    if (payload.scale_factor - actual_scale_factor).abs() > 0.001 {
        return Err(StaleLayoutReason::Scale);
    }
    if last_applied_frame.is_some_and(|applied| payload.frame < applied) {
        return Err(StaleLayoutReason::Frame);
    }
    Ok(())
}

/// What one queued region command (Refresh or ApplyRegions) must do with
/// the window's stored layout payload. The command KIND gives a MISSING
/// payload its meaning — the shared rule that keeps a duplicate queued
/// command from suspending a still-valid applied plan.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LayoutAction {
    /// The stored payload is exactly current: plan both region kinds
    /// from it. An explicit empty frame (a current payload with no drag
    /// rectangles) is this action too — its empty caption is a real plan,
    /// never "no data".
    ApplyPayload,
    /// Apply the suspension plan (no app caption, reserved-keeping
    /// whole-client passthrough). `reason` distinguishes a rejected
    /// stale payload from a refresh that found nothing stored under an
    /// outstanding metric invalidation.
    ApplySuspension { reason: Option<StaleLayoutReason> },
    /// An apply command found no stored payload: nothing new was
    /// published, so whatever plan is applied stands — never clear valid
    /// drag regions for lack of new data.
    RetainCurrent,
}

/// Decides what a queued region command does with the window's stored
/// layout payload. A refresh — the metric re-measurement — treats a
/// missing payload as the invalidation suspension; an apply command
/// treats it as "nothing new" and retains the applied plan. A STORED
/// payload must be exactly current (see [`payload_currency`]) for either
/// command to use it; a stale one suspends, whichever command queued.
pub fn queued_layout_action(
    command_is_refresh: bool,
    stored: Option<&WindowControlLayout>,
    session_identity: u64,
    current_revision: u64,
    actual_client: RegionRect,
    actual_scale_factor: f32,
    last_applied_frame: Option<u64>,
) -> LayoutAction {
    let Some(payload) = stored else {
        return if command_is_refresh {
            LayoutAction::ApplySuspension { reason: None }
        } else {
            LayoutAction::RetainCurrent
        };
    };
    match payload_currency(
        payload,
        session_identity,
        current_revision,
        actual_client,
        actual_scale_factor,
        last_applied_frame,
    ) {
        Ok(()) => LayoutAction::ApplyPayload,
        Err(reason) => LayoutAction::ApplySuspension {
            reason: Some(reason),
        },
    }
}

/// The region sets to publish for one frame: the app's Caption rectangles
/// and the client Passthrough rectangles, both physical and
/// client-coordinate. Empty `caption` publishes no app caption (the SDK's
/// own caption behavior then applies or the native caption is suspended); that is a
/// valid plan, never a failure.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RegionPlan {
    pub caption: Vec<RegionRect>,
    pub passthrough: Vec<RegionRect>,
}

/// Plans both region kinds for one apply.
///
/// `client` is the actual physical client rect; `metrics` is `None` when
/// no valid measurement exists. `drag`/`passthrough` are the frame's
/// already-rounded rectangles (inward/outward); `sdk_rects` are the
/// SDK-owned rectangles reported by `GetRegionRects` for its reserved
/// kinds (buttons, icon, borders).
///
/// With usable metrics the caption is the drag rectangles clipped to the
/// measured title band, the measured left/right inset strips reserved
/// (an EMPTY SDK region array is not proof the buttons have no geometry —
/// the measured insets are the conservative reservation), every SDK-owned
/// rectangle excluded, and the passthrough rectangles cut away so the
/// kinds stay disjoint. The explicit passthrough annotations keep out of
/// the reserved geometry too: an annotation crossing a native button or a
/// measured inset strip is clipped around it, never allowed to restore
/// client routing over a reservation. EVERYTHING else — the rest of the
/// band (an overlay-suppressed drag rectangle, the rail's top strip) and
/// the whole area below it — is planned as explicit client Passthrough:
/// no point of the client surface is ever left to an unproven default
/// routing. With unusable metrics there is no usable native drag layout:
/// no app caption is planned and the annotated passthrough survives, SDK
/// exclusions still applied.
pub fn plan_regions(
    client: RegionRect,
    metrics: Option<RegionMetrics>,
    drag: &[RegionRect],
    passthrough: &[RegionRect],
    sdk_rects: &[RegionRect],
) -> RegionPlan {
    let clip_to_client = |rects: &[RegionRect]| -> Vec<RegionRect> {
        rects
            .iter()
            .filter_map(|rect| rect.intersect(&client))
            .collect()
    };
    let exclude = |rects: &[RegionRect], cuts: &[&[RegionRect]]| -> Vec<RegionRect> {
        let mut pieces = rects.to_vec();
        for cut in cuts {
            let mut next = Vec::new();
            for piece in &pieces {
                // Subtract this cut group progressively: each cut removes
                // area from what the previous cuts left, never from the
                // original piece. An empty cut group removes nothing.
                let mut remainder = vec![*piece];
                for cut_rect in *cut {
                    let mut grown = Vec::new();
                    for remaining in &remainder {
                        grown.extend(remaining.subtract(cut_rect));
                    }
                    remainder = grown;
                }
                next.extend(remainder);
            }
            pieces = next;
        }
        pieces
    };
    let canonical = |mut rects: Vec<RegionRect>| -> Vec<RegionRect> {
        rects.sort_by_key(|rect| (rect.x, rect.y, rect.width, rect.height));
        rects.dedup();
        rects
    };

    let Some(metrics) = metrics.filter(|metrics| metrics.usable()) else {
        return RegionPlan {
            caption: Vec::new(),
            passthrough: canonical(exclude(&clip_to_client(passthrough), &[sdk_rects])),
        };
    };

    let band = RegionRect {
        x: client.x,
        y: client.y,
        width: client.width,
        height: metrics.caption_height.min(client.height),
    };
    let mut reservations: Vec<RegionRect> = sdk_rects.to_vec();
    if metrics.left_inset > 0 {
        reservations.push(RegionRect {
            x: band.x,
            y: band.y,
            width: metrics.left_inset,
            height: band.height,
        });
    }
    if metrics.right_inset > 0 {
        reservations.push(RegionRect {
            x: band.x + band.width - metrics.right_inset,
            y: band.y,
            width: metrics.right_inset,
            height: band.height,
        });
    }

    let passthrough_rects = clip_to_client(passthrough);
    let caption = canonical(exclude(
        &clip_to_client(drag)
            .iter()
            .filter_map(|rect| rect.intersect(&band))
            .collect::<Vec<_>>(),
        &[&reservations, &passthrough_rects],
    ));

    // Explicit annotations keep out of the reservations too: re-adding
    // the raw rectangles after the client remainder excluded them would
    // let an annotation crossing a native button or an inset strip
    // restore client routing over reserved geometry.
    let annotated_passthrough = exclude(&passthrough_rects, &[&reservations]);

    // Everything else routes as client: an overlay-suppressed drag
    // rectangle, the unclaimed band remainder, and the whole area below
    // the band. Only the SDK's own reserved rectangles and the reserved
    // inset strips are kept out of it.
    let mut client_remainder = exclude(&[client], &[&reservations, &passthrough_rects, &caption]);
    client_remainder.extend(annotated_passthrough);
    let passthrough = canonical(exclude(&client_remainder, &[&caption]));

    RegionPlan {
        caption,
        passthrough,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rect(x: i32, y: i32, width: i32, height: i32) -> RegionRect {
        RegionRect {
            x,
            y,
            width,
            height,
        }
    }

    fn bounds(left: f32, top: f32, right: f32, bottom: f32) -> Bounds<Pixels> {
        Bounds {
            origin: crate::point(Pixels(left), Pixels(top)),
            size: crate::size(Pixels(right - left), Pixels(bottom - top)),
        }
    }

    #[test]
    fn inward_rounding_shrinks_and_drops_empty_rects() {
        assert_eq!(
            logical_to_physical_inward(bounds(10.4, 0.6, 100.6, 30.2), 2.0),
            Some(rect(21, 2, 180, 58)),
            "ceil left/top, floor right/bottom: the rect only shrinks"
        );
        assert_eq!(
            logical_to_physical_inward(bounds(0., 0., 0.4, 30.), 1.0),
            None,
            "a rect with no whole physical pixel inside is dropped"
        );
        assert_eq!(
            logical_to_physical_outward(bounds(10.4, 0.6, 100.6, 30.2), 2.0),
            Some(rect(20, 1, 182, 60)),
            "floor left/top, ceil right/bottom: the rect only grows"
        );
    }

    #[test]
    fn a_later_blocking_hitbox_suppresses_the_whole_drag_rectangle() {
        let hitboxes = [
            OrderedHitbox {
                clipped: bounds(0., 0., 200., 35.),
                blocks_mouse: false,
            },
            OrderedHitbox {
                clipped: bounds(50., 0., 150., 35.),
                blocks_mouse: true,
            },
            OrderedHitbox {
                clipped: bounds(100., 0., 300., 35.),
                blocks_mouse: false,
            },
        ];
        let drags = [
            OrderedDragRect {
                rect: bounds(0., 0., 100., 35.),
                order: 0,
            },
            OrderedDragRect {
                rect: bounds(150., 0., 250., 35.),
                order: 2,
            },
        ];
        assert_eq!(
            suppress_covered_drag_rects(&drags, &hitboxes),
            vec![bounds(150., 0., 250., 35.)],
            "the first drag is covered by a later blocker and suppressed whole; \
             the second only meets a later non-blocker and stays"
        );
    }

    /// A layout payload for identity 7 with the given stamps and frame
    /// viewport; the rectangles are irrelevant to the currency rules.
    fn layout(
        identity: u64,
        revision: u64,
        frame: u64,
        client_width: f32,
        client_height: f32,
        scale_factor: f32,
    ) -> WindowControlLayout {
        WindowControlLayout {
            drag: Vec::new(),
            passthrough: Vec::new(),
            identity,
            frame,
            client_size: crate::size(Pixels(client_width), Pixels(client_height)),
            scale_factor,
            metric_revision: revision,
        }
    }

    /// The currency decision's fixed facts: session identity 7, current
    /// revision 3, an 800×600 physical client at scale 1.0, nothing
    /// applied yet.
    fn currency(payload: &WindowControlLayout) -> Result<(), StaleLayoutReason> {
        payload_currency(payload, 7, 3, rect(0, 0, 800, 600), 1.0, None)
    }

    #[test]
    fn a_matching_payload_is_accepted_and_stale_ones_rejected_with_reasons() {
        assert_eq!(currency(&layout(7, 3, 4, 800., 600., 1.0)), Ok(()));
        assert_eq!(
            currency(&layout(8, 3, 4, 800., 600., 1.0)),
            Err(StaleLayoutReason::Identity),
            "a payload from another identity at a reused HWND never applies"
        );
        assert_eq!(
            currency(&layout(7, 2, 4, 800., 600., 1.0)),
            Err(StaleLayoutReason::Revision),
            "a payload stamped before a metric invalidation never applies"
        );
        // Frame ordering needs an applied frame to compare against; the
        // dedicated test below covers it.
    }

    #[test]
    fn a_payload_stale_against_the_actual_extent_or_scale_is_rejected() {
        // The frame was drawn under a different viewport than the
        // window's ACTUAL client extent — for example a size change that
        // reentered mid-draw and would otherwise be stamped current at
        // publication.
        assert_eq!(
            currency(&layout(7, 3, 4, 640., 600., 1.0)),
            Err(StaleLayoutReason::Extent),
        );
        // Sub-physical-pixel rounding from the logical/physical
        // conversion is not a mismatch.
        assert_eq!(currency(&layout(7, 3, 4, 799.6, 600., 1.0)), Ok(()));
        // Same implied physical extent (640×480 at 1.25 = 800×600), a
        // different DPI scale than the window's actual one.
        assert_eq!(
            payload_currency(
                &layout(7, 3, 4, 640., 480., 1.25),
                7,
                3,
                rect(0, 0, 800, 600),
                1.0,
                None
            ),
            Err(StaleLayoutReason::Scale),
            "a payload from another DPI scale never applies against the actual scale"
        );
    }

    #[test]
    fn an_older_frame_never_replaces_a_newer_applied_plan() {
        let payload = layout(7, 3, 4, 800., 600., 1.0);
        assert_eq!(
            payload_currency(&payload, 7, 3, rect(0, 0, 800, 600), 1.0, Some(4)),
            Ok(()),
            "the same frame as the applied plan may re-apply; the dedupe decides"
        );
        assert_eq!(
            payload_currency(&payload, 7, 3, rect(0, 0, 800, 600), 1.0, Some(5)),
            Err(StaleLayoutReason::Frame),
        );
    }

    #[test]
    fn a_second_queued_apply_with_no_new_payload_retains_the_applied_plan() {
        let payload = layout(7, 3, 4, 800., 600., 1.0);
        // Two draws each queued an apply before the host drove the
        // inbox; the mailbox holds the latest frame's payload and the
        // first command applies it.
        assert_eq!(
            queued_layout_action(false, Some(&payload), 7, 3, rect(0, 0, 800, 600), 1.0, None),
            LayoutAction::ApplyPayload,
        );
        // The second command finds nothing stored: nothing new was
        // published, so the applied plan stands. Suspending here — the
        // pre-correction behavior — cleared the still-valid drag regions.
        assert_eq!(
            queued_layout_action(false, None, 7, 3, rect(0, 0, 800, 600), 1.0, Some(4)),
            LayoutAction::RetainCurrent,
        );
    }

    #[test]
    fn a_refresh_queued_behind_an_apply_never_clears_its_plan() {
        let payload = layout(7, 3, 4, 800., 600., 1.0);
        // The refresh reads the current payload and applies it.
        assert_eq!(
            queued_layout_action(true, Some(&payload), 7, 3, rect(0, 0, 800, 600), 1.0, None),
            LayoutAction::ApplyPayload,
        );
        // The apply queued behind it finds nothing new and retains the
        // plan the refresh applied.
        assert_eq!(
            queued_layout_action(false, None, 7, 3, rect(0, 0, 800, 600), 1.0, Some(4)),
            LayoutAction::RetainCurrent,
        );
        // A refresh with nothing stored under an outstanding metric
        // invalidation suspends — the one command kind allowed to.
        assert_eq!(
            queued_layout_action(true, None, 7, 3, rect(0, 0, 800, 600), 1.0, Some(4)),
            LayoutAction::ApplySuspension { reason: None },
        );
    }

    #[test]
    fn a_real_revision_change_suspends_until_the_next_current_frame() {
        // The stored payload predates the bump: stale for BOTH command
        // kinds, with the reason carried for the rejection log.
        let stale = layout(7, 2, 4, 800., 600., 1.0);
        assert_eq!(
            queued_layout_action(false, Some(&stale), 7, 3, rect(0, 0, 800, 600), 1.0, None),
            LayoutAction::ApplySuspension {
                reason: Some(StaleLayoutReason::Revision),
            },
        );
        assert_eq!(
            queued_layout_action(true, Some(&stale), 7, 3, rect(0, 0, 800, 600), 1.0, None),
            LayoutAction::ApplySuspension {
                reason: Some(StaleLayoutReason::Revision),
            },
        );
    }

    #[test]
    fn an_explicit_empty_frame_is_a_real_plan_not_missing_data() {
        // A current payload with no drag rectangles — the frame drew no
        // control areas — is accepted as a plan, never read as "no
        // data": the distinction from a missing payload is the point.
        let empty = layout(7, 3, 5, 800., 600., 1.0);
        assert_eq!(
            queued_layout_action(
                false,
                Some(&empty),
                7,
                3,
                rect(0, 0, 800, 600),
                1.0,
                Some(4)
            ),
            LayoutAction::ApplyPayload,
        );
        let metrics = RegionMetrics {
            caption_height: 32,
            left_inset: 10,
            right_inset: 50,
        };
        let plan = plan_regions(rect(0, 0, 800, 600), Some(metrics), &[], &[], &[]);
        assert!(plan.caption.is_empty());
        assert_eq!(
            plan.passthrough,
            vec![rect(0, 32, 800, 568), rect(10, 0, 740, 32)],
            "the empty frame's plan routes everything but the measured \
             reservations as client"
        );
    }

    #[test]
    fn an_explicit_passthrough_annotation_never_restores_reserved_geometry() {
        let client = rect(0, 0, 800, 600);
        let metrics = RegionMetrics {
            caption_height: 32,
            left_inset: 10,
            right_inset: 50,
        };
        let close_button = rect(744, 4, 44, 24);
        // One annotation crosses the LEFT measured inset strip, another
        // crosses the SDK's close button AND the right inset strip; the
        // drag crosses both the button and the right strip too.
        let drag = rect(100, 0, 700, 32);
        let annotations = [rect(0, 0, 20, 32), rect(740, 0, 60, 32)];
        let plan = plan_regions(
            client,
            Some(metrics),
            &[drag],
            &annotations,
            &[close_button],
        );
        let reservations = [close_button, rect(0, 0, 10, 32), rect(750, 0, 50, 32)];
        // NO planned rectangle of either kind may overlap a native
        // button or a measured inset strip.
        for planned in plan.caption.iter().chain(plan.passthrough.iter()) {
            for reservation in &reservations {
                assert!(
                    planned.intersect(reservation).is_none(),
                    "planned {planned:?} overlaps reserved {reservation:?}"
                );
            }
        }
        assert_eq!(
            plan.caption,
            vec![
                rect(100, 0, 640, 4),
                rect(100, 4, 640, 24),
                rect(100, 28, 640, 4),
            ],
            "the drag clips to the band around the button, the right inset \
             strip and the annotations"
        );
        assert_eq!(
            plan.passthrough,
            vec![
                rect(0, 32, 800, 568),
                rect(10, 0, 10, 32),
                rect(20, 0, 80, 4),
                rect(20, 4, 80, 24),
                rect(20, 28, 80, 4),
                rect(740, 0, 10, 4),
                rect(740, 4, 4, 24),
                rect(740, 28, 10, 4),
            ],
            "the annotations route as client only OUTSIDE the reservations: \
             the pre-correction behavior re-added them raw and restored \
             client routing over the button and both inset strips"
        );
    }

    #[test]
    fn unusable_metrics_plan_no_caption_and_keep_client_passthrough() {
        let client = rect(0, 0, 800, 600);
        let rail = rect(0, 0, 100, 600);
        let plan = plan_regions(
            client,
            None,
            &[rect(100, 0, 650, 32)],
            &[rail],
            &[rect(700, 0, 50, 32)],
        );
        assert!(
            plan.caption.is_empty(),
            "no usable measurement: no native drag geometry may be planned"
        );
        assert_eq!(
            plan.passthrough,
            vec![rect(0, 0, 100, 600)],
            "the annotated passthrough survives, untouched by the SDK rectangle \
             that lies entirely beyond it"
        );
    }

    #[test]
    fn the_caption_is_the_band_clipped_drag_with_reservations_and_exclusions() {
        let client = rect(0, 0, 800, 600);
        let metrics = RegionMetrics {
            caption_height: 32,
            left_inset: 10,
            right_inset: 50,
        };
        let rail = rect(0, 0, 100, 600);
        // The frame's drag spans the whole measured band below the rail.
        let drag = rect(100, 0, 700, 32);
        let close_button = rect(744, 4, 44, 24);
        let plan = plan_regions(client, Some(metrics), &[drag], &[rail], &[close_button]);
        assert_eq!(
            plan.caption,
            vec![
                rect(100, 0, 650, 4),
                rect(100, 4, 644, 24),
                rect(100, 28, 650, 4),
            ],
            "the drag clips to the band, reserves both measured inset strips and \
             the rail's passthrough, and excludes the SDK's own button rectangle"
        );
        assert_eq!(
            plan.passthrough,
            vec![
                rect(0, 32, 100, 568),
                rect(10, 0, 90, 32),
                rect(100, 32, 700, 568),
            ],
            "the rail routes as client EXCEPT the measured left inset strip it \
             crosses (the pre-correction behavior re-added it raw over the \
             strip), the whole area below the measured band routes as client, \
             and the caption/passthrough kinds stay disjoint"
        );
        for caption_rect in &plan.caption {
            for passthrough_rect in &plan.passthrough {
                assert!(
                    caption_rect.intersect(passthrough_rect).is_none(),
                    "kinds overlap at {caption_rect:?} vs {passthrough_rect:?}"
                );
            }
        }
    }

    #[test]
    fn an_overlay_suppressed_drag_leaves_only_safe_client_routing() {
        let client = rect(0, 0, 800, 600);
        let metrics = RegionMetrics {
            caption_height: 32,
            left_inset: 0,
            right_inset: 50,
        };
        // Every drag rectangle suppressed by an overlay: no caption at all.
        let plan = plan_regions(client, Some(metrics), &[], &[], &[]);
        assert!(
            plan.caption.is_empty(),
            "a suppressed drag rectangle is never replaced by a guessed caption"
        );
        assert_eq!(
            plan.passthrough,
            vec![rect(0, 0, 750, 32), rect(0, 32, 800, 568)],
            "the nonreserved band AND the whole area below it route as safe \
             client passthrough while the measured right inset stays reserved"
        );
    }

    #[test]
    fn an_empty_sdk_region_array_still_reserves_the_measured_insets() {
        let client = rect(0, 0, 400, 300);
        let metrics = RegionMetrics {
            caption_height: 32,
            left_inset: 0,
            right_inset: 150,
        };
        let plan = plan_regions(
            client,
            Some(metrics),
            &[rect(0, 0, 400, 32)],
            &[],
            // The SDK supplied NO button rectangles for its own buttons.
            &[],
        );
        assert_eq!(
            plan.caption,
            vec![rect(0, 0, 250, 32)],
            "an empty SDK region array is not proof the buttons have no \
             geometry: the measured right inset strip stays reserved"
        );
    }

    #[test]
    fn a_nonpositive_title_height_plans_no_caption() {
        let client = rect(0, 0, 400, 300);
        let plan = plan_regions(
            client,
            Some(RegionMetrics {
                caption_height: 0,
                left_inset: 0,
                right_inset: 0,
            }),
            &[rect(0, 0, 400, 32)],
            &[],
            &[],
        );
        assert!(plan.caption.is_empty());
    }
}
