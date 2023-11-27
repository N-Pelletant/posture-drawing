use nwd::NwgUi;
use nwg::RadioButtonState::Checked;
use std::{cell::RefCell, ffi::OsString, time::Duration};

pub const WIDTH: u32 = 300;
pub const HEIGHT: u32 = 250;
const DEFAULT_DURATION: u64 = 30;

#[derive(Default, NwgUi)]
pub struct SpeedDrawingApp {
    pub loaded_image: RefCell<Option<nwg::Bitmap>>,
    pub image_list: RefCell<Option<Vec<OsString>>>,
    pub image_indices: RefCell<Option<Vec<usize>>>,
    pub x_pos: RefCell<Option<i32>>,
    pub y_pos: RefCell<Option<i32>>,
    pub duration: RefCell<Option<Duration>>,
    pub time_left: RefCell<Option<u64>>,
    pub is_running: RefCell<Option<bool>>,

    /**
     * RESOURCES
     **/
    #[nwg_resource(family: "Segoe UI", size: 18)]
    pub font: nwg::Font,

    #[nwg_resource]
    pub decoder: nwg::ImageDecoder,

    #[nwg_resource(
        title: "Sélectionner un dossier",
        action: nwg::FileDialogAction::OpenDirectory,
    )]
    pub dialog: nwg::FileDialog,

    /** GENERAL LAYOUT **/
    #[nwg_control(
        title: "Posture drawing",
        size: (
            WIDTH.try_into().unwrap(),
            HEIGHT.try_into().unwrap()
        ),
        center: true,
    )]
    #[nwg_events(OnWindowClose: [SpeedDrawingApp::exit])]
    pub window: nwg::Window,

    #[nwg_layout(parent: window, max_row: Some(12), max_column: Some(12))]
    pub main_layout: nwg::GridLayout,

    /**
     * CONTROL CONTENT
     **/
    #[nwg_control(text: "Choisir le dossier")]
    #[nwg_layout_item(layout: main_layout, col: 0, col_span:6, row: 0, row_span: 2)]
    #[nwg_events(OnButtonClick: [SpeedDrawingApp::choose_directory_click])]
    pub choose_directory_button: nwg::Button,

    #[nwg_control(readonly: true)]
    #[nwg_layout_item(layout: main_layout, col: 6, col_span:6, row: 0, row_span: 2)]
    pub directory_input: nwg::TextInput,

    #[nwg_control(text: "Timer")]
    #[nwg_layout_item(layout: main_layout, col: 0, col_span: 6, row: 2, row_span: 8)]
    pub timer_select_label: nwg::Label,

    #[nwg_control(text: "00:30", check_state: Checked)]
    #[nwg_layout_item(layout: main_layout, col: 6, col_span: 6, row: 2, row_span: 2)]
    #[nwg_events(OnButtonClick: [SpeedDrawingApp::timer_radio_click])]
    pub timer_radio_00_30: nwg::RadioButton,

    #[nwg_control(text: "01:00")]
    #[nwg_layout_item(layout: main_layout, col: 6, col_span: 6, row: 4, row_span: 2)]
    #[nwg_events(OnButtonClick: [SpeedDrawingApp::timer_radio_click])]
    pub timer_radio_01_00: nwg::RadioButton,

    #[nwg_control(text: "02:00")]
    #[nwg_layout_item(layout: main_layout, col: 6, col_span: 6, row: 6, row_span: 2)]
    #[nwg_events(OnButtonClick: [SpeedDrawingApp::timer_radio_click])]
    pub timer_radio_02_00: nwg::RadioButton,

    #[nwg_control(text: "05:00")]
    #[nwg_layout_item(layout: main_layout, col: 6, col_span: 6, row: 8, row_span: 2)]
    #[nwg_events(OnButtonClick: [SpeedDrawingApp::timer_radio_click])]
    pub timer_radio_05_00: nwg::RadioButton,

    #[nwg_control(text: "Commencer", enabled: false)]
    #[nwg_layout_item(layout: main_layout, col: 3, col_span: 6, row: 10, row_span: 2)]
    #[nwg_events(OnButtonClick: [SpeedDrawingApp::start_drawing_click])]
    pub start_drawing_button: nwg::Button,

    /**
     * DRAWING CONTENT
     **/
    #[nwg_control]
    #[nwg_layout_item(
        layout: main_layout,
        col: 0,
        col_span: 12,
        row: 0,
        row_span: 11,
    )]
    pub image_container: nwg::ImageFrame,

    #[nwg_control(text: "Retour")]
    #[nwg_layout_item(layout: main_layout, col: 0, row: 11)]
    #[nwg_events(OnButtonClick: [SpeedDrawingApp::back_to_control_click])]
    pub back_to_control_btn: nwg::Button,

    #[nwg_control(text: "00:00")]
    #[nwg_layout_item(layout: main_layout, col: 6, col_span:1, row: 11)]
    pub time_left_label: nwg::Label,

    #[nwg_control(text: "Play/Pause")]
    #[nwg_layout_item(layout: main_layout, col: 10, row: 11)]
    #[nwg_events(OnButtonClick: [SpeedDrawingApp::play_pause_click])]
    pub play_pause_btn: nwg::Button,

    #[nwg_control(text: "Skip")]
    #[nwg_layout_item(layout: main_layout, col: 11, row: 11)]
    #[nwg_events(OnButtonClick: [SpeedDrawingApp::skip_click])]
    pub skip_btn: nwg::Button,

    /** TIMERS */
    #[nwg_control(
        parent: window,
        interval: Duration::from_secs(1),
        active: false,
    )]
    #[nwg_events(
        OnTimerTick: [SpeedDrawingApp::chrono_timer_tick],
    )]
    pub chrono_timer: nwg::AnimationTimer,
}

impl SpeedDrawingApp {
    pub fn init(&self) {
        self.duration
            .borrow_mut()
            .replace(Duration::from_secs(DEFAULT_DURATION));
        self.time_left.borrow_mut().replace(DEFAULT_DURATION);
        self.is_running.borrow_mut().replace(false);

        self.stop_chrono_timer();

        self.init_layout();
    }

    pub fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}
