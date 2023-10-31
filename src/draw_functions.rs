use windows::{Win32::{Graphics::Gdi::{CreatePen, SelectObject, MoveToEx, LineTo, DeleteObject, HGDIOBJ, PS_SOLID, HDC, HPEN, CreateSolidBrush, HBRUSH, Polygon}, Foundation::{COLORREF,POINT}}};

use crate::Triangle;

pub fn draw_line(hdc:HDC, start_x:f32, start_y:f32, end_x:f32, end_y:f32){
  let rounded_start_x:i32 = unsafe{start_x.to_int_unchecked::<i32>()};
  let rounded_start_y:i32 = unsafe{start_y.to_int_unchecked::<i32>()};
  let rounded_end_x:i32 = unsafe{end_x.to_int_unchecked::<i32>()};
  let rounded_end_y:i32 = unsafe{end_y.to_int_unchecked::<i32>()};
  let pen = unsafe{CreatePen(PS_SOLID, 2, COLORREF(0x00000000))};
  let h_pen_old: HGDIOBJ = unsafe{SelectObject(hdc, pen)};
  unsafe{MoveToEx(hdc, rounded_start_x, rounded_start_y, None)};
  unsafe{LineTo(hdc, rounded_end_x, rounded_end_y)};
  unsafe { SelectObject(hdc, h_pen_old) };
  unsafe{ DeleteObject(pen) };
}

pub fn draw_triangle(hdc:HDC, tri:&Triangle){
  draw_line(hdc, tri.p[0].x, tri.p[0].y, tri.p[1].x, tri.p[1].y);
  draw_line(hdc, tri.p[1].x, tri.p[1].y, tri.p[2].x, tri.p[2].y);
  draw_line(hdc, tri.p[2].x, tri.p[2].y, tri.p[0].x, tri.p[0].y);
}

pub fn draw_filled_triangle(hdc:HDC, tri:&Triangle){
  
  let mut points:[POINT; 3] = unsafe{[
    POINT { x:tri.p[0].x.to_int_unchecked::<i32>(), y:tri.p[0].y.to_int_unchecked::<i32>() },
    POINT { x:tri.p[1].x.to_int_unchecked::<i32>(), y:tri.p[1].y.to_int_unchecked::<i32>() },
    POINT { x:tri.p[2].x.to_int_unchecked::<i32>(), y:tri.p[2].y.to_int_unchecked::<i32>() }
  ]};
  let h_pen:HPEN = unsafe { CreatePen(PS_SOLID, 2, COLORREF(tri.col.try_into().unwrap())) };
  let h_old_pen:HGDIOBJ  = unsafe {SelectObject(hdc, h_pen)};
  let h_brush:HBRUSH  = unsafe { CreateSolidBrush(COLORREF(tri.col.try_into().unwrap())) };
  let h_old_brush:HGDIOBJ = unsafe { SelectObject(hdc, h_brush) };
  unsafe { Polygon(hdc, &points) };
  unsafe {
    SelectObject(hdc, h_old_pen);
    DeleteObject(h_pen);
    SelectObject(hdc, h_old_brush);
    DeleteObject(h_brush);
  }
}