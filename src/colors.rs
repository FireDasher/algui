/// Converts a color with channels in a 0-1 range to a compact u32 color
pub const fn unorm(r: f32, g: f32, b: f32, a: f32) -> u32 {
	((r * 255.0) as u32) | ((g * 255.0) as u32)<<8 | ((b * 255.0) as u32)<<16 | ((a * 255.0) as u32)<<24
}

/// Based on ImGuiCol enum
pub struct Colors {
    pub text: u32,
    pub text_disabled: u32,
    pub window_bg: u32,                    // Background of normal windows
    pub child_bg: u32,                     // Background of child windows
    pub popup_bg: u32,                     // Background of popups, menus, tooltips windows
    pub border: u32,
    pub border_shadow: u32,
    pub frame_bg: u32,                     // Background of checkbox, radio button, plot, slider, text input
    pub frame_bg_hovered: u32,
    pub frame_bg_active: u32,
    pub title_bg: u32,                     // Title bar
    pub title_bg_active: u32,              // Title bar when focused
    pub title_bg_collapsed: u32,           // Title bar when collapsed
    pub menu_bar_bg: u32,
    pub scrollbar_bg: u32,
    pub scrollbar_grab: u32,
    pub scrollbar_grab_hovered: u32,
    pub scrollbar_grab_active: u32,
    pub check_mark: u32,                   // Checkbox tick and Radio_Button circle
    pub checkbox_selected_bg: u32,         // Checkbox background when Selected, otherwise use Frame_Bg
    pub slider_grab: u32,
    pub slider_grab_active: u32,
    pub button: u32,
    pub button_hovered: u32,
    pub button_active: u32,
    pub header: u32,                       // Header* colors are used for Collapsing_Header, Tree_Node, Selectable, Menu_Item
    pub header_hovered: u32,
    pub header_active: u32,
    pub separator: u32,
    pub separator_hovered: u32,
    pub separator_active: u32,
    pub resize_grip: u32,                  // Resize grip in lower-right and lower-left corners of windows.
    pub resize_grip_hovered: u32,
    pub resize_grip_active: u32,
    pub input_text_cursor: u32,            // Input_Text cursor/caret
    pub tab_hovered: u32,                  // Tab background, when hovered
    pub tab: u32,                          // Tab background, when tab-bar is focused & tab is unselected
    pub tab_selected: u32,                 // Tab background, when tab-bar is focused & tab is selected
    pub tab_selected_overline: u32,        // Tab horizontal overline, when tab-bar is focused & tab is selected
    pub tab_dimmed: u32,                   // Tab background, when tab-bar is unfocused & tab is unselected
    pub tab_dimmed_selected: u32,          // Tab background, when tab-bar is unfocused & tab is selected
    pub tab_dimmed_selected_overline: u32, // Horizontal overline, when tab-bar is unfocused & tab is selected
    pub plot_lines: u32,
    pub plot_lines_hovered: u32,
    pub plot_histogram: u32,
    pub plot_histogram_hovered: u32,
    pub table_header_bg: u32,              // Table header background
    pub table_border_strong: u32,          // Table outer and header borders (prefer using Alpha=1.0 here)
    pub table_border_light: u32,           // Table inner borders (prefer using Alpha=1.0 here)
    pub table_row_bg: u32,                 // Table row background (even rows)
    pub table_row_bg_alt: u32,             // Table row background (odd rows)
    pub text_link: u32,                    // Hyperlink color
    pub text_selected_bg: u32,             // Selected text inside an Input_Text
    pub tree_lines: u32,                   // Tree node hierarchy outlines when using Im_Gui_Tree_Node_Flags_Draw_Lines
    pub drag_drop_target: u32,             // Rectangle border highlighting a drop target
    pub drag_drop_target_bg: u32,          // Rectangle background highlighting a drop target
    pub unsaved_marker: u32,               // Unsaved Document marker (in window title and tabs)
    pub nav_cursor: u32,                   // Color of keyboard/gamepad navigation cursor/rectangle, when visible
    pub nav_windowing_highlight: u32,      // Highlight window when using Ctrl+Tab
    pub nav_windowing_dim_bg: u32,         // Darken/colorize entire screen behind the Ctrl+Tab window list, when active
    pub modal_window_dim_bg: u32,          // Darken/colorize entire screen behind a modal window, when one is active
}

// Themes
impl Colors {
	/// Dark theme copied from ImGui
	pub const DARK: Self =
		Self {
			text: unorm(1.00, 1.00, 1.00, 1.00),
			text_disabled: unorm(0.50, 0.50, 0.50, 1.00),
			window_bg: unorm(0.06, 0.06, 0.06, 0.94),
			child_bg: unorm(0.00, 0.00, 0.00, 0.00),
			popup_bg: unorm(0.08, 0.08, 0.08, 0.94),
			border: unorm(0.43, 0.43, 0.50, 0.50),
			border_shadow: unorm(0.00, 0.00, 0.00, 0.00),
			frame_bg: unorm(0.16, 0.29, 0.48, 0.54),
			frame_bg_hovered: unorm(0.26, 0.59, 0.98, 0.40),
			frame_bg_active: unorm(0.26, 0.59, 0.98, 0.67),
			title_bg: unorm(0.04, 0.04, 0.04, 1.00),
			title_bg_active: unorm(0.16, 0.29, 0.48, 1.00),
			title_bg_collapsed: unorm(0.00, 0.00, 0.00, 0.51),
			menu_bar_bg: unorm(0.14, 0.14, 0.14, 1.00),
			scrollbar_bg: unorm(0.02, 0.02, 0.02, 0.53),
			scrollbar_grab: unorm(0.31, 0.31, 0.31, 1.00),
			scrollbar_grab_hovered: unorm(0.41, 0.41, 0.41, 1.00),
			scrollbar_grab_active: unorm(0.51, 0.51, 0.51, 1.00),
			check_mark: unorm(0.26, 0.59, 0.98, 1.00),
			checkbox_selected_bg: unorm(0.23, 0.48, 0.80, 0.45),
			slider_grab: unorm(0.24, 0.52, 0.88, 1.00),
			slider_grab_active: unorm(0.26, 0.59, 0.98, 1.00),
			button: unorm(0.26, 0.59, 0.98, 0.40),
			button_hovered: unorm(0.26, 0.59, 0.98, 1.00),
			button_active: unorm(0.06, 0.53, 0.98, 1.00),
			header: unorm(0.26, 0.59, 0.98, 0.31),
			header_hovered: unorm(0.26, 0.59, 0.98, 0.80),
			header_active: unorm(0.26, 0.59, 0.98, 1.00),
			separator: unorm(0.43, 0.43, 0.50, 0.50),
			separator_hovered: unorm(0.10, 0.40, 0.75, 0.78),
			separator_active: unorm(0.10, 0.40, 0.75, 1.00),
			resize_grip: unorm(0.26, 0.59, 0.98, 0.20),
			resize_grip_hovered: unorm(0.26, 0.59, 0.98, 0.67),
			resize_grip_active: unorm(0.26, 0.59, 0.98, 0.95),
			input_text_cursor: unorm(1.00, 1.00, 1.00, 1.00),
			tab_hovered: unorm(0.26, 0.59, 0.98, 0.80),
			tab: unorm(0.18, 0.35, 0.58, 0.86),
			tab_selected: unorm(0.20, 0.41, 0.68, 1.00),
			tab_selected_overline: unorm(0.26, 0.59, 0.98, 1.00),
			tab_dimmed: unorm(0.07, 0.10, 0.15, 0.97),
			tab_dimmed_selected: unorm(0.14, 0.26, 0.42, 1.00),
			tab_dimmed_selected_overline: unorm(0.50, 0.50, 0.50, 0.00),
			plot_lines: unorm(0.61, 0.61, 0.61, 1.00),
			plot_lines_hovered: unorm(1.00, 0.43, 0.35, 1.00),
			plot_histogram: unorm(0.90, 0.70, 0.00, 1.00),
			plot_histogram_hovered: unorm(1.00, 0.60, 0.00, 1.00),
			table_header_bg: unorm(0.19, 0.19, 0.20, 1.00),
			table_border_strong: unorm(0.31, 0.31, 0.35, 1.00),
			table_border_light: unorm(0.23, 0.23, 0.25, 1.00),
			table_row_bg: unorm(0.00, 0.00, 0.00, 0.00),
			table_row_bg_alt: unorm(1.00, 1.00, 1.00, 0.06),
			text_link: unorm(0.26, 0.59, 0.98, 1.00),
			text_selected_bg: unorm(0.26, 0.59, 0.98, 0.35),
			tree_lines: unorm(0.43, 0.43, 0.50, 0.50),
			drag_drop_target: unorm(1.00, 1.00, 0.00, 0.90),
			drag_drop_target_bg: unorm(0.00, 0.00, 0.00, 0.00),
			unsaved_marker: unorm(1.00, 1.00, 1.00, 1.00),
			nav_cursor: unorm(0.26, 0.59, 0.98, 1.00),
			nav_windowing_highlight: unorm(1.00, 1.00, 1.00, 0.70),
			nav_windowing_dim_bg: unorm(0.80, 0.80, 0.80, 0.20),
			modal_window_dim_bg: unorm(0.80, 0.80, 0.80, 0.35),
		};
	/// Classic ImGui theme
	pub const CLASSIC: Self =
		Self {
			text: unorm(0.90, 0.90, 0.90, 1.00),
			text_disabled: unorm(0.60, 0.60, 0.60, 1.00),
			window_bg: unorm(0.00, 0.00, 0.00, 0.85),
			child_bg: unorm(0.00, 0.00, 0.00, 0.00),
			popup_bg: unorm(0.11, 0.11, 0.14, 0.92),
			border: unorm(0.50, 0.50, 0.50, 0.50),
			border_shadow: unorm(0.00, 0.00, 0.00, 0.00),
			frame_bg: unorm(0.43, 0.43, 0.43, 0.39),
			frame_bg_hovered: unorm(0.47, 0.47, 0.69, 0.40),
			frame_bg_active: unorm(0.42, 0.41, 0.64, 0.69),
			title_bg: unorm(0.27, 0.27, 0.54, 0.83),
			title_bg_active: unorm(0.32, 0.32, 0.63, 0.87),
			title_bg_collapsed: unorm(0.40, 0.40, 0.80, 0.20),
			menu_bar_bg: unorm(0.40, 0.40, 0.55, 0.80),
			scrollbar_bg: unorm(0.20, 0.25, 0.30, 0.60),
			scrollbar_grab: unorm(0.40, 0.40, 0.80, 0.30),
			scrollbar_grab_hovered: unorm(0.40, 0.40, 0.80, 0.40),
			scrollbar_grab_active: unorm(0.41, 0.39, 0.80, 0.60),
			check_mark: unorm(0.90, 0.90, 0.90, 0.50),
			checkbox_selected_bg: unorm(0.42, 0.42, 0.57, 0.58),
			slider_grab: unorm(1.00, 1.00, 1.00, 0.30),
			slider_grab_active: unorm(0.41, 0.39, 0.80, 0.60),
			button: unorm(0.35, 0.40, 0.61, 0.62),
			button_hovered: unorm(0.40, 0.48, 0.71, 0.79),
			button_active: unorm(0.46, 0.54, 0.80, 1.00),
			header: unorm(0.40, 0.40, 0.90, 0.45),
			header_hovered: unorm(0.45, 0.45, 0.90, 0.80),
			header_active: unorm(0.53, 0.53, 0.87, 0.80),
			separator: unorm(0.50, 0.50, 0.50, 0.60),
			separator_hovered: unorm(0.60, 0.60, 0.70, 1.00),
			separator_active: unorm(0.70, 0.70, 0.90, 1.00),
			resize_grip: unorm(1.00, 1.00, 1.00, 0.10),
			resize_grip_hovered: unorm(0.78, 0.82, 1.00, 0.60),
			resize_grip_active: unorm(0.78, 0.82, 1.00, 0.90),
			input_text_cursor: unorm(0.90, 0.90, 0.90, 1.00),
			tab_hovered: unorm(0.45, 0.45, 0.90, 0.80),
			tab: unorm(0.34, 0.34, 0.68, 0.79),
			tab_selected: unorm(0.40, 0.40, 0.73, 0.84),
			tab_selected_overline: unorm(0.53, 0.53, 0.87, 0.80),
			tab_dimmed: unorm(0.28, 0.28, 0.57, 0.82),
			tab_dimmed_selected: unorm(0.35, 0.35, 0.65, 0.84),
			tab_dimmed_selected_overline: unorm(0.53, 0.53, 0.87, 0.00),
			plot_lines: unorm(1.00, 1.00, 1.00, 1.00),
			plot_lines_hovered: unorm(0.90, 0.70, 0.00, 1.00),
			plot_histogram: unorm(0.90, 0.70, 0.00, 1.00),
			plot_histogram_hovered: unorm(1.00, 0.60, 0.00, 1.00),
			table_header_bg: unorm(0.27, 0.27, 0.38, 1.00),
			table_border_strong: unorm(0.31, 0.31, 0.45, 1.00),
			table_border_light: unorm(0.26, 0.26, 0.28, 1.00),
			table_row_bg: unorm(0.00, 0.00, 0.00, 0.00),
			table_row_bg_alt: unorm(1.00, 1.00, 1.00, 0.07),
			text_link: unorm(0.53, 0.53, 0.87, 0.80),
			text_selected_bg: unorm(0.00, 0.00, 1.00, 0.35),
			tree_lines: unorm(0.50, 0.50, 0.50, 0.50),
			drag_drop_target: unorm(1.00, 1.00, 0.00, 0.90),
			drag_drop_target_bg: unorm(0.00, 0.00, 0.00, 0.00),
			unsaved_marker: unorm(0.90, 0.90, 0.90, 1.00),
			nav_cursor: unorm(0.45, 0.45, 0.90, 0.80),
			nav_windowing_highlight: unorm(1.00, 1.00, 1.00, 0.70),
			nav_windowing_dim_bg: unorm(0.80, 0.80, 0.80, 0.20),
			modal_window_dim_bg: unorm(0.20, 0.20, 0.20, 0.35),
		};
	/// Light theme copied from ImGui, but it is recommended to use dark instead because light mode sucks
	pub const LIGHT: Self =
		Self {
			text: unorm(0.00, 0.00, 0.00, 1.00),
			text_disabled: unorm(0.60, 0.60, 0.60, 1.00),
			window_bg: unorm(0.94, 0.94, 0.94, 1.00),
			child_bg: unorm(0.00, 0.00, 0.00, 0.00),
			popup_bg: unorm(1.00, 1.00, 1.00, 0.98),
			border: unorm(0.00, 0.00, 0.00, 0.30),
			border_shadow: unorm(0.00, 0.00, 0.00, 0.00),
			frame_bg: unorm(1.00, 1.00, 1.00, 1.00),
			frame_bg_hovered: unorm(0.26, 0.59, 0.98, 0.40),
			frame_bg_active: unorm(0.26, 0.59, 0.98, 0.67),
			title_bg: unorm(0.96, 0.96, 0.96, 1.00),
			title_bg_active: unorm(0.82, 0.82, 0.82, 1.00),
			title_bg_collapsed: unorm(1.00, 1.00, 1.00, 0.51),
			menu_bar_bg: unorm(0.86, 0.86, 0.86, 1.00),
			scrollbar_bg: unorm(0.98, 0.98, 0.98, 0.53),
			scrollbar_grab: unorm(0.69, 0.69, 0.69, 0.80),
			scrollbar_grab_hovered: unorm(0.49, 0.49, 0.49, 0.80),
			scrollbar_grab_active: unorm(0.49, 0.49, 0.49, 1.00),
			check_mark: unorm(0.26, 0.59, 0.98, 1.00),
			checkbox_selected_bg: unorm(0.95, 0.97, 1.00, 1.00),
			slider_grab: unorm(0.26, 0.59, 0.98, 0.78),
			slider_grab_active: unorm(0.46, 0.54, 0.80, 0.60),
			button: unorm(0.26, 0.59, 0.98, 0.40),
			button_hovered: unorm(0.26, 0.59, 0.98, 1.00),
			button_active: unorm(0.06, 0.53, 0.98, 1.00),
			header: unorm(0.26, 0.59, 0.98, 0.31),
			header_hovered: unorm(0.26, 0.59, 0.98, 0.80),
			header_active: unorm(0.26, 0.59, 0.98, 1.00),
			separator: unorm(0.39, 0.39, 0.39, 0.62),
			separator_hovered: unorm(0.14, 0.44, 0.80, 0.78),
			separator_active: unorm(0.14, 0.44, 0.80, 1.00),
			resize_grip: unorm(0.35, 0.35, 0.35, 0.17),
			resize_grip_hovered: unorm(0.26, 0.59, 0.98, 0.67),
			resize_grip_active: unorm(0.26, 0.59, 0.98, 0.95),
			input_text_cursor: unorm(0.00, 0.00, 0.00, 1.00),
			tab_hovered: unorm(0.26, 0.59, 0.98, 0.80),
			tab: unorm(0.76, 0.80, 0.84, 0.93),
			tab_selected: unorm(0.60, 0.73, 0.88, 1.00),
			tab_selected_overline: unorm(0.26, 0.59, 0.98, 1.00),
			tab_dimmed: unorm(0.92, 0.93, 0.94, 0.99),
			tab_dimmed_selected: unorm(0.74, 0.82, 0.91, 1.00),
			tab_dimmed_selected_overline: unorm(0.26, 0.59, 1.00, 0.00),
			plot_lines: unorm(0.39, 0.39, 0.39, 1.00),
			plot_lines_hovered: unorm(1.00, 0.43, 0.35, 1.00),
			plot_histogram: unorm(0.90, 0.70, 0.00, 1.00),
			plot_histogram_hovered: unorm(1.00, 0.45, 0.00, 1.00),
			table_header_bg: unorm(0.78, 0.87, 0.98, 1.00),
			table_border_strong: unorm(0.57, 0.57, 0.64, 1.00),
			table_border_light: unorm(0.68, 0.68, 0.74, 1.00),
			table_row_bg: unorm(0.00, 0.00, 0.00, 0.00),
			table_row_bg_alt: unorm(0.30, 0.30, 0.30, 0.09),
			text_link: unorm(0.26, 0.59, 0.98, 1.00),
			text_selected_bg: unorm(0.26, 0.59, 0.98, 0.35),
			tree_lines: unorm(0.00, 0.00, 0.00, 0.30),
			drag_drop_target: unorm(0.26, 0.59, 0.98, 0.95),
			drag_drop_target_bg: unorm(0.00, 0.00, 0.00, 0.00),
			unsaved_marker: unorm(0.00, 0.00, 0.00, 1.00),
			nav_cursor: unorm(0.26, 0.59, 0.98, 0.80),
			nav_windowing_highlight: unorm(0.70, 0.70, 0.70, 0.70),
			nav_windowing_dim_bg: unorm(0.20, 0.20, 0.20, 0.20),
			modal_window_dim_bg: unorm(0.20, 0.20, 0.20, 0.35),
		};
}

impl Default for Colors {
	fn default() -> Self {
    	Self::DARK
	}
}