//! Convert a cursor into an iterator of rects.

use log::{debug, info};

use alacritty_terminal::vte::ansi::CursorShape;
// iksi4prs
use alacritty_terminal::selection::Selection;
use alacritty_terminal::selection::SelectionType;

use crate::display::color::Rgb;
use crate::display::content::RenderableCursor;
use crate::display::SizeInfo;
use crate::renderer::rects::RenderRect;

/// Trait for conversion into the iterator.
pub trait IntoRects {
    /// Consume the cursor for an iterator of rects.
    //fn rects(self, size_info: &SizeInfo, thickness: f32, selection_type: char) -> CursorRects;

    fn rects(self, size_info: &SizeInfo, thickness: f32, selection: &Option<Selection>) -> CursorRects;
    
}

impl IntoRects for RenderableCursor {
    //fn rects(self, size_info: &SizeInfo, thickness: f32, selection_type: char) -> CursorRects {
    fn rects(self, size_info: &SizeInfo, thickness: f32, selection: &Option<Selection>) -> CursorRects {
        let point = self.point();
        let x = point.column.0 as f32 * size_info.cell_width() + size_info.padding_x();
        let y = point.line as f32 * size_info.cell_height() + size_info.padding_y();

        let mut width = size_info.cell_width();
        let height = size_info.cell_height();

        let thickness = (thickness * width).round().max(1.);

        if self.is_wide() {
            width *= 2.;
        }

        match selection {
            Some(selection) => {
                info!("55555 START");
                let thickness_2 = 3.0;
                match selection.ty {
                    SelectionType::Simple => { info!("55555 Simple");default2()}, // TEMP
                    SelectionType::Block =>  { info!("55555 Block"); default2()}, // TEMP
                    SelectionType::Semantic => { info!("55555 Semantic"); selection_type_letter_s(x, y, width, height, thickness_2, self.color())},
                    SelectionType::Lines => { info!("55555 Lines"); selection_type_letter_l(x, y, width, height, thickness_2, self.color())},
                    _ =>  { info!("55555 underscore"); default2()}, // throw ???
                }
                //eturn selection_type_letter(selection, x, y, width, height, thickness, self.color());
            },
            None => {
                info!("8888002 (no selection, going to use orig cursors)");
                let red = Rgb::new(255,0,0);
                match self.shape() {
                    CursorShape::Beam => beam(x, y, height, thickness, self.color()),
                    CursorShape::Underline => underline(x, y, width, height, thickness, self.color()),
                    CursorShape::HollowBlock => hollow(x, y, width, height, thickness, self.color()),
                    // for CursorShape::Block, done by changing fg of cell, not drawing rects. see content.rs
                    //_ => hollow(x, y, width, height, thickness +1., red),
                    _ => CursorRects::default(),
                } 
            }
          }
        // //let is_selection_mode = true; // iksi4prs
        // if selection_type != '\0' {
        //     return selection_type_letter(x, y, width, height, thickness, self.color());
        // }
        // else {
          
        // }
    }
}

/// Cursor rect iterator.
#[derive(Default)]
pub struct CursorRects {
    rects: [Option<RenderRect>; 4],
    index: usize,
}

impl From<RenderRect> for CursorRects {
    fn from(rect: RenderRect) -> Self {
        Self { rects: [Some(rect), None, None, None], index: 0 }
    }
}

impl Iterator for CursorRects {
    type Item = RenderRect;

    fn next(&mut self) -> Option<Self::Item> {
        let rect = self.rects.get_mut(self.index)?;
        self.index += 1;
        rect.take()
    }
}

/// Create an iterator yielding a single beam rect.
fn beam(x: f32, y: f32, height: f32, thickness: f32, color: Rgb) -> CursorRects {
    RenderRect::new(x, y, thickness, height, color, 1.).into()
}

/// Create an iterator yielding a single underline rect.
fn underline(x: f32, y: f32, width: f32, height: f32, thickness: f32, color: Rgb) -> CursorRects {
    let y = y + height - thickness;
    RenderRect::new(x, y, width, thickness, color, 1.).into()
}

/// Create an iterator yielding a rect for each side of the hollow block cursor.
fn hollow(x: f32, y: f32, width: f32, height: f32, thickness: f32, color: Rgb) -> CursorRects {
    let top_line = RenderRect::new(x, y, width, thickness, color, 1.);

    let vertical_y = y + thickness;
    let vertical_height = height - 2. * thickness;
    let left_line = RenderRect::new(x, vertical_y, thickness, vertical_height, color, 1.);

    let bottom_y = y + height - thickness;
    let bottom_line = RenderRect::new(x, bottom_y, width, thickness, color, 1.);

    let right_x = x + width - thickness;
    let right_line = RenderRect::new(right_x, vertical_y, thickness, vertical_height, color, 1.);

    CursorRects {
        rects: [Some(top_line), Some(bottom_line), Some(left_line), Some(right_line)],
        index: 0,
    }
}

/// iksi4prs
// based on hollow
/*
fn selection_type_letter_NOT_USED(
    selection: &Option<Selection>,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    thickness: f32,
     color: Rgb
    ) -> CursorRects {
    
    let thickness_2 = 3.0;

    match selection {
        Some(selection) => {
            match selection.ty {
                SelectionType::Simple => CursorRects::default(), // TEMP
                SelectionType::Block => CursorRects::default(), // TEMP
                SelectionType::Semantic => selection_type_letter_s(x, y, width, height, thickness_2, color),
                SelectionType::Lines => selection_type_letter_l(x, y, width, height, thickness_2, color),
                _ => CursorRects::default(), // throw ???
            }
        },
        // throw ??
        None =>  CursorRects::default(),
      }

    // let mode = 'L';
    // match mode {
    //     'L' => selection_type_letter_l(x, y, width, height, thickness_2, color),
    //     _ => CursorRects::default(),
    // }
}
*/

fn selection_type_letter_l(x: f32, y: f32, width: f32, height: f32, thickness: f32, color: Rgb) -> CursorRects {
    
    //let top_line = RenderRect::new(x, y, width, thickness_2, color, 1.);

    let vertical_y = y + thickness;
    let vertical_height = height - 2. * thickness;
    let left_line = RenderRect::new(x, vertical_y, thickness, vertical_height, color, 1.);

    let bottom_y = y + height - thickness;
    let bottom_line = RenderRect::new(x, bottom_y, width, thickness, color, 1.);

    //let right_x = x + width - thickness;
    //let right_line = RenderRect::new(right_x, vertical_y, thickness, vertical_height, color, 1.);

    let empty = RenderRect::new(x, bottom_y, 0., 0., color, 1.);

    CursorRects {
        rects: [Some(left_line),Some(bottom_line), Some(empty), Some(empty)],
        index: 0,
    }
}

// just temp
fn selection_type_letter_s(x: f32, y: f32, width: f32, height: f32, thickness: f32, color: Rgb) -> CursorRects {
    
    //let top_line = RenderRect::new(x, y, width, thickness_2, color, 1.);

    let vertical_y = y + thickness;
    let vertical_height = height - 2. * thickness;
    let left_line = RenderRect::new(x, vertical_y, thickness, vertical_height, color, 1.);

    let bottom_y = y + height - thickness;
    let bottom_line = RenderRect::new(x, bottom_y, width, thickness, color, 1.);

    let right_x = x + width - thickness;
    let right_line = RenderRect::new(right_x, vertical_y, thickness, vertical_height, color, 1.);

    let empty = RenderRect::new(x, bottom_y, 0., 0., color, 1.);

    CursorRects {
        rects: [Some(left_line),Some(right_line), Some(empty), Some(empty)],
        index: 0,
    }
}

fn default2() -> CursorRects {
    //let y = y + height - thickness;
    info!("8888001 (default2)");
    //let color = Rgb::default();
    let color = Rgb::new(255, 0, 0);
    let alpha  = 1.;
    RenderRect::new(0., 0., 3., 3. , color, alpha).into()
}
