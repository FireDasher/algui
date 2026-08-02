from re import compile as regex_compile

from pyperclip import copy as clipboard_copy


class Color:
	r: float
	g: float
	b: float
	a: float
	def __init__(self, r: float, g: float, b: float, a: float) -> None:
		self.r = r
		self.g = g
		self.b = b
		self.a = a
	def __add__(self, other):
		if isinstance(other, Color): return Color(self.r + other.r, self.g + other.g, self.b + other.b, self.a + other.a)
		elif isinstance(other, float): return Color(self.r + other, self.g + other, self.b + other, self.a + other)
	def __sub__(self, other):
		if isinstance(other, Color): return Color(self.r - other.r, self.g - other.g, self.b - other.b, self.a - other.a)
		elif isinstance(other, float): return Color(self.r - other, self.g - other, self.b - other, self.a - other)
	def __mul__(self, other):
		if isinstance(other, Color): return Color(self.r * other.r, self.g * other.g, self.b * other.b, self.a * other.a)
		elif isinstance(other, float): return Color(self.r * other, self.g * other, self.b * other, self.a * other)
	def __truediv__(self, other):
		if isinstance(other, Color): return Color(self.r / other.r, self.g / other.g, self.b / other.b, self.a / other.a)
		elif isinstance(other, float): return Color(self.r / other, self.g / other, self.b / other, self.a / other)
	def lerp(self, other, t: float):
		return (self + (other - self) * t)

theme_code_dark: str = \
"""
colors[ImGuiCol_Text]                   = ImVec4(1.00f, 1.00f, 1.00f, 1.00f);
colors[ImGuiCol_TextDisabled]           = ImVec4(0.50f, 0.50f, 0.50f, 1.00f);
colors[ImGuiCol_WindowBg]               = ImVec4(0.06f, 0.06f, 0.06f, 0.94f);
colors[ImGuiCol_ChildBg]                = ImVec4(0.00f, 0.00f, 0.00f, 0.00f);
colors[ImGuiCol_PopupBg]                = ImVec4(0.08f, 0.08f, 0.08f, 0.94f);
colors[ImGuiCol_Border]                 = ImVec4(0.43f, 0.43f, 0.50f, 0.50f);
colors[ImGuiCol_BorderShadow]           = ImVec4(0.00f, 0.00f, 0.00f, 0.00f);
colors[ImGuiCol_FrameBg]                = ImVec4(0.16f, 0.29f, 0.48f, 0.54f);
colors[ImGuiCol_FrameBgHovered]         = ImVec4(0.26f, 0.59f, 0.98f, 0.40f);
colors[ImGuiCol_FrameBgActive]          = ImVec4(0.26f, 0.59f, 0.98f, 0.67f);
colors[ImGuiCol_TitleBg]                = ImVec4(0.04f, 0.04f, 0.04f, 1.00f);
colors[ImGuiCol_TitleBgActive]          = ImVec4(0.16f, 0.29f, 0.48f, 1.00f);
colors[ImGuiCol_TitleBgCollapsed]       = ImVec4(0.00f, 0.00f, 0.00f, 0.51f);
colors[ImGuiCol_MenuBarBg]              = ImVec4(0.14f, 0.14f, 0.14f, 1.00f);
colors[ImGuiCol_ScrollbarBg]            = ImVec4(0.02f, 0.02f, 0.02f, 0.53f);
colors[ImGuiCol_ScrollbarGrab]          = ImVec4(0.31f, 0.31f, 0.31f, 1.00f);
colors[ImGuiCol_ScrollbarGrabHovered]   = ImVec4(0.41f, 0.41f, 0.41f, 1.00f);
colors[ImGuiCol_ScrollbarGrabActive]    = ImVec4(0.51f, 0.51f, 0.51f, 1.00f);
colors[ImGuiCol_CheckMark]              = ImVec4(0.26f, 0.59f, 0.98f, 1.00f);
colors[ImGuiCol_CheckboxSelectedBg]     = ImLerp(colors[ImGuiCol_FrameBg], colors[ImGuiCol_FrameBgHovered], 0.65f);
colors[ImGuiCol_SliderGrab]             = ImVec4(0.24f, 0.52f, 0.88f, 1.00f);
colors[ImGuiCol_SliderGrabActive]       = ImVec4(0.26f, 0.59f, 0.98f, 1.00f);
colors[ImGuiCol_Button]                 = ImVec4(0.26f, 0.59f, 0.98f, 0.40f);
colors[ImGuiCol_ButtonHovered]          = ImVec4(0.26f, 0.59f, 0.98f, 1.00f);
colors[ImGuiCol_ButtonActive]           = ImVec4(0.06f, 0.53f, 0.98f, 1.00f);
colors[ImGuiCol_Header]                 = ImVec4(0.26f, 0.59f, 0.98f, 0.31f);
colors[ImGuiCol_HeaderHovered]          = ImVec4(0.26f, 0.59f, 0.98f, 0.80f);
colors[ImGuiCol_HeaderActive]           = ImVec4(0.26f, 0.59f, 0.98f, 1.00f);
colors[ImGuiCol_Separator]              = colors[ImGuiCol_Border];
colors[ImGuiCol_SeparatorHovered]       = ImVec4(0.10f, 0.40f, 0.75f, 0.78f);
colors[ImGuiCol_SeparatorActive]        = ImVec4(0.10f, 0.40f, 0.75f, 1.00f);
colors[ImGuiCol_ResizeGrip]             = ImVec4(0.26f, 0.59f, 0.98f, 0.20f);
colors[ImGuiCol_ResizeGripHovered]      = ImVec4(0.26f, 0.59f, 0.98f, 0.67f);
colors[ImGuiCol_ResizeGripActive]       = ImVec4(0.26f, 0.59f, 0.98f, 0.95f);
colors[ImGuiCol_InputTextCursor]        = colors[ImGuiCol_Text];
colors[ImGuiCol_TabHovered]             = colors[ImGuiCol_HeaderHovered];
colors[ImGuiCol_Tab]                    = ImLerp(colors[ImGuiCol_Header],       colors[ImGuiCol_TitleBgActive], 0.80f);
colors[ImGuiCol_TabSelected]            = ImLerp(colors[ImGuiCol_HeaderActive], colors[ImGuiCol_TitleBgActive], 0.60f);
colors[ImGuiCol_TabSelectedOverline]    = colors[ImGuiCol_HeaderActive];
colors[ImGuiCol_TabDimmed]              = ImLerp(colors[ImGuiCol_Tab],          colors[ImGuiCol_TitleBg], 0.80f);
colors[ImGuiCol_TabDimmedSelected]      = ImLerp(colors[ImGuiCol_TabSelected],  colors[ImGuiCol_TitleBg], 0.40f);
colors[ImGuiCol_TabDimmedSelectedOverline] = ImVec4(0.50f, 0.50f, 0.50f, 0.00f);
colors[ImGuiCol_PlotLines]              = ImVec4(0.61f, 0.61f, 0.61f, 1.00f);
colors[ImGuiCol_PlotLinesHovered]       = ImVec4(1.00f, 0.43f, 0.35f, 1.00f);
colors[ImGuiCol_PlotHistogram]          = ImVec4(0.90f, 0.70f, 0.00f, 1.00f);
colors[ImGuiCol_PlotHistogramHovered]   = ImVec4(1.00f, 0.60f, 0.00f, 1.00f);
colors[ImGuiCol_TableHeaderBg]          = ImVec4(0.19f, 0.19f, 0.20f, 1.00f);
colors[ImGuiCol_TableBorderStrong]      = ImVec4(0.31f, 0.31f, 0.35f, 1.00f);   // Prefer using Alpha=1.0 here
colors[ImGuiCol_TableBorderLight]       = ImVec4(0.23f, 0.23f, 0.25f, 1.00f);   // Prefer using Alpha=1.0 here
colors[ImGuiCol_TableRowBg]             = ImVec4(0.00f, 0.00f, 0.00f, 0.00f);
colors[ImGuiCol_TableRowBgAlt]          = ImVec4(1.00f, 1.00f, 1.00f, 0.06f);
colors[ImGuiCol_TextLink]               = colors[ImGuiCol_HeaderActive];
colors[ImGuiCol_TextSelectedBg]         = ImVec4(0.26f, 0.59f, 0.98f, 0.35f);
colors[ImGuiCol_TreeLines]              = colors[ImGuiCol_Border];
colors[ImGuiCol_DragDropTarget]         = ImVec4(1.00f, 1.00f, 0.00f, 0.90f);
colors[ImGuiCol_DragDropTargetBg]       = ImVec4(0.00f, 0.00f, 0.00f, 0.00f);
colors[ImGuiCol_UnsavedMarker]          = ImVec4(1.00f, 1.00f, 1.00f, 1.00f);
colors[ImGuiCol_NavCursor]              = ImVec4(0.26f, 0.59f, 0.98f, 1.00f);
colors[ImGuiCol_NavWindowingHighlight]  = ImVec4(1.00f, 1.00f, 1.00f, 0.70f);
colors[ImGuiCol_NavWindowingDimBg]      = ImVec4(0.80f, 0.80f, 0.80f, 0.20f);
colors[ImGuiCol_ModalWindowDimBg]       = ImVec4(0.80f, 0.80f, 0.80f, 0.35f);
"""
theme_code_classic = \
"""
colors[ImGuiCol_Text]                   = ImVec4(0.90f, 0.90f, 0.90f, 1.00f);
colors[ImGuiCol_TextDisabled]           = ImVec4(0.60f, 0.60f, 0.60f, 1.00f);
colors[ImGuiCol_WindowBg]               = ImVec4(0.00f, 0.00f, 0.00f, 0.85f);
colors[ImGuiCol_ChildBg]                = ImVec4(0.00f, 0.00f, 0.00f, 0.00f);
colors[ImGuiCol_PopupBg]                = ImVec4(0.11f, 0.11f, 0.14f, 0.92f);
colors[ImGuiCol_Border]                 = ImVec4(0.50f, 0.50f, 0.50f, 0.50f);
colors[ImGuiCol_BorderShadow]           = ImVec4(0.00f, 0.00f, 0.00f, 0.00f);
colors[ImGuiCol_FrameBg]                = ImVec4(0.43f, 0.43f, 0.43f, 0.39f);
colors[ImGuiCol_FrameBgHovered]         = ImVec4(0.47f, 0.47f, 0.69f, 0.40f);
colors[ImGuiCol_FrameBgActive]          = ImVec4(0.42f, 0.41f, 0.64f, 0.69f);
colors[ImGuiCol_TitleBg]                = ImVec4(0.27f, 0.27f, 0.54f, 0.83f);
colors[ImGuiCol_TitleBgActive]          = ImVec4(0.32f, 0.32f, 0.63f, 0.87f);
colors[ImGuiCol_TitleBgCollapsed]       = ImVec4(0.40f, 0.40f, 0.80f, 0.20f);
colors[ImGuiCol_MenuBarBg]              = ImVec4(0.40f, 0.40f, 0.55f, 0.80f);
colors[ImGuiCol_ScrollbarBg]            = ImVec4(0.20f, 0.25f, 0.30f, 0.60f);
colors[ImGuiCol_ScrollbarGrab]          = ImVec4(0.40f, 0.40f, 0.80f, 0.30f);
colors[ImGuiCol_ScrollbarGrabHovered]   = ImVec4(0.40f, 0.40f, 0.80f, 0.40f);
colors[ImGuiCol_ScrollbarGrabActive]    = ImVec4(0.41f, 0.39f, 0.80f, 0.60f);
colors[ImGuiCol_CheckMark]              = ImVec4(0.90f, 0.90f, 0.90f, 0.50f);
colors[ImGuiCol_CheckboxSelectedBg]     = ImLerp(colors[ImGuiCol_FrameBg], colors[ImGuiCol_FrameBgActive], 0.65f);
colors[ImGuiCol_SliderGrab]             = ImVec4(1.00f, 1.00f, 1.00f, 0.30f);
colors[ImGuiCol_SliderGrabActive]       = ImVec4(0.41f, 0.39f, 0.80f, 0.60f);
colors[ImGuiCol_Button]                 = ImVec4(0.35f, 0.40f, 0.61f, 0.62f);
colors[ImGuiCol_ButtonHovered]          = ImVec4(0.40f, 0.48f, 0.71f, 0.79f);
colors[ImGuiCol_ButtonActive]           = ImVec4(0.46f, 0.54f, 0.80f, 1.00f);
colors[ImGuiCol_Header]                 = ImVec4(0.40f, 0.40f, 0.90f, 0.45f);
colors[ImGuiCol_HeaderHovered]          = ImVec4(0.45f, 0.45f, 0.90f, 0.80f);
colors[ImGuiCol_HeaderActive]           = ImVec4(0.53f, 0.53f, 0.87f, 0.80f);
colors[ImGuiCol_Separator]              = ImVec4(0.50f, 0.50f, 0.50f, 0.60f);
colors[ImGuiCol_SeparatorHovered]       = ImVec4(0.60f, 0.60f, 0.70f, 1.00f);
colors[ImGuiCol_SeparatorActive]        = ImVec4(0.70f, 0.70f, 0.90f, 1.00f);
colors[ImGuiCol_ResizeGrip]             = ImVec4(1.00f, 1.00f, 1.00f, 0.10f);
colors[ImGuiCol_ResizeGripHovered]      = ImVec4(0.78f, 0.82f, 1.00f, 0.60f);
colors[ImGuiCol_ResizeGripActive]       = ImVec4(0.78f, 0.82f, 1.00f, 0.90f);
colors[ImGuiCol_InputTextCursor]        = colors[ImGuiCol_Text];
colors[ImGuiCol_TabHovered]             = colors[ImGuiCol_HeaderHovered];
colors[ImGuiCol_Tab]                    = ImLerp(colors[ImGuiCol_Header],       colors[ImGuiCol_TitleBgActive], 0.80f);
colors[ImGuiCol_TabSelected]            = ImLerp(colors[ImGuiCol_HeaderActive], colors[ImGuiCol_TitleBgActive], 0.60f);
colors[ImGuiCol_TabSelectedOverline]    = colors[ImGuiCol_HeaderActive];
colors[ImGuiCol_TabDimmed]              = ImLerp(colors[ImGuiCol_Tab],          colors[ImGuiCol_TitleBg], 0.80f);
colors[ImGuiCol_TabDimmedSelected]      = ImLerp(colors[ImGuiCol_TabSelected],  colors[ImGuiCol_TitleBg], 0.40f);
colors[ImGuiCol_TabDimmedSelectedOverline] = ImVec4(0.53f, 0.53f, 0.87f, 0.00f);
colors[ImGuiCol_PlotLines]              = ImVec4(1.00f, 1.00f, 1.00f, 1.00f);
colors[ImGuiCol_PlotLinesHovered]       = ImVec4(0.90f, 0.70f, 0.00f, 1.00f);
colors[ImGuiCol_PlotHistogram]          = ImVec4(0.90f, 0.70f, 0.00f, 1.00f);
colors[ImGuiCol_PlotHistogramHovered]   = ImVec4(1.00f, 0.60f, 0.00f, 1.00f);
colors[ImGuiCol_TableHeaderBg]          = ImVec4(0.27f, 0.27f, 0.38f, 1.00f);
colors[ImGuiCol_TableBorderStrong]      = ImVec4(0.31f, 0.31f, 0.45f, 1.00f);   // Prefer using Alpha=1.0 here
colors[ImGuiCol_TableBorderLight]       = ImVec4(0.26f, 0.26f, 0.28f, 1.00f);   // Prefer using Alpha=1.0 here
colors[ImGuiCol_TableRowBg]             = ImVec4(0.00f, 0.00f, 0.00f, 0.00f);
colors[ImGuiCol_TableRowBgAlt]          = ImVec4(1.00f, 1.00f, 1.00f, 0.07f);
colors[ImGuiCol_TextLink]               = colors[ImGuiCol_HeaderActive];
colors[ImGuiCol_TextSelectedBg]         = ImVec4(0.00f, 0.00f, 1.00f, 0.35f);
colors[ImGuiCol_TreeLines]              = colors[ImGuiCol_Border];
colors[ImGuiCol_DragDropTarget]         = ImVec4(1.00f, 1.00f, 0.00f, 0.90f);
colors[ImGuiCol_DragDropTargetBg]       = ImVec4(0.00f, 0.00f, 0.00f, 0.00f);
colors[ImGuiCol_UnsavedMarker]          = ImVec4(0.90f, 0.90f, 0.90f, 1.00f);
colors[ImGuiCol_NavCursor]              = colors[ImGuiCol_HeaderHovered];
colors[ImGuiCol_NavWindowingHighlight]  = ImVec4(1.00f, 1.00f, 1.00f, 0.70f);
colors[ImGuiCol_NavWindowingDimBg]      = ImVec4(0.80f, 0.80f, 0.80f, 0.20f);
colors[ImGuiCol_ModalWindowDimBg]       = ImVec4(0.20f, 0.20f, 0.20f, 0.35f);
"""
# I hate light mode but I have to add it anyway
theme_code_light = \
"""
colors[ImGuiCol_Text]                   = ImVec4(0.00f, 0.00f, 0.00f, 1.00f);
colors[ImGuiCol_TextDisabled]           = ImVec4(0.60f, 0.60f, 0.60f, 1.00f);
colors[ImGuiCol_WindowBg]               = ImVec4(0.94f, 0.94f, 0.94f, 1.00f);
colors[ImGuiCol_ChildBg]                = ImVec4(0.00f, 0.00f, 0.00f, 0.00f);
colors[ImGuiCol_PopupBg]                = ImVec4(1.00f, 1.00f, 1.00f, 0.98f);
colors[ImGuiCol_Border]                 = ImVec4(0.00f, 0.00f, 0.00f, 0.30f);
colors[ImGuiCol_BorderShadow]           = ImVec4(0.00f, 0.00f, 0.00f, 0.00f);
colors[ImGuiCol_FrameBg]                = ImVec4(1.00f, 1.00f, 1.00f, 1.00f);
colors[ImGuiCol_FrameBgHovered]         = ImVec4(0.26f, 0.59f, 0.98f, 0.40f);
colors[ImGuiCol_FrameBgActive]          = ImVec4(0.26f, 0.59f, 0.98f, 0.67f);
colors[ImGuiCol_TitleBg]                = ImVec4(0.96f, 0.96f, 0.96f, 1.00f);
colors[ImGuiCol_TitleBgActive]          = ImVec4(0.82f, 0.82f, 0.82f, 1.00f);
colors[ImGuiCol_TitleBgCollapsed]       = ImVec4(1.00f, 1.00f, 1.00f, 0.51f);
colors[ImGuiCol_MenuBarBg]              = ImVec4(0.86f, 0.86f, 0.86f, 1.00f);
colors[ImGuiCol_ScrollbarBg]            = ImVec4(0.98f, 0.98f, 0.98f, 0.53f);
colors[ImGuiCol_ScrollbarGrab]          = ImVec4(0.69f, 0.69f, 0.69f, 0.80f);
colors[ImGuiCol_ScrollbarGrabHovered]   = ImVec4(0.49f, 0.49f, 0.49f, 0.80f);
colors[ImGuiCol_ScrollbarGrabActive]    = ImVec4(0.49f, 0.49f, 0.49f, 1.00f);
colors[ImGuiCol_CheckMark]              = ImVec4(0.26f, 0.59f, 0.98f, 1.00f);
colors[ImGuiCol_CheckboxSelectedBg]     = ImVec4(0.95f, 0.97f, 1.00f, 1.00f);
colors[ImGuiCol_SliderGrab]             = ImVec4(0.26f, 0.59f, 0.98f, 0.78f);
colors[ImGuiCol_SliderGrabActive]       = ImVec4(0.46f, 0.54f, 0.80f, 0.60f);
colors[ImGuiCol_Button]                 = ImVec4(0.26f, 0.59f, 0.98f, 0.40f);
colors[ImGuiCol_ButtonHovered]          = ImVec4(0.26f, 0.59f, 0.98f, 1.00f);
colors[ImGuiCol_ButtonActive]           = ImVec4(0.06f, 0.53f, 0.98f, 1.00f);
colors[ImGuiCol_Header]                 = ImVec4(0.26f, 0.59f, 0.98f, 0.31f);
colors[ImGuiCol_HeaderHovered]          = ImVec4(0.26f, 0.59f, 0.98f, 0.80f);
colors[ImGuiCol_HeaderActive]           = ImVec4(0.26f, 0.59f, 0.98f, 1.00f);
colors[ImGuiCol_Separator]              = ImVec4(0.39f, 0.39f, 0.39f, 0.62f);
colors[ImGuiCol_SeparatorHovered]       = ImVec4(0.14f, 0.44f, 0.80f, 0.78f);
colors[ImGuiCol_SeparatorActive]        = ImVec4(0.14f, 0.44f, 0.80f, 1.00f);
colors[ImGuiCol_ResizeGrip]             = ImVec4(0.35f, 0.35f, 0.35f, 0.17f);
colors[ImGuiCol_ResizeGripHovered]      = ImVec4(0.26f, 0.59f, 0.98f, 0.67f);
colors[ImGuiCol_ResizeGripActive]       = ImVec4(0.26f, 0.59f, 0.98f, 0.95f);
colors[ImGuiCol_InputTextCursor]        = colors[ImGuiCol_Text];
colors[ImGuiCol_TabHovered]             = colors[ImGuiCol_HeaderHovered];
colors[ImGuiCol_Tab]                    = ImLerp(colors[ImGuiCol_Header],       colors[ImGuiCol_TitleBgActive], 0.90f);
colors[ImGuiCol_TabSelected]            = ImLerp(colors[ImGuiCol_HeaderActive], colors[ImGuiCol_TitleBgActive], 0.60f);
colors[ImGuiCol_TabSelectedOverline]    = colors[ImGuiCol_HeaderActive];
colors[ImGuiCol_TabDimmed]              = ImLerp(colors[ImGuiCol_Tab],          colors[ImGuiCol_TitleBg], 0.80f);
colors[ImGuiCol_TabDimmedSelected]      = ImLerp(colors[ImGuiCol_TabSelected],  colors[ImGuiCol_TitleBg], 0.40f);
colors[ImGuiCol_TabDimmedSelectedOverline] = ImVec4(0.26f, 0.59f, 1.00f, 0.00f);
colors[ImGuiCol_PlotLines]              = ImVec4(0.39f, 0.39f, 0.39f, 1.00f);
colors[ImGuiCol_PlotLinesHovered]       = ImVec4(1.00f, 0.43f, 0.35f, 1.00f);
colors[ImGuiCol_PlotHistogram]          = ImVec4(0.90f, 0.70f, 0.00f, 1.00f);
colors[ImGuiCol_PlotHistogramHovered]   = ImVec4(1.00f, 0.45f, 0.00f, 1.00f);
colors[ImGuiCol_TableHeaderBg]          = ImVec4(0.78f, 0.87f, 0.98f, 1.00f);
colors[ImGuiCol_TableBorderStrong]      = ImVec4(0.57f, 0.57f, 0.64f, 1.00f);   // Prefer using Alpha=1.0 here
colors[ImGuiCol_TableBorderLight]       = ImVec4(0.68f, 0.68f, 0.74f, 1.00f);   // Prefer using Alpha=1.0 here
colors[ImGuiCol_TableRowBg]             = ImVec4(0.00f, 0.00f, 0.00f, 0.00f);
colors[ImGuiCol_TableRowBgAlt]          = ImVec4(0.30f, 0.30f, 0.30f, 0.09f);
colors[ImGuiCol_TextLink]               = colors[ImGuiCol_HeaderActive];
colors[ImGuiCol_TextSelectedBg]         = ImVec4(0.26f, 0.59f, 0.98f, 0.35f);
colors[ImGuiCol_TreeLines]              = colors[ImGuiCol_Border];
colors[ImGuiCol_DragDropTarget]         = ImVec4(0.26f, 0.59f, 0.98f, 0.95f);
colors[ImGuiCol_DragDropTargetBg]       = ImVec4(0.00f, 0.00f, 0.00f, 0.00f);
colors[ImGuiCol_UnsavedMarker]          = ImVec4(0.00f, 0.00f, 0.00f, 1.00f);
colors[ImGuiCol_NavCursor]              = colors[ImGuiCol_HeaderHovered];
colors[ImGuiCol_NavWindowingHighlight]  = ImVec4(0.70f, 0.70f, 0.70f, 0.70f);
colors[ImGuiCol_NavWindowingDimBg]      = ImVec4(0.20f, 0.20f, 0.20f, 0.20f);
colors[ImGuiCol_ModalWindowDimBg]       = ImVec4(0.20f, 0.20f, 0.20f, 0.35f);
"""

theme_code = theme_code_dark # choose which one to process

clean_formatting = True # whether or not to use newlines after each one or pack everything into a single line of code

line_pattern   = regex_compile(r"colors\s*\[\s*(\w+)\s*\]\s*=\s*([^;]+);")
float_pattern  = regex_compile(r"[-+]?\d*\.\d+|\d+")
lerp_pattern   = regex_compile(r"colors\s*\[\s*(\w+)\s*\]")
weight_pattern = regex_compile(r"[-+]?\d*\.\d+|\d+")
source_pattern = regex_compile(r"colors\s*\[\s*(\w+)\s*\]")

colors: dict[str, Color] = {}
for line in theme_code.strip("\n").splitlines():
	line = line.split("//",1)[0].strip()
	match = line_pattern.search(line)

	if not match: continue

	key = match.group(1)
	expression = match.group(2).strip()

	if expression.startswith("ImVec4"):
		values = float_pattern.findall(expression)
		colors[key] = Color(float(values[1]), float(values[2]), float(values[3]), float(values[4]))
	elif expression.startswith("ImLerp"):
		lerpkeys = lerp_pattern.findall(expression)
		weight = weight_pattern.findall(expression.split(",")[-1])[0]
		colors[key] = colors[lerpkeys[0]].lerp(colors[lerpkeys[1]], float(weight))
	elif expression.startswith("colors"):
		sourcekey = source_pattern.search(expression).group(1)
		colors[key] = colors[sourcekey]

sep = "\n" if clean_formatting else " "
indent = "\t" if clean_formatting else "" # tabs can be replaced with spaces if you prefer
result = [f"Self {{{sep}"]

for (key, color) in colors.items():
	pascal_case = "".join([f"_{c.lower()}" if c.isupper() else c for c in key.removeprefix("ImGuiCol_")]).removeprefix("_")
	result.append(f"{indent}{pascal_case}: color({color.r:.2f}, {color.g:.2f}, {color.b:.2f}, {color.a:.2f}),{sep}")

if not clean_formatting:
	result[-1] = result[-1].removesuffix(", ") + " "

result.append("}")
result = "".join(result)

clipboard_copy(result)
print("Code copied to clipboard succesfully!")