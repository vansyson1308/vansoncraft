use super::{canvas, cursors, render_worker};

/// Cached canvas raster.
pub struct CanvasCache {
    pub renderer: vansoncraft_render::Renderer,
    pub texture: Option<egui::TextureHandle>,
    pub key: Option<CacheKey>,
    /// The document the frame shown was rendered from, and the options it was rendered with, with
    /// [`Self::key`]: when only the document changes, the canvas renders again only where it
    /// changed, measured by [`Self::renderer`], which drew it ([`vansoncraft_render::Renderer::changed`]).
    pub shown: Option<(std::sync::Arc<vansoncraft_doc::Document>, vansoncraft_render::RenderOptions)>,
    pub last_ms: f64,
    /// The frame shown drew raster effects from rasters made before their objects moved, or at a
    /// drag's lower resolution ([`vansoncraft_render::FrameStats::moved`]): the next frame outside
    /// an interaction runs them again, on the worker when there is one.
    pub moved: bool,
    /// The frame shown was rendered on the worker: an interaction's frames stay there
    /// ([`canvas::show`]).
    pub from_worker: bool,
    /// The frame shown holds pixels the worker copied from its last frame, and when it was shown
    /// (seconds, egui's input time): it renders whole once it has been shown for
    /// [`canvas::STILL_SECS`] with the view unmoved.
    pub reused: Option<f64>,
    /// Set in the web build, which has no render worker: while the view changes on a document whose
    /// last render took more than 8 ms, the canvas draws the frame shown reprojected and renders
    /// once the view has been still for [`canvas::STILL_SECS`].
    pub waits_for_still: bool,
    /// The cache key the canvas last asked for and when it first asked for it (seconds, egui's
    /// input time), kept while [`Self::waits_for_still`] is set.
    pub still: Option<(CacheKey, f64)>,
    /// The low-resolution frame that covers the parts of the canvas the frame shown leaves
    /// uncovered while it is drawn for a view it was not rendered for ([`canvas::Fill`]).
    pub fill: Option<canvas::Fill>,
    /// The thread that renders [`Self::fill`], beside [`Self::worker`]. None on wasm, which has no
    /// fill.
    pub fill_worker: Option<render_worker::Worker>,
    pub worker: Option<render_worker::Worker>,
    pub worker_started: bool,
    /// The slices as laid out for (document uid, revision): the canvas draws them every frame.
    pub slices: Option<SliceCache>,
    /// The print tiling's pages for (document uid, revision) (View → Show Print Tiling).
    pub print_tiling: Option<PrintTilingCache>,
    /// The gap lines View → Show Live Paint Gaps draws for (document uid, revision).
    pub live_paint_gaps: Option<LivePaintGapsCache>,
    /// The marks Type → Show Hidden Characters drew on each text object in the last frame. A frame
    /// that does not draw them drops them ([`canvas::show`]).
    pub hidden_chars: canvas::HiddenChars,
    /// The words Edit › Spelling › Auto Spell Check underlined on each text object in the last
    /// frame; dropped while it is off.
    pub spelling: canvas::SpellingMarks,
    /// The selected objects' outlines, anchors and handles as drawn in the last frame. A frame that
    /// does not draw them drops them ([`canvas::show`]).
    pub selection_marks: canvas::SelectionMarks,
    /// [`VansoncraftApp::selection_box`] for (document uid, revision, Use Preview Bounds and
    /// Measure Type by Glyph Bounds).
    pub selection_box: Option<((u64, u64, vansoncraft_effects::preview::Measure), Option<vansoncraft_doc::OrientedBox>)>,
    /// [`VansoncraftApp::selection_bounds`] for (document uid, revision).
    pub selection_bounds: Option<((u64, u64), Option<vansoncraft_geom::Rect>)>,
    /// [`VansoncraftApp::content_bounds`] for (document uid, revision).
    pub content_bounds: Option<((u64, u64), Option<vansoncraft_geom::Rect>)>,
    /// [`CacheKey::plugins`] by document uid, with the plug-in revision
    /// ([`vansoncraft_render::effects::plugin_revision`]) it was last checked at.
    pub plugin_keys: std::collections::HashMap<u64, (u64, u64)>,
    /// The tools' cursors as OS cursor bitmaps.
    pub cursors: cursors::Images,
    /// The view the canvas last drew and when it last changed (egui's input time): while it moves,
    /// the canvas doesn't hit-test the art under the pointer ([`canvas::hover_wait`], #2452).
    pub view_moved: Option<(canvas::ViewKey, f64)>,
    /// The cursor the active tool last gave over the canvas, with the tool and the modifier keys
    /// it was for: kept while the view moves.
    pub hover_cursor: Option<(&'static str, vansoncraft_tools::Mods, vansoncraft_tools::Cursor)>,
    /// In Presentation Mode, the canvas rect the active artboard was last fitted to: it is fitted
    /// again when the rect changes (the window going full screen).
    pub presented: Option<egui::Rect>,
    /// The canvas a document's opening view was fitted to, with the view the fit gave. The window's
    /// layout can still move the canvas's edges over the next frames, so until the view is moved or
    /// the user acts, the opening view is fitted again when the canvas changes ([`canvas::show`]).
    pub opening_fit: Option<canvas::ViewKey>,
    /// Window › Arrange › Tile: the renders the panes of the other documents draw.
    pub panes: canvas::Panes,
    /// Window › Arrange › Float in Window: what the floating document windows keep between frames.
    pub windows: crate::doc_windows::Windows,
}

/// [`CanvasCache::slices`]: the layout of the slices of (document uid, revision).
pub type SliceCache = ((u64, u64), std::sync::Arc<Vec<vansoncraft_doc::SliceArea>>);

/// [`CanvasCache::live_paint_gaps`]: the gap lines of the selected Live Paint groups of (document
/// uid, revision).
pub type LivePaintGapsCache = ((u64, u64), std::sync::Arc<Vec<vansoncraft_tools::Overlay>>);

/// [`CanvasCache::print_tiling`]: the print tiling of (document uid, revision).
pub type PrintTilingCache = ((u64, u64), std::sync::Arc<Vec<vansoncraft_pdf::TilingPage>>);

#[derive(Clone, Debug, PartialEq)]
pub struct CacheKey {
    pub doc: usize,
    pub revision: u64,
    /// An interaction is in progress ([`vansoncraft_render::RenderOptions::interacting`]).
    pub interacting: bool,
    /// A tool's drag on the canvas is in progress ([`vansoncraft_render::RenderOptions::dragging`]).
    pub dragging: bool,
    pub zoom: f64,
    pub cx: f64,
    pub cy: f64,
    pub w: u32,
    pub h: u32,
    pub outline: bool,
    pub trim: bool,
    /// View › Hide Template: template layers left out ([`vansoncraft_render::RenderOptions::hide_templates`]).
    pub templates: bool,
    /// View › Show/Hide Guides: whether guide paths are visible in the artwork raster.
    pub guides: bool,
    pub ppp: f32,
    pub hidden: Vec<u64>,
    pub rot: f64,
    /// General › Anti-aliased Artwork.
    pub anti_alias: bool,
    /// Type › Highlight Alternate Glyphs (#394).
    pub highlight_alternate_glyphs: bool,
    /// Type › Highlight Substituted Fonts.
    pub highlight_substituted_fonts: bool,
    /// [`vansoncraft_render::placed_document::generation`]: placed documents' bitmaps made since.
    pub placed: u64,
    /// The plug-in revision ([`vansoncraft_render::effects::plugin_revision`]) of the last plug-in
    /// install or removal, or of the last time deferred plug-in effect stages gave results, after
    /// which the document drew a plug-in effect ([`vansoncraft_render::draws_plugin_effects`]): a
    /// frame drawn before it draws those effects as the plug-ins installed then did. 0 until then.
    pub plugins: u64,
    /// View › Pixel Preview: the document pixels rendered (x0, y0, x1, y1), one per point, shown
    /// with hard edges. None: the art is rendered for the screen.
    pub pixel: Option<[i64; 4]>,
    /// Images sampled smoothly ([`vansoncraft_render::RenderOptions::smooth_images`]).
    pub smooth_images: bool,
    /// Isolation mode's group or layer ([`vansoncraft_render::RenderOptions::isolated`]).
    pub isolated: Option<u64>,
    /// Appearance of Black › On Screen › Display All Blacks as Rich Black
    /// ([`vansoncraft_render::RenderOptions::rich_black`]).
    pub rich_black: bool,
    /// Colour and style of guide paths ([`vansoncraft_render::RenderOptions::guide`]).
    pub guide: ([u8; 3], bool),
    /// The Image Trace objects drawn with their source image
    /// ([`vansoncraft_render::RenderOptions::trace_source`]).
    pub trace_source: Option<std::sync::Arc<Vec<vansoncraft_doc::NodeId>>>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Perf {
    pub frame_ms: f64,
    pub render_ms: f64,
    pub fps: f64,
}
