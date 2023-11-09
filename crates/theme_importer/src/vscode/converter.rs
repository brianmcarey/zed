use std::collections::HashMap;

use anyhow::Result;
use gpui::{Hsla, Rgba};
use theme::{
    StatusColorsRefinement, ThemeColorsRefinement, UserSyntaxTheme, UserTheme,
    UserThemeStylesRefinement,
};

use crate::util::Traverse;
use crate::vscode::VsCodeTheme;
use crate::ThemeMetadata;

pub(crate) fn try_parse_color(color: &str) -> Result<Hsla> {
    Ok(Rgba::try_from(color)?.into())
}

pub struct VsCodeThemeConverter {
    theme: VsCodeTheme,
    theme_metadata: ThemeMetadata,
}

impl VsCodeThemeConverter {
    pub fn new(theme: VsCodeTheme, theme_metadata: ThemeMetadata) -> Self {
        Self {
            theme,
            theme_metadata,
        }
    }

    pub fn convert(self) -> Result<UserTheme> {
        let appearance = self.theme_metadata.appearance.into();

        let status_color_refinements = self.convert_status_colors()?;
        let theme_colors_refinements = self.convert_theme_colors()?;

        let mut highlight_styles = HashMap::new();

        for token_color in self.theme.token_colors {
            highlight_styles.extend(token_color.highlight_styles()?);
        }

        let syntax_theme = UserSyntaxTheme {
            highlights: highlight_styles.into_iter().collect(),
        };

        Ok(UserTheme {
            name: self.theme_metadata.name.into(),
            appearance,
            styles: UserThemeStylesRefinement {
                colors: theme_colors_refinements,
                status: status_color_refinements,
                syntax: Some(syntax_theme),
            },
        })
    }

    fn convert_status_colors(&self) -> Result<StatusColorsRefinement> {
        let vscode_colors = &self.theme.colors;

        Ok(StatusColorsRefinement {
            // conflict: None,
            // created: None,
            deleted: vscode_colors
                .error_foreground
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            error: vscode_colors
                .error_foreground
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            hidden: vscode_colors
                .tab_inactive_foreground
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            // ignored: None,
            // info: None,
            // modified: None,
            // renamed: None,
            // success: None,
            warning: vscode_colors
                .list_warning_foreground
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            ..Default::default()
        })
    }

    fn convert_theme_colors(&self) -> Result<ThemeColorsRefinement> {
        let vscode_colors = &self.theme.colors;

        Ok(ThemeColorsRefinement {
            border: vscode_colors
                .panel_border
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            border_variant: vscode_colors
                .panel_border
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            border_focused: vscode_colors
                .focus_border
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            border_disabled: vscode_colors
                .panel_border
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            border_selected: vscode_colors
                .panel_border
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            border_transparent: vscode_colors
                .panel_border
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            elevated_surface_background: vscode_colors
                .panel_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            surface_background: vscode_colors
                .panel_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            background: vscode_colors
                .editor_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            element_background: vscode_colors
                .button_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            element_hover: vscode_colors
                .list_hover_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            element_selected: vscode_colors
                .list_active_selection_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            ghost_element_hover: vscode_colors
                .list_hover_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            drop_target_background: vscode_colors
                .list_drop_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            text: vscode_colors
                .foreground
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            tab_active_background: vscode_colors
                .tab_active_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            tab_inactive_background: vscode_colors
                .tab_inactive_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            editor_background: vscode_colors
                .editor_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            editor_gutter_background: vscode_colors
                .editor_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            editor_line_number: vscode_colors
                .editor_line_number_foreground
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            editor_active_line_number: vscode_colors
                .editor_foreground
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_background: vscode_colors
                .terminal_background
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_bright_black: vscode_colors
                .terminal_ansi_bright_black
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_bright_red: vscode_colors
                .terminal_ansi_bright_red
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_bright_green: vscode_colors
                .terminal_ansi_bright_green
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_bright_yellow: vscode_colors
                .terminal_ansi_bright_yellow
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_bright_blue: vscode_colors
                .terminal_ansi_bright_blue
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_bright_magenta: vscode_colors
                .terminal_ansi_bright_magenta
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_bright_cyan: vscode_colors
                .terminal_ansi_bright_cyan
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_bright_white: vscode_colors
                .terminal_ansi_bright_white
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_black: vscode_colors
                .terminal_ansi_black
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_red: vscode_colors
                .terminal_ansi_red
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_green: vscode_colors
                .terminal_ansi_green
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_yellow: vscode_colors
                .terminal_ansi_yellow
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_blue: vscode_colors
                .terminal_ansi_blue
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_magenta: vscode_colors
                .terminal_ansi_magenta
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_cyan: vscode_colors
                .terminal_ansi_cyan
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            terminal_ansi_white: vscode_colors
                .terminal_ansi_white
                .as_ref()
                .traverse(|color| try_parse_color(&color))?,
            ..Default::default()
        })
    }
}
