use windows::{
    Win32::{
        Foundation::*, Graphics::Gdi::*, System::LibraryLoader::GetModuleHandleW,
        UI::WindowsAndMessaging::*,
    },
    core::{Error, w},
};

fn main() -> windows::core::Result<()> {
    println!("Hello, world!");
    let class_name = w!("MyWindowClass");
    let window_name = w!("Rust Window Sample");
    let position = (CW_USEDEFAULT, CW_USEDEFAULT);
    let (width, height) = (1920, 1080);

    unsafe {
        // 1. インスタンスハンドルの取得
        let instance = GetModuleHandleW(None)?.into();

        // 2. ウィンドウクラスの定義
        let window_class = class_name;
        let wc: WNDCLASSW = WNDCLASSW {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: instance,
            lpszClassName: window_class,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        // 3. ウィンドウクラスの登録
        if RegisterClassW(&wc) == 0 {
            return Err(Error::from_thread());
        }

        // 4. ウィンドウの作成
        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            window_class,
            window_name,
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            position.0,
            position.1,
            width,
            height,
            None,
            None,
            Some(instance),
            None,
        )?;

        if hwnd.is_invalid() {
            return Err(Error::from_thread());
        }


        // 5. メッセージループ

        // let mut msg = MSG::default();
        // while GetMessageW(&mut msg, None, 0, 0).as_bool() {
        //     let _ = TranslateMessage(&msg);
        //     DispatchMessageW(&msg);
        // }

        // GetMessageWによるブロックが嫌な場合は、PeekMessageWを使う
        'mainloop: loop {
            let mut msg = MSG::default();
            while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
                if msg.message == WM_QUIT {
                    break 'mainloop;
                }
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }

    Ok(())
}

#[allow(non_snake_case)]
extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(window, &mut ps);
                FillRect(
                    hdc,
                    &ps.rcPaint,
                    HBRUSH((COLOR_WINDOW.0 + 1) as *mut core::ffi::c_void),
                );
                LRESULT(0)
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(window, message, wparam, lparam),
        }
    }
}
