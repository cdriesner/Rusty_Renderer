#![warn(unused_imports)]

use std::sync::Once;

use windows::{
  core::{w, ComInterface, Result, HSTRING, PCWSTR},
  Foundation::Numerics::Vector2,
  Graphics::SizeInt32,
  Win32::{
    Foundation::{HWND, LPARAM, LRESULT, RECT, WPARAM, COLORREF},
    System::{LibraryLoader::GetModuleHandleW, WinRT::Composition::ICompositorDesktopInterop},
    UI::WindowsAndMessaging::{
      AdjustWindowRectEx, CreateWindowExW, DefWindowProcW, GetClientRect, GetWindowLongPtrW,
      LoadCursorW, PostQuitMessage, RegisterClassW, SetWindowLongPtrW, ShowWindow,
      CREATESTRUCTW, CW_USEDEFAULT, GWLP_USERDATA, IDC_ARROW, SW_SHOW, WM_DESTROY,
      WM_LBUTTONDOWN, WM_MOUSEMOVE, WM_CREATE, WM_NCCREATE, WM_RBUTTONDOWN, WM_SIZE, WM_SIZING,
      WNDCLASSW, WS_EX_NOREDIRECTIONBITMAP, WS_OVERLAPPEDWINDOW, WM_PAINT, WS_EX_OVERLAPPEDWINDOW, WM_KEYDOWN,
    }, Graphics::Gdi::{PAINTSTRUCT, HDC, BeginPaint, FillRect, HBRUSH, EndPaint, COLOR_WINDOW, SYS_COLOR_INDEX, CreatePen, PS_SOLID, MoveToEx, SelectObject, LineTo, HGDIOBJ, DeleteObject, LOGBRUSH, BS_SOLID, ExtCreatePen, PS_COSMETIC, UpdateWindow, RedrawWindow, RDW_FRAME, InvalidateRect},
  },
  UI::Composition::{Compositor, Desktop::DesktopWindowTarget},
};

use crate::{ draw_line, SCENE};

static REGISTER_WINDOW_CLASS: Once = Once::new();
const WINDOW_CLASS_NAME: PCWSTR = w!("minesweeper-rs.Window");


#[derive(Clone,Copy)]
pub struct Window {
  handle: HWND,
}

impl Window {
  pub fn new(title: &str, width: u32, height: u32) -> Result<Box<Self>> {
    let instance = unsafe { GetModuleHandleW(None)? };
    REGISTER_WINDOW_CLASS.call_once(|| {
      let class = WNDCLASSW {
        hCursor: unsafe { LoadCursorW(None, IDC_ARROW).ok().unwrap() },
        hInstance: instance.into(),
        lpszClassName: WINDOW_CLASS_NAME,
        lpfnWndProc: Some(Self::wnd_proc),
        ..Default::default()
      };
      assert_ne!(unsafe { RegisterClassW(&class) }, 0);
    });
    let window_ex_style = WS_EX_OVERLAPPEDWINDOW;
    let window_style = WS_OVERLAPPEDWINDOW;
    let (adjusted_width, adjusted_height) = {
      let mut rect = RECT {
        left: 0,
        top: 0,
        right: width as i32,
        bottom: height as i32,
      };
      unsafe {
        AdjustWindowRectEx(&mut rect, window_style, false, window_ex_style)?;
      }
      (rect.right - rect.left, rect.bottom - rect.top)
    };

    let mut result = Box::new(Self {
      handle: HWND(0),
    });

    let window = unsafe {
      CreateWindowExW(
        window_ex_style,
        WINDOW_CLASS_NAME,
        &HSTRING::from(title),
        window_style,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        adjusted_width,
        adjusted_height,
        None,
        None,
        instance,
        Some(result.as_mut() as *mut _ as _),
      )
    };
    unsafe { ShowWindow(window, SW_SHOW) };

    Ok(result)
    }

    pub fn size(&self) -> Result<SizeInt32> {
      get_window_size(self.handle)
    }

  pub fn handle(&self) -> HWND {
    self.handle
  }

  pub fn create_window_target(
    &self,
    compositor: &Compositor,
    is_topmost: bool,
  ) -> Result<DesktopWindowTarget> {
    let compositor_desktop: ICompositorDesktopInterop = compositor.cast()?;
    unsafe { compositor_desktop.CreateDesktopWindowTarget(self.handle(), is_topmost) }
  }

  pub fn paint_window(&self) {
    unsafe{
      InvalidateRect(self.handle, None, false);
      UpdateWindow(self.handle);
    };
  }

  pub fn redraw_window(&self) {
    unsafe{RedrawWindow(self.handle, None, None, RDW_FRAME)};
  }


  fn message_handler(&mut self, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match message {
      WM_NCCREATE => {
        println!("NC Create");
        LRESULT(1);
      }
      WM_CREATE => {
        println!("Create");
      }
      WM_DESTROY => {
        unsafe { PostQuitMessage(0) };
        LRESULT(0);
      }
      WM_MOUSEMOVE => {
        let (x, y) = get_mouse_position(lparam);
        //println!("x:{}, y:{}",x,y);
        let _point = Vector2 {
          X: x as f32,
          Y: y as f32,
        };
      }
      WM_SIZE | WM_SIZING => {
        let new_size = self.size().unwrap();
        let _new_size = Vector2 {
          X: new_size.Width as f32,
          Y: new_size.Height as f32,
        };
      }
      WM_PAINT => {
        let mut ps: PAINTSTRUCT = PAINTSTRUCT::default();
        let hdc: HDC = unsafe {BeginPaint(self.handle, &mut ps as *mut PAINTSTRUCT)};
        let hbrush = HBRUSH::default();
        unsafe {FillRect(hdc, &ps.rcPaint,hbrush)};
        unsafe { SCENE.render(hdc) };
        unsafe {EndPaint(self.handle, &ps)};
      }
      WM_LBUTTONDOWN => {
      }
      WM_RBUTTONDOWN => {
      }
      WM_KEYDOWN => {
        
      }
      _ => {}
    }
    unsafe { DefWindowProcW(self.handle, message, wparam, lparam) }
  }

  unsafe extern "system" fn wnd_proc(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
  ) -> LRESULT {
    if message == WM_NCCREATE {
        let cs = lparam.0 as *const CREATESTRUCTW;
        let this = (*cs).lpCreateParams as *mut Self;
        (*this).handle = window;
        SetWindowLongPtrW(window, GWLP_USERDATA, this as _);
    } else {
      let this = GetWindowLongPtrW(window, GWLP_USERDATA) as *mut Self;
      if let Some(this) = this.as_mut() {
        return this.message_handler(message, wparam, lparam);
      }
    }
    DefWindowProcW(window, message, wparam, lparam)
  }
}

fn get_window_size(window_handle: HWND) -> Result<SizeInt32> {
  unsafe {
    let mut rect = RECT::default();
    GetClientRect(window_handle, &mut rect)?;
    let width = rect.right - rect.left;
    let height = rect.bottom - rect.top;
    Ok(SizeInt32 {
      Width: width,
      Height: height,
    })
  }
}

fn get_mouse_position(lparam: LPARAM) -> (isize, isize) {
  let x = lparam.0 & 0xffff;
  let y = (lparam.0 >> 16) & 0xffff;
  (x, y)
}