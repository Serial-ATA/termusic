// use crate::ui::components::{
//     AlbumPhotoAlign, AlbumPhotoWidth, AlbumPhotoX, AlbumPhotoY, CEHeader, CEThemeSelectTable,
//     ConfigLibraryBackground, ConfigLibraryBorder, ConfigLibraryForeground, ConfigLibraryHighlight,
//     ConfigLibraryHighlightSymbol, ConfigLibraryTitle, ConfigLyricBackground, ConfigLyricBorder,
//     ConfigLyricForeground, ConfigLyricTitle, ConfigPlaylistBackground, ConfigPlaylistBorder,
//     ConfigPlaylistForeground, ConfigPlaylistHighlight, ConfigPlaylistHighlightSymbol,
//     ConfigPlaylistTitle, ConfigProgressBackground, ConfigProgressBorder, ConfigProgressForeground,
//     ConfigProgressTitle, ConfigSavePopup, ExitConfirmation, Footer, GlobalListener, MusicDir,
//     PlaylistDisplaySymbol, PlaylistRandomAlbum, PlaylistRandomTrack,
// };
use crate::config::Settings;
use crate::ui::components::*;
use crate::utils::draw_area_in_absolute;

use crate::ui::components::Alignment as XywhAlign;
use crate::ui::model::{ConfigEditorLayout, Model};
use crate::ui::{Application, Id, IdConfigEditor, Msg};
use tuirealm::event::NoUserEvent;
use tuirealm::tui::layout::{Constraint, Direction, Layout};
use tuirealm::tui::widgets::Clear;
use tuirealm::Frame;
use tuirealm::{State, StateValue};

impl Model {
    #[allow(clippy::too_many_lines)]
    pub fn view_config_editor_general(&mut self) {
        assert!(self
            .terminal
            .raw_mut()
            .draw(|f| {
                let chunks_main = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Length(3),
                            Constraint::Min(3),
                            Constraint::Length(2),
                        ]
                        .as_ref(),
                    )
                    .split(f.size());

                let chunks_middle = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(0)
                    .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
                    .split(chunks_main[1]);

                let chunks_middle_left = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Length(3),
                            Constraint::Length(3),
                            Constraint::Length(3),
                            Constraint::Length(3),
                            Constraint::Length(3),
                            Constraint::Length(3),
                            Constraint::Length(3),
                            Constraint::Min(2),
                        ]
                        .as_ref(),
                    )
                    .split(chunks_middle[0]);

                let chunks_middle_right = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Length(3),
                            Constraint::Length(3),
                            Constraint::Length(3),
                            Constraint::Length(3),
                            Constraint::Length(3),
                            Constraint::Length(3),
                            Constraint::Length(3),
                            Constraint::Min(2),
                        ]
                        .as_ref(),
                    )
                    .split(chunks_middle[1]);
                self.app
                    .view(&Id::ConfigEditor(IdConfigEditor::Header), f, chunks_main[0]);
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::MusicDir),
                    f,
                    chunks_middle_left[0],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::ExitConfirmation),
                    f,
                    chunks_middle_left[1],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::PlaylistDisplaySymbol),
                    f,
                    chunks_middle_left[2],
                );

                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::PlaylistRandomTrack),
                    f,
                    chunks_middle_left[3],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::PlaylistRandomAlbum),
                    f,
                    chunks_middle_left[4],
                );

                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::AlbumPhotoX),
                    f,
                    chunks_middle_right[0],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::AlbumPhotoY),
                    f,
                    chunks_middle_right[1],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::AlbumPhotoWidth),
                    f,
                    chunks_middle_right[2],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::AlbumPhotoAlign),
                    f,
                    chunks_middle_right[3],
                );

                self.app
                    .view(&Id::ConfigEditor(IdConfigEditor::Footer), f, chunks_main[2]);

                Self::view_config_editor_commons(f, &mut self.app);
            })
            .is_ok());
    }

    fn view_config_editor_commons(f: &mut Frame<'_>, app: &mut Application<Id, Msg, NoUserEvent>) {
        // -- popups
        if app.mounted(&Id::ConfigEditor(IdConfigEditor::ConfigSavePopup)) {
            let popup = draw_area_in_absolute(f.size(), 50, 3);
            f.render_widget(Clear, popup);
            app.view(&Id::ConfigEditor(IdConfigEditor::ConfigSavePopup), f, popup);
        }
    }

    #[allow(clippy::too_many_lines)]
    pub fn view_config_editor_color(&mut self) {
        let select_library_foreground_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::LibraryForeground))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };
        let select_library_background_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::LibraryBackground))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };
        let select_library_border_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::LibraryBorder))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };
        let select_library_highlight_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::LibraryHighlight))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };
        let select_playlist_foreground_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::PlaylistForeground))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };
        let select_playlist_background_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::PlaylistBackground))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };
        let select_playlist_border_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::PlaylistBorder))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };
        let select_playlist_highlight_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::PlaylistHighlight))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };

        let select_progress_foreground_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::ProgressForeground))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };
        let select_progress_background_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::ProgressBackground))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };
        let select_progress_border_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::ProgressBorder))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };
        let select_lyric_foreground_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::LyricForeground))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };
        let select_lyric_background_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::LyricBackground))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };
        let select_lyric_border_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::LyricBorder))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };
        assert!(self
            .terminal
            .raw_mut()
            .draw(|f| {
                let chunks_main = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Length(3),
                            Constraint::Min(3),
                            Constraint::Length(2),
                        ]
                        .as_ref(),
                    )
                    .split(f.size());

                let chunks_middle = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(0)
                    .constraints([Constraint::Ratio(1, 4), Constraint::Ratio(3, 4)].as_ref())
                    .split(chunks_main[1]);

                let chunks_middle_right = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Ratio(1, 4),
                            Constraint::Ratio(1, 4),
                            Constraint::Ratio(1, 4),
                            Constraint::Ratio(1, 4),
                        ]
                        .as_ref(),
                    )
                    .split(chunks_middle[1]);
                let chunks_library = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Length(1),
                            Constraint::Length(select_library_foreground_len),
                            Constraint::Length(select_library_background_len),
                            Constraint::Length(select_library_border_len),
                            Constraint::Length(select_library_highlight_len),
                            Constraint::Length(3),
                            Constraint::Min(3),
                        ]
                        .as_ref(),
                    )
                    .split(chunks_middle_right[0]);

                let chunks_playlist = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Length(1),
                            Constraint::Length(select_playlist_foreground_len),
                            Constraint::Length(select_playlist_background_len),
                            Constraint::Length(select_playlist_border_len),
                            Constraint::Length(select_playlist_highlight_len),
                            Constraint::Length(3),
                            Constraint::Min(3),
                        ]
                        .as_ref(),
                    )
                    .split(chunks_middle_right[1]);

                let chunks_progress = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Length(1),
                            Constraint::Length(select_progress_foreground_len),
                            Constraint::Length(select_progress_background_len),
                            Constraint::Length(select_progress_border_len),
                            Constraint::Min(3),
                        ]
                        .as_ref(),
                    )
                    .split(chunks_middle_right[2]);

                let chunks_lyric = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Length(1),
                            Constraint::Length(select_lyric_foreground_len),
                            Constraint::Length(select_lyric_background_len),
                            Constraint::Length(select_lyric_border_len),
                            Constraint::Min(3),
                        ]
                        .as_ref(),
                    )
                    .split(chunks_middle_right[3]);

                self.app
                    .view(&Id::ConfigEditor(IdConfigEditor::Header), f, chunks_main[0]);

                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::CEThemeSelect),
                    f,
                    chunks_middle[0],
                );
                self.app
                    .view(&Id::ConfigEditor(IdConfigEditor::Footer), f, chunks_main[2]);
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::LibraryLabel),
                    f,
                    chunks_library[0],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::LibraryForeground),
                    f,
                    chunks_library[1],
                );

                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::LibraryBackground),
                    f,
                    chunks_library[2],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::LibraryBorder),
                    f,
                    chunks_library[3],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::LibraryHighlight),
                    f,
                    chunks_library[4],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::LibraryHighlightSymbol),
                    f,
                    chunks_library[5],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::PlaylistLabel),
                    f,
                    chunks_playlist[0],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::PlaylistForeground),
                    f,
                    chunks_playlist[1],
                );

                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::PlaylistBackground),
                    f,
                    chunks_playlist[2],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::PlaylistBorder),
                    f,
                    chunks_playlist[3],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::PlaylistHighlight),
                    f,
                    chunks_playlist[4],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::PlaylistHighlightSymbol),
                    f,
                    chunks_playlist[5],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::ProgressLabel),
                    f,
                    chunks_progress[0],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::ProgressForeground),
                    f,
                    chunks_progress[1],
                );

                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::ProgressBackground),
                    f,
                    chunks_progress[2],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::ProgressBorder),
                    f,
                    chunks_progress[3],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::LyricLabel),
                    f,
                    chunks_lyric[0],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::LyricForeground),
                    f,
                    chunks_lyric[1],
                );

                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::LyricBackground),
                    f,
                    chunks_lyric[2],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::LyricBorder),
                    f,
                    chunks_lyric[3],
                );
                Self::view_config_editor_commons(f, &mut self.app);
            })
            .is_ok());
    }

    #[allow(clippy::too_many_lines)]
    pub fn view_config_editor_key1(&mut self) {
        let select_global_quit_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::GlobalQuit))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };
        let select_global_left_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::GlobalLeft))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };
        let select_global_right_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::GlobalRight))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };
        let select_global_up_len = match self.app.state(&Id::ConfigEditor(IdConfigEditor::GlobalUp))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };
        let select_global_down_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::GlobalDown))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };
        let select_global_goto_top_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::GlobalGotoTop))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };
        let select_global_goto_bottom_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::GlobalGotoBottom))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };
        let select_global_player_toggle_pause_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::GlobalPlayerTogglePause))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };
        let select_global_player_next_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::GlobalPlayerNext))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };
        let select_global_player_previous_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::GlobalPlayerPrevious))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };

        let select_global_help_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::GlobalHelp))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };

        let select_global_volume_up_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::GlobalVolumeUp))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };

        let select_global_volume_down_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::GlobalVolumeDown))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };

        let select_global_player_seek_forward_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::GlobalPlayerSeekForward))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };
        let select_global_player_seek_backward_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::GlobalPlayerSeekBackward))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };
        let select_global_player_speed_up_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::GlobalPlayerSpeedUp))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };
        let select_global_player_speed_down_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::GlobalPlayerSpeedDown))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };

        let select_global_lyric_adjust_forward_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::GlobalLyricAdjustForward))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };
        let select_global_lyric_adjust_backward_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::GlobalLyricAdjustBackward))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };
        let select_global_lyric_cycle_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::GlobalLyricCycle))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };
        let select_global_layout_treeview_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::GlobalLayoutTreeview))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };

        let select_global_layout_database_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::GlobalLayoutDatabase))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };

        let select_global_player_toggle_gapless_len = match self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::GlobalPlayerToggleGapless))
        {
            Ok(State::One(_)) => 3,
            _ => 8,
        };

        assert!(self
            .terminal
            .raw_mut()
            .draw(|f| {
                let chunks_main = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Length(3),
                            Constraint::Min(3),
                            Constraint::Length(2),
                        ]
                        .as_ref(),
                    )
                    .split(f.size());

                let chunks_middle = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Ratio(1, 6),
                            Constraint::Ratio(1, 12),
                            Constraint::Ratio(1, 6),
                            Constraint::Ratio(1, 12),
                            Constraint::Ratio(1, 6),
                            Constraint::Ratio(1, 12),
                            Constraint::Ratio(1, 6),
                            Constraint::Ratio(1, 12),
                        ]
                        .as_ref(),
                    )
                    .split(chunks_main[1]);

                let chunks_middle_column1 = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Length(select_global_quit_len),
                            Constraint::Length(select_global_left_len),
                            Constraint::Length(select_global_down_len),
                            Constraint::Length(select_global_up_len),
                            Constraint::Length(select_global_right_len),
                            Constraint::Length(select_global_goto_top_len),
                            Constraint::Length(select_global_goto_bottom_len),
                            Constraint::Length(select_global_player_toggle_pause_len),
                            Constraint::Length(select_global_player_next_len),
                            Constraint::Min(0),
                        ]
                        .as_ref(),
                    )
                    .split(chunks_middle[0]);

                let chunks_middle_column2 = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Length(select_global_quit_len),
                            Constraint::Length(select_global_left_len),
                            Constraint::Length(select_global_down_len),
                            Constraint::Length(select_global_up_len),
                            Constraint::Length(select_global_right_len),
                            Constraint::Length(select_global_goto_top_len),
                            Constraint::Length(select_global_goto_bottom_len),
                            Constraint::Length(select_global_player_toggle_pause_len),
                            Constraint::Length(select_global_player_next_len),
                            Constraint::Min(0),
                        ]
                        .as_ref(),
                    )
                    .split(chunks_middle[1]);

                let chunks_middle_column3 = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Length(select_global_player_previous_len),
                            Constraint::Length(select_global_help_len),
                            Constraint::Length(select_global_volume_up_len),
                            Constraint::Length(select_global_volume_down_len),
                            Constraint::Length(select_global_player_seek_forward_len),
                            Constraint::Length(select_global_player_seek_backward_len),
                            Constraint::Length(select_global_player_speed_up_len),
                            Constraint::Length(select_global_player_speed_down_len),
                            Constraint::Length(select_global_lyric_adjust_forward_len),
                            Constraint::Min(0),
                        ]
                        .as_ref(),
                    )
                    .split(chunks_middle[2]);
                let chunks_middle_column4 = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Length(select_global_player_previous_len),
                            Constraint::Length(select_global_help_len),
                            Constraint::Length(select_global_volume_up_len),
                            Constraint::Length(select_global_volume_down_len),
                            Constraint::Length(select_global_player_seek_forward_len),
                            Constraint::Length(select_global_player_seek_backward_len),
                            Constraint::Length(select_global_player_speed_up_len),
                            Constraint::Length(select_global_player_speed_down_len),
                            Constraint::Length(select_global_lyric_adjust_forward_len),
                            Constraint::Min(0),
                        ]
                        .as_ref(),
                    )
                    .split(chunks_middle[3]);
                let chunks_middle_column5 = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Length(select_global_lyric_adjust_backward_len),
                            Constraint::Length(select_global_lyric_cycle_len),
                            Constraint::Length(select_global_layout_treeview_len),
                            Constraint::Length(select_global_layout_database_len),
                            Constraint::Length(select_global_player_toggle_gapless_len),
                            // Constraint::Length(),
                            // Constraint::Length(),
                            // Constraint::Length(),
                            // Constraint::Length(),
                            Constraint::Min(0),
                        ]
                        .as_ref(),
                    )
                    .split(chunks_middle[4]);
                let chunks_middle_column6 = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Length(select_global_lyric_adjust_backward_len),
                            Constraint::Length(select_global_lyric_cycle_len),
                            Constraint::Length(select_global_layout_treeview_len),
                            Constraint::Length(select_global_layout_database_len),
                            Constraint::Length(select_global_player_toggle_gapless_len),
                            // Constraint::Length(),
                            // Constraint::Length(),
                            // Constraint::Length(),
                            // Constraint::Length(),
                            Constraint::Min(0),
                        ]
                        .as_ref(),
                    )
                    .split(chunks_middle[5]);

                // let chunks_middle_column7 = Layout::default()
                //     .direction(Direction::Vertical)
                //     .margin(0)
                //     .constraints(
                //         [
                //             Constraint::Length(select_library_search_len),
                //             Constraint::Length(select_library_search_youtube_len),
                //             Constraint::Length(select_playlist_delete_len),
                //             Constraint::Length(select_playlist_delete_all_len),
                //             Constraint::Length(select_playlist_search_len),
                //             Constraint::Length(select_playlist_shuffle_len),
                //             Constraint::Length(select_playlist_add_front_len),
                //             Constraint::Length(select_playlist_mode_cycle_len),
                //             Constraint::Length(select_playlist_play_selected_len),
                //             Constraint::Min(0),
                //         ]
                //         .as_ref(),
                //     )
                //     .split(chunks_middle[6]);
                // let chunks_middle_column8 = Layout::default()
                //     .direction(Direction::Vertical)
                //     .margin(0)
                //     .constraints(
                //         [
                //             Constraint::Length(select_library_search_len),
                //             Constraint::Length(select_library_search_youtube_len),
                //             Constraint::Length(select_playlist_delete_len),
                //             Constraint::Length(select_playlist_delete_all_len),
                //             Constraint::Length(select_playlist_search_len),
                //             Constraint::Length(select_playlist_shuffle_len),
                //             Constraint::Length(select_playlist_add_front_len),
                //             Constraint::Length(select_playlist_mode_cycle_len),
                //             Constraint::Length(select_playlist_play_selected_len),
                //             Constraint::Min(0),
                //         ]
                //         .as_ref(),
                //     )
                //     .split(chunks_middle[7]);

                self.app
                    .view(&Id::ConfigEditor(IdConfigEditor::Header), f, chunks_main[0]);
                self.app
                    .view(&Id::ConfigEditor(IdConfigEditor::Footer), f, chunks_main[2]);

                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalQuit),
                    f,
                    chunks_middle_column1[0],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalQuitInput),
                    f,
                    chunks_middle_column2[0],
                );

                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalLeft),
                    f,
                    chunks_middle_column1[1],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalLeftInput),
                    f,
                    chunks_middle_column2[1],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalDown),
                    f,
                    chunks_middle_column1[2],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalDownInput),
                    f,
                    chunks_middle_column2[2],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalUp),
                    f,
                    chunks_middle_column1[3],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalUpInput),
                    f,
                    chunks_middle_column2[3],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalRight),
                    f,
                    chunks_middle_column1[4],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalRightInput),
                    f,
                    chunks_middle_column2[4],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalGotoTop),
                    f,
                    chunks_middle_column1[5],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalGotoTopInput),
                    f,
                    chunks_middle_column2[5],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalGotoBottom),
                    f,
                    chunks_middle_column1[6],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalGotoBottomInput),
                    f,
                    chunks_middle_column2[6],
                );

                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalPlayerTogglePause),
                    f,
                    chunks_middle_column1[7],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalPlayerTogglePauseInput),
                    f,
                    chunks_middle_column2[7],
                );

                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalPlayerNext),
                    f,
                    chunks_middle_column1[8],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalPlayerNextInput),
                    f,
                    chunks_middle_column2[8],
                );

                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalPlayerPrevious),
                    f,
                    chunks_middle_column3[0],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalPlayerPreviousInput),
                    f,
                    chunks_middle_column4[0],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalHelp),
                    f,
                    chunks_middle_column3[1],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalHelpInput),
                    f,
                    chunks_middle_column4[1],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalVolumeUp),
                    f,
                    chunks_middle_column3[2],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalVolumeUpInput),
                    f,
                    chunks_middle_column4[2],
                );

                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalVolumeDown),
                    f,
                    chunks_middle_column3[3],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalVolumeDownInput),
                    f,
                    chunks_middle_column4[3],
                );

                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalPlayerSeekForward),
                    f,
                    chunks_middle_column3[4],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalPlayerSeekForwardInput),
                    f,
                    chunks_middle_column4[4],
                );

                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalPlayerSeekBackward),
                    f,
                    chunks_middle_column3[5],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalPlayerSeekBackwardInput),
                    f,
                    chunks_middle_column4[5],
                );

                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalPlayerSpeedUp),
                    f,
                    chunks_middle_column3[6],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalPlayerSpeedUpInput),
                    f,
                    chunks_middle_column4[6],
                );

                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalPlayerSpeedDown),
                    f,
                    chunks_middle_column3[7],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalPlayerSpeedDownInput),
                    f,
                    chunks_middle_column4[7],
                );

                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalLyricAdjustForward),
                    f,
                    chunks_middle_column3[8],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalLyricAdjustForwardInput),
                    f,
                    chunks_middle_column4[8],
                );

                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalLyricAdjustBackward),
                    f,
                    chunks_middle_column5[0],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalLyricAdjustBackwardInput),
                    f,
                    chunks_middle_column6[0],
                );

                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalLyricCycle),
                    f,
                    chunks_middle_column5[1],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalLyricCycleInput),
                    f,
                    chunks_middle_column6[1],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalLayoutTreeview),
                    f,
                    chunks_middle_column5[2],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalLayoutTreeviewInput),
                    f,
                    chunks_middle_column6[2],
                );

                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalLayoutDatabase),
                    f,
                    chunks_middle_column5[3],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalLayoutDatabaseInput),
                    f,
                    chunks_middle_column6[3],
                );

                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalPlayerToggleGapless),
                    f,
                    chunks_middle_column5[4],
                );
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::GlobalPlayerToggleGaplessInput),
                    f,
                    chunks_middle_column6[4],
                );
                Self::view_config_editor_commons(f, &mut self.app);
            })
            .is_ok());
    }

    pub fn view_config_editor_key2(&mut self) {
        // let select_library_delete_len =
        //     match self.app.state(&Id::KeyEditor(IdKeyEditor::LibraryDelete)) {
        //         Ok(State::One(_)) => 3,
        //         _ => 8,
        //     };
        // let select_library_load_dir_len =
        //     match self.app.state(&Id::KeyEditor(IdKeyEditor::LibraryLoadDir)) {
        //         Ok(State::One(_)) => 3,
        //         _ => 8,
        //     };
        // let select_library_yank_len = match self.app.state(&Id::KeyEditor(IdKeyEditor::LibraryYank))
        // {
        //     Ok(State::One(_)) => 3,
        //     _ => 8,
        // };
        // let select_library_paste_len =
        //     match self.app.state(&Id::KeyEditor(IdKeyEditor::LibraryPaste)) {
        //         Ok(State::One(_)) => 3,
        //         _ => 8,
        //     };
        // let select_library_search_len =
        //     match self.app.state(&Id::KeyEditor(IdKeyEditor::LibrarySearch)) {
        //         Ok(State::One(_)) => 3,
        //         _ => 8,
        //     };
        // let select_library_search_youtube_len = match self
        //     .app
        //     .state(&Id::KeyEditor(IdKeyEditor::LibrarySearchYoutube))
        // {
        //     Ok(State::One(_)) => 3,
        //     _ => 8,
        // };
        // let select_library_tag_editor_len = match self
        //     .app
        //     .state(&Id::KeyEditor(IdKeyEditor::LibraryTagEditor))
        // {
        //     Ok(State::One(_)) => 3,
        //     _ => 8,
        // };
        // let select_playlist_delete_len =
        //     match self.app.state(&Id::KeyEditor(IdKeyEditor::PlaylistDelete)) {
        //         Ok(State::One(_)) => 3,
        //         _ => 8,
        //     };
        // let select_playlist_delete_all_len = match self
        //     .app
        //     .state(&Id::KeyEditor(IdKeyEditor::PlaylistDeleteAll))
        // {
        //     Ok(State::One(_)) => 3,
        //     _ => 8,
        // };

        // let select_playlist_shuffle_len =
        //     match self.app.state(&Id::KeyEditor(IdKeyEditor::PlaylistShuffle)) {
        //         Ok(State::One(_)) => 3,
        //         _ => 8,
        //     };

        // let select_playlist_mode_cycle_len = match self
        //     .app
        //     .state(&Id::KeyEditor(IdKeyEditor::PlaylistModeCycle))
        // {
        //     Ok(State::One(_)) => 3,
        //     _ => 8,
        // };
        // let select_playlist_add_front_len = match self
        //     .app
        //     .state(&Id::KeyEditor(IdKeyEditor::PlaylistAddFront))
        // {
        //     Ok(State::One(_)) => 3,
        //     _ => 8,
        // };
        // let select_playlist_search_len =
        //     match self.app.state(&Id::KeyEditor(IdKeyEditor::PlaylistSearch)) {
        //         Ok(State::One(_)) => 3,
        //         _ => 8,
        //     };
        // let select_playlist_play_selected_len = match self
        //     .app
        //     .state(&Id::KeyEditor(IdKeyEditor::PlaylistPlaySelected))
        // {
        //     Ok(State::One(_)) => 3,
        //     _ => 8,
        // };

        // let select_playlist_swap_down_len = match self
        //     .app
        //     .state(&Id::KeyEditor(IdKeyEditor::PlaylistSwapDown))
        // {
        //     Ok(State::One(_)) => 3,
        //     _ => 8,
        // };

        // let select_playlist_swap_up_len =
        //     match self.app.state(&Id::KeyEditor(IdKeyEditor::PlaylistSwapUp)) {
        //         Ok(State::One(_)) => 3,
        //         _ => 8,
        //     };

        // let select_database_add_all_len =
        //     match self.app.state(&Id::KeyEditor(IdKeyEditor::DatabaseAddAll)) {
        //         Ok(State::One(_)) => 3,
        //         _ => 8,
        //     };

        assert!(self
            .terminal
            .raw_mut()
            .draw(|f| {
                let chunks_main = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Length(3),
                            Constraint::Min(3),
                            Constraint::Length(2),
                        ]
                        .as_ref(),
                    )
                    .split(f.size());

                self.app
                    .view(&Id::ConfigEditor(IdConfigEditor::Header), f, chunks_main[0]);
                self.app.view(
                    &Id::ConfigEditor(IdConfigEditor::MusicDir),
                    f,
                    chunks_main[1],
                );
                self.app
                    .view(&Id::ConfigEditor(IdConfigEditor::Footer), f, chunks_main[2]);

                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::LibraryTagEditor),
                //     f,
                //     chunks_middle_column5[4],
                // );
                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::LibraryTagEditorInput),
                //     f,
                //     chunks_middle_column6[4],
                // );

                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::LibraryDelete),
                //     f,
                //     chunks_middle_column5[5],
                // );
                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::LibraryDeleteInput),
                //     f,
                //     chunks_middle_column6[5],
                // );

                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::LibraryLoadDir),
                //     f,
                //     chunks_middle_column5[6],
                // );
                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::LibraryLoadDirInput),
                //     f,
                //     chunks_middle_column6[6],
                // );

                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::LibraryYank),
                //     f,
                //     chunks_middle_column5[7],
                // );
                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::LibraryYankInput),
                //     f,
                //     chunks_middle_column6[7],
                // );

                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::LibraryPaste),
                //     f,
                //     chunks_middle_column5[8],
                // );
                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::LibraryPasteInput),
                //     f,
                //     chunks_middle_column6[8],
                // );

                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::LibrarySearch),
                //     f,
                //     chunks_middle_column7[0],
                // );
                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::LibrarySearchInput),
                //     f,
                //     chunks_middle_column8[0],
                // );

                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::LibrarySearchYoutube),
                //     f,
                //     chunks_middle_column7[1],
                // );
                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::LibrarySearchYoutubeInput),
                //     f,
                //     chunks_middle_column8[1],
                // );

                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::PlaylistDelete),
                //     f,
                //     chunks_middle_column7[2],
                // );
                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::PlaylistDeleteInput),
                //     f,
                //     chunks_middle_column8[2],
                // );
                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::PlaylistDeleteAll),
                //     f,
                //     chunks_middle_column7[3],
                // );
                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::PlaylistDeleteAllInput),
                //     f,
                //     chunks_middle_column8[3],
                // );
                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::PlaylistSearch),
                //     f,
                //     chunks_middle_column7[4],
                // );
                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::PlaylistSearchInput),
                //     f,
                //     chunks_middle_column8[4],
                // );

                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::PlaylistShuffle),
                //     f,
                //     chunks_middle_column7[5],
                // );
                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::PlaylistShuffleInput),
                //     f,
                //     chunks_middle_column8[5],
                // );

                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::PlaylistAddFront),
                //     f,
                //     chunks_middle_column7[6],
                // );
                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::PlaylistAddFrontInput),
                //     f,
                //     chunks_middle_column8[6],
                // );

                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::PlaylistModeCycle),
                //     f,
                //     chunks_middle_column7[7],
                // );
                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::PlaylistModeCycleInput),
                //     f,
                //     chunks_middle_column8[7],
                // );

                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::PlaylistPlaySelected),
                //     f,
                //     chunks_middle_column7[8],
                // );
                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::PlaylistPlaySelectedInput),
                //     f,
                //     chunks_middle_column8[8],
                // );

                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::PlaylistSwapDown),
                //     f,
                //     chunks_middle_column9[0],
                // );
                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::PlaylistSwapDownInput),
                //     f,
                //     chunks_middle_column10[0],
                // );

                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::PlaylistSwapUp),
                //     f,
                //     chunks_middle_column9[1],
                // );
                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::PlaylistSwapUpInput),
                //     f,
                //     chunks_middle_column10[1],
                // );

                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::DatabaseAddAll),
                //     f,
                //     chunks_middle_column9[4],
                // );
                // self.app.view(
                //     &Id::KeyEditor(IdKeyEditor::DatabaseAddAllInput),
                //     f,
                //     chunks_middle_column10[4],
                // );

                Self::view_config_editor_commons(f, &mut self.app);
            })
            .is_ok());
    }

    #[allow(clippy::too_many_lines)]
    pub fn mount_config_editor(&mut self) {
        self.config_layout = ConfigEditorLayout::General;
        let layout = self.config_layout.clone();

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::Header),
                Box::new(CEHeader::new(&layout, &self.config)),
                vec![]
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::Footer),
                Box::new(Footer::default()),
                vec![]
            )
            .is_ok());

        // Mount general page
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::MusicDir),
                Box::new(MusicDir::new(&self.config)),
                vec![]
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::ExitConfirmation),
                Box::new(ExitConfirmation::new(&self.config)),
                vec![]
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::PlaylistDisplaySymbol),
                Box::new(PlaylistDisplaySymbol::new(&self.config)),
                vec![]
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::PlaylistRandomTrack),
                Box::new(PlaylistRandomTrack::new(&self.config)),
                vec![]
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::PlaylistRandomAlbum),
                Box::new(PlaylistRandomAlbum::new(&self.config)),
                vec![]
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::AlbumPhotoX),
                Box::new(AlbumPhotoX::new(&self.config)),
                vec![]
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::AlbumPhotoY),
                Box::new(AlbumPhotoY::new(&self.config)),
                vec![]
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::AlbumPhotoWidth),
                Box::new(AlbumPhotoWidth::new(&self.config)),
                vec![]
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::AlbumPhotoAlign),
                Box::new(AlbumPhotoAlign::new(&self.config)),
                vec![]
            )
            .is_ok());

        let config = self.config.clone();
        self.remount_config_color(&config);

        // Active Config Editor
        assert!(self
            .app
            .active(&Id::ConfigEditor(IdConfigEditor::MusicDir))
            .is_ok());

        if let Err(e) = Self::theme_select_save() {
            self.mount_error_popup(format!("theme save error: {}", e).as_str());
        }
        if let Err(e) = self.theme_select_load_themes() {
            self.mount_error_popup(format!("Error load themes: {}", e).as_str());
        }
        self.ce_theme_select_sync();
        self.app.lock_subs();
        if let Err(e) = self.update_photo() {
            self.mount_error_popup(format!("clear photo error: {}", e).as_str());
        }
    }

    #[allow(clippy::too_many_lines)]
    pub fn remount_config_color(&mut self, config: &Settings) {
        // Mount color page

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::CEThemeSelect),
                Box::new(CEThemeSelectTable::new(config)),
                vec![]
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::LibraryLabel),
                Box::new(ConfigLibraryTitle::default()),
                vec![]
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::LibraryForeground),
                Box::new(ConfigLibraryForeground::new(config)),
                vec![]
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::LibraryBackground),
                Box::new(ConfigLibraryBackground::new(config)),
                vec![]
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::LibraryBorder),
                Box::new(ConfigLibraryBorder::new(config)),
                vec![]
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::LibraryHighlight),
                Box::new(ConfigLibraryHighlight::new(config)),
                vec![]
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::PlaylistLabel),
                Box::new(ConfigPlaylistTitle::default()),
                vec![]
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::PlaylistForeground),
                Box::new(ConfigPlaylistForeground::new(config)),
                vec![]
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::PlaylistBackground),
                Box::new(ConfigPlaylistBackground::new(config)),
                vec![]
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::PlaylistBorder),
                Box::new(ConfigPlaylistBorder::new(config)),
                vec![]
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::PlaylistHighlight),
                Box::new(ConfigPlaylistHighlight::new(config)),
                vec![]
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::ProgressLabel),
                Box::new(ConfigProgressTitle::default()),
                vec![]
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::ProgressForeground),
                Box::new(ConfigProgressForeground::new(config)),
                vec![]
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::ProgressBackground),
                Box::new(ConfigProgressBackground::new(config)),
                vec![]
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::ProgressBorder),
                Box::new(ConfigProgressBorder::new(config)),
                vec![]
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::LyricLabel),
                Box::new(ConfigLyricTitle::default()),
                vec![]
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::LyricForeground),
                Box::new(ConfigLyricForeground::new(config)),
                vec![]
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::LyricBackground),
                Box::new(ConfigLyricBackground::new(config)),
                vec![]
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::LyricBorder),
                Box::new(ConfigLyricBorder::new(config)),
                vec![]
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::LibraryHighlightSymbol),
                Box::new(ConfigLibraryHighlightSymbol::new(config)),
                vec![]
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::PlaylistHighlightSymbol),
                Box::new(ConfigPlaylistHighlightSymbol::new(config)),
                vec![]
            )
            .is_ok());

        // Key 1: Global keys

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalQuit),
                Box::new(ConfigGlobalQuit::new(config)),
                vec![],
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalQuitInput),
                Box::new(ConfigGlobalQuitInput::new(config)),
                vec![],
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalLeft),
                Box::new(ConfigGlobalLeft::new(config)),
                vec![],
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalLeftInput),
                Box::new(ConfigGlobalLeftInput::new(config)),
                vec![],
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalRight),
                Box::new(ConfigGlobalRight::new(config)),
                vec![],
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalRightInput),
                Box::new(ConfigGlobalRightInput::new(config)),
                vec![],
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalUp),
                Box::new(ConfigGlobalUp::new(config)),
                vec![],
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalUpInput),
                Box::new(ConfigGlobalUpInput::new(config)),
                vec![],
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalDown),
                Box::new(ConfigGlobalDown::new(config)),
                vec![],
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalDownInput),
                Box::new(ConfigGlobalDownInput::new(config)),
                vec![],
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalGotoTop),
                Box::new(ConfigGlobalGotoTop::new(config)),
                vec![],
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalGotoTopInput),
                Box::new(ConfigGlobalGotoTopInput::new(config)),
                vec![],
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalGotoBottom),
                Box::new(ConfigGlobalGotoBottom::new(config)),
                vec![],
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalGotoBottomInput),
                Box::new(ConfigGlobalGotoBottomInput::new(config)),
                vec![],
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalPlayerTogglePause),
                Box::new(ConfigGlobalPlayerTogglePause::new(config)),
                vec![],
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalPlayerTogglePauseInput),
                Box::new(ConfigGlobalPlayerTogglePauseInput::new(config)),
                vec![],
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalPlayerNext),
                Box::new(ConfigGlobalPlayerNext::new(config)),
                vec![],
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalPlayerNextInput),
                Box::new(ConfigGlobalPlayerNextInput::new(config)),
                vec![],
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalPlayerPrevious),
                Box::new(ConfigGlobalPlayerPrevious::new(config)),
                vec![],
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalPlayerPreviousInput),
                Box::new(ConfigGlobalPlayerPreviousInput::new(config)),
                vec![],
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalHelp),
                Box::new(ConfigGlobalHelp::new(config)),
                vec![],
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalHelpInput),
                Box::new(ConfigGlobalHelpInput::new(config)),
                vec![],
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalVolumeUp),
                Box::new(ConfigGlobalVolumeUp::new(config)),
                vec![],
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalVolumeUpInput),
                Box::new(ConfigGlobalVolumeUpInput::new(config)),
                vec![],
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalVolumeDown),
                Box::new(ConfigGlobalVolumeDown::new(config)),
                vec![],
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalVolumeDownInput),
                Box::new(ConfigGlobalVolumeDownInput::new(config)),
                vec![],
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalPlayerSeekForward),
                Box::new(ConfigGlobalPlayerSeekForward::new(config)),
                vec![],
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalPlayerSeekForwardInput),
                Box::new(ConfigGlobalPlayerSeekForwardInput::new(config)),
                vec![],
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalPlayerSeekBackward),
                Box::new(ConfigGlobalPlayerSeekBackward::new(config)),
                vec![],
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalPlayerSeekBackwardInput),
                Box::new(ConfigGlobalPlayerSeekBackwardInput::new(config)),
                vec![],
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalPlayerSpeedUp),
                Box::new(ConfigGlobalPlayerSpeedUp::new(config)),
                vec![],
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalPlayerSpeedUpInput),
                Box::new(ConfigGlobalPlayerSpeedUpInput::new(config)),
                vec![],
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalPlayerSpeedDown),
                Box::new(ConfigGlobalPlayerSpeedDown::new(config)),
                vec![],
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalPlayerSpeedDownInput),
                Box::new(ConfigGlobalPlayerSpeedDownInput::new(config)),
                vec![],
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalLyricAdjustForward),
                Box::new(ConfigGlobalLyricAdjustForward::new(config)),
                vec![],
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalLyricAdjustForwardInput),
                Box::new(ConfigGlobalLyricAdjustForwardInput::new(config)),
                vec![],
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalLyricAdjustBackward),
                Box::new(ConfigGlobalLyricAdjustBackward::new(config)),
                vec![],
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalLyricAdjustBackwardInput),
                Box::new(ConfigGlobalLyricAdjustBackwardInput::new(config)),
                vec![],
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalLyricCycle),
                Box::new(ConfigGlobalLyricCycle::new(config)),
                vec![],
            )
            .is_ok());
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalLyricCycleInput),
                Box::new(ConfigGlobalLyricCycleInput::new(config)),
                vec![],
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalPlayerToggleGapless),
                Box::new(ConfigGlobalPlayerToggleGapless::new(config)),
                vec![],
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalPlayerToggleGaplessInput),
                Box::new(ConfigGlobalPlayerToggleGaplessInput::new(config)),
                vec![],
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalLayoutTreeview),
                Box::new(ConfigGlobalLayoutTreeview::new(config)),
                vec![],
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalLayoutTreeviewInput),
                Box::new(ConfigGlobalLayoutTreeviewInput::new(config)),
                vec![],
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalLayoutDatabase),
                Box::new(ConfigGlobalLayoutDatabase::new(config)),
                vec![],
            )
            .is_ok());

        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::GlobalLayoutDatabaseInput),
                Box::new(ConfigGlobalLayoutDatabaseInput::new(config)),
                vec![],
            )
            .is_ok());

        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::LibraryDelete),
        //         Box::new(KELibraryDelete::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());
        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::LibraryDeleteInput),
        //         Box::new(KELibraryDeleteInput::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());
        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::LibraryLoadDir),
        //         Box::new(KELibraryLoadDir::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());
        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::LibraryLoadDirInput),
        //         Box::new(KELibraryLoadDirInput::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());
        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::LibraryYank),
        //         Box::new(KELibraryYank::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());
        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::LibraryYankInput),
        //         Box::new(KELibraryYankInput::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());
        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::LibraryPaste),
        //         Box::new(KELibraryPaste::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());
        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::LibraryPasteInput),
        //         Box::new(KELibraryPasteInput::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());
        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::LibrarySearch),
        //         Box::new(KELibrarySearch::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());
        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::LibrarySearchInput),
        //         Box::new(KELibrarySearchInput::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());
        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::LibrarySearchYoutube),
        //         Box::new(KELibrarySearchYoutube::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());
        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::LibrarySearchYoutubeInput),
        //         Box::new(KELibrarySearchYoutubeInput::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());
        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::LibraryTagEditor),
        //         Box::new(KELibraryTagEditor::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());
        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::LibraryTagEditorInput),
        //         Box::new(KELibraryTagEditorInput::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());

        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::PlaylistDelete),
        //         Box::new(KEPlaylistDelete::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());
        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::PlaylistDeleteInput),
        //         Box::new(KEPlaylistDeleteInput::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());
        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::PlaylistDeleteAll),
        //         Box::new(KEPlaylistDeleteAll::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());
        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::PlaylistDeleteAllInput),
        //         Box::new(KEPlaylistDeleteAllInput::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());
        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::PlaylistAddFront),
        //         Box::new(KEPlaylistAddFront::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());
        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::PlaylistAddFrontInput),
        //         Box::new(KEPlaylistAddFrontInput::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());
        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::PlaylistShuffle),
        //         Box::new(KEPlaylistShuffle::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());
        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::PlaylistShuffleInput),
        //         Box::new(KEPlaylistShuffleInput::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());
        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::PlaylistSearch),
        //         Box::new(KEPlaylistSearch::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());
        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::PlaylistSearchInput),
        //         Box::new(KEPlaylistSearchInput::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());
        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::PlaylistPlaySelected),
        //         Box::new(KEPlaylistPlaySelected::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());
        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::PlaylistPlaySelectedInput),
        //         Box::new(KEPlaylistPlaySelectedInput::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());
        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::PlaylistModeCycle),
        //         Box::new(KEPlaylistModeCycle::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());
        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::PlaylistModeCycleInput),
        //         Box::new(KEPlaylistModeCycleInput::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());

        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::PlaylistSwapDown),
        //         Box::new(KEPlaylistSwapDown::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());

        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::PlaylistSwapDownInput),
        //         Box::new(KEPlaylistSwapDownInput::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());

        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::PlaylistSwapUp),
        //         Box::new(KEPlaylistSwapUp::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());

        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::PlaylistSwapUpInput),
        //         Box::new(KEPlaylistSwapUpInput::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());

        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::DatabaseAddAll),
        //         Box::new(KEDatabaseAddAll::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());

        // assert!(self
        //     .app
        //     .remount(
        //         Id::KeyEditor(IdKeyEditor::DatabaseAddAllInput),
        //         Box::new(KEDatabaseAddAllInput::new(&self.config.keys)),
        //         vec![],
        //     )
        //     .is_ok());

        self.ce_theme_select_sync();
    }

    #[allow(clippy::too_many_lines)]
    pub fn umount_config_editor(&mut self) {
        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::Header))
            .is_ok());

        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::Footer))
            .is_ok());

        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::MusicDir))
            .is_ok());

        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::ExitConfirmation))
            .is_ok());

        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::PlaylistDisplaySymbol))
            .is_ok());

        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::PlaylistRandomAlbum))
            .is_ok());
        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::PlaylistRandomTrack))
            .is_ok());

        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::AlbumPhotoX))
            .is_ok());
        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::AlbumPhotoY))
            .is_ok());
        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::AlbumPhotoWidth))
            .is_ok());
        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::AlbumPhotoAlign))
            .is_ok());

        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::CEThemeSelect))
            .is_ok());

        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::LibraryLabel))
            .is_ok());

        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::LibraryForeground))
            .is_ok());

        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::LibraryBackground))
            .is_ok());
        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::LibraryBorder))
            .is_ok());
        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::LibraryHighlight))
            .is_ok());

        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::PlaylistLabel))
            .is_ok());

        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::PlaylistForeground))
            .is_ok());

        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::PlaylistBackground))
            .is_ok());
        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::PlaylistBorder))
            .is_ok());
        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::PlaylistHighlight))
            .is_ok());
        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::ProgressLabel))
            .is_ok());

        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::ProgressForeground))
            .is_ok());

        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::ProgressBackground))
            .is_ok());
        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::ProgressBorder))
            .is_ok());
        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::LyricLabel))
            .is_ok());

        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::LyricForeground))
            .is_ok());

        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::LyricBackground))
            .is_ok());
        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::LyricBorder))
            .is_ok());
        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::LibraryHighlightSymbol))
            .is_ok());
        assert!(self
            .app
            .umount(&Id::ConfigEditor(IdConfigEditor::PlaylistHighlightSymbol))
            .is_ok());

        // umount keys global

        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalQuit))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalQuitInput))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalLeft))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalLeftInput))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalRight))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalRightInput))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalUp))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalUpInput))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalDown))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalDownInput))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalGotoTop))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalGotoTopInput))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalGotoBottom))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalGotoBottomInput))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalPlayerTogglePause))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(
                IdConfigEditor::GlobalPlayerTogglePauseInput,
            ))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalPlayerNext))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalPlayerNextInput))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalPlayerPrevious))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalPlayerPreviousInput))
            .ok();

        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalHelp))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalHelpInput))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalVolumeUp))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalVolumeUpInput))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalVolumeDown))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalVolumeDownInput))
            .ok();

        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalPlayerSeekForward))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(
                IdConfigEditor::GlobalPlayerSeekForwardInput,
            ))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalPlayerSeekBackward))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(
                IdConfigEditor::GlobalPlayerSeekBackwardInput,
            ))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalPlayerSpeedUp))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalPlayerSpeedUpInput))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalPlayerSpeedDown))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(
                IdConfigEditor::GlobalPlayerSpeedDownInput,
            ))
            .ok();

        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalLyricAdjustForward))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(
                IdConfigEditor::GlobalLyricAdjustForwardInput,
            ))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalLyricAdjustBackward))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(
                IdConfigEditor::GlobalLyricAdjustBackwardInput,
            ))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalLyricCycle))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalLyricCycleInput))
            .ok();
        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalLayoutDatabase))
            .ok();

        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalLayoutDatabaseInput))
            .ok();

        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalLayoutTreeview))
            .ok();

        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalLayoutTreeviewInput))
            .ok();

        self.app
            .umount(&Id::ConfigEditor(IdConfigEditor::GlobalPlayerToggleGapless))
            .ok();

        self.app
            .umount(&Id::ConfigEditor(
                IdConfigEditor::GlobalPlayerToggleGaplessInput,
            ))
            .ok();

        // umount keys other
        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::LibraryDelete))
        //     .ok();
        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::LibraryDeleteInput))
        //     .ok();
        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::LibraryLoadDir))
        //     .ok();
        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::LibraryLoadDirInput))
        //     .ok();
        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::LibraryYank))
        //     .ok();
        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::LibraryYankInput))
        //     .ok();
        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::LibraryPaste))
        //     .ok();
        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::LibraryPasteInput))
        //     .ok();
        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::LibrarySearch))
        //     .ok();
        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::LibrarySearchInput))
        //     .ok();
        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::LibrarySearchYoutube))
        //     .ok();
        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::LibrarySearchYoutubeInput))
        //     .ok();
        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::LibraryTagEditor))
        //     .ok();
        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::LibraryTagEditorInput))
        //     .ok();
        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::PlaylistDelete))
        //     .ok();
        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::PlaylistDeleteInput))
        //     .ok();

        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::PlaylistDeleteAll))
        //     .ok();
        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::PlaylistDeleteAllInput))
        //     .ok();
        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::PlaylistShuffle))
        //     .ok();
        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::PlaylistShuffleInput))
        //     .ok();
        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::PlaylistModeCycle))
        //     .ok();
        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::PlaylistModeCycleInput))
        //     .ok();
        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::PlaylistPlaySelected))
        //     .ok();
        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::PlaylistPlaySelectedInput))
        //     .ok();
        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::PlaylistAddFront))
        //     .ok();
        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::PlaylistAddFrontInput))
        //     .ok();
        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::PlaylistSearch))
        //     .ok();
        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::PlaylistSearchInput))
        //     .ok();

        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::PlaylistSwapDown))
        //     .ok();
        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::PlaylistSwapDownInput))
        //     .ok();

        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::PlaylistSwapUp))
        //     .ok();
        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::PlaylistSwapUpInput))
        //     .ok();

        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::DatabaseAddAll))
        //     .ok();

        // self.app
        //     .umount(&Id::KeyEditor(IdKeyEditor::DatabaseAddAllInput))
        //     .ok();

        assert!(self
            .app
            .remount(
                Id::GlobalListener,
                Box::new(GlobalListener::new(&self.config.keys)),
                Self::subscribe(&self.config.keys),
            )
            .is_ok());

        self.library_reload_tree();
        self.playlist_reload();
        self.database_reload();
        self.progress_reload();

        self.app.unlock_subs();
        self.remount_label_help();
        self.global_fix_focus();
        self.lyric_reload();
        self.update_lyric();
        if let Err(e) = self.update_photo() {
            self.mount_error_popup(format!("update photo error: {}", e).as_ref());
        }
    }

    pub fn action_change_layout(&mut self) {
        match self.config_layout {
            ConfigEditorLayout::General => self.config_layout = ConfigEditorLayout::Color,

            ConfigEditorLayout::Color => self.config_layout = ConfigEditorLayout::Key1,
            ConfigEditorLayout::Key1 => self.config_layout = ConfigEditorLayout::Key2,
            ConfigEditorLayout::Key2 => self.config_layout = ConfigEditorLayout::General,
        }

        let layout = self.config_layout.clone();
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::Header),
                Box::new(CEHeader::new(&layout, &self.config)),
                vec![]
            )
            .is_ok());
        match self.config_layout {
            ConfigEditorLayout::General => self
                .app
                .active(&Id::ConfigEditor(IdConfigEditor::MusicDir))
                .ok(),
            ConfigEditorLayout::Color => self
                .app
                .active(&Id::ConfigEditor(IdConfigEditor::CEThemeSelect))
                .ok(),
            ConfigEditorLayout::Key1 => self
                .app
                .active(&Id::ConfigEditor(IdConfigEditor::GlobalQuit))
                .ok(),
            ConfigEditorLayout::Key2 => None,
        };
    }

    /// Mount quit popup
    pub fn mount_config_save_popup(&mut self) {
        assert!(self
            .app
            .remount(
                Id::ConfigEditor(IdConfigEditor::ConfigSavePopup),
                Box::new(ConfigSavePopup::new(&self.config)),
                vec![]
            )
            .is_ok());
        assert!(self
            .app
            .active(&Id::ConfigEditor(IdConfigEditor::ConfigSavePopup))
            .is_ok());
    }

    pub fn collect_config_data(&mut self) {
        self.config.style_color_symbol = self.ce_style_color_symbol.clone();
        if let Ok(State::One(StateValue::String(music_dir))) =
            self.app.state(&Id::ConfigEditor(IdConfigEditor::MusicDir))
        {
            self.config.music_dir = music_dir;
        }

        if let Ok(State::One(StateValue::Usize(exit_confirmation))) = self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::ExitConfirmation))
        {
            self.config.enable_exit_confirmation = matches!(exit_confirmation, 0);
        }

        if let Ok(State::One(StateValue::Usize(display_symbol))) = self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::PlaylistDisplaySymbol))
        {
            self.config.playlist_display_symbol = matches!(display_symbol, 0);
        }

        if let Ok(State::One(StateValue::String(random_track_quantity_str))) = self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::PlaylistRandomTrack))
        {
            if let Ok(quantity) = random_track_quantity_str.parse::<u32>() {
                self.config.playlist_select_random_track_quantity = quantity;
            }
        }

        if let Ok(State::One(StateValue::String(random_album_quantity_str))) = self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::PlaylistRandomAlbum))
        {
            if let Ok(quantity) = random_album_quantity_str.parse::<u32>() {
                self.config.playlist_select_random_album_quantity = quantity;
            }
        }

        if let Ok(State::One(StateValue::String(album_photo_x_between_1_100_str))) = self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::AlbumPhotoX))
        {
            if let Ok(quantity) = album_photo_x_between_1_100_str.parse::<u32>() {
                self.config.album_photo_xywh.x_between_1_100 = quantity;
            }
        }
        if let Ok(State::One(StateValue::String(album_photo_y_between_1_100_str))) = self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::AlbumPhotoY))
        {
            if let Ok(quantity) = album_photo_y_between_1_100_str.parse::<u32>() {
                self.config.album_photo_xywh.y_between_1_100 = quantity;
            }
        }
        if let Ok(State::One(StateValue::String(album_photo_width_between_1_100_str))) = self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::AlbumPhotoWidth))
        {
            if let Ok(quantity) = album_photo_width_between_1_100_str.parse::<u32>() {
                self.config.album_photo_xywh.width_between_1_100 = quantity;
            }
        }
        if let Ok(State::One(StateValue::Usize(align))) = self
            .app
            .state(&Id::ConfigEditor(IdConfigEditor::AlbumPhotoAlign))
        {
            let align = match align {
                0 => XywhAlign::BottomRight,
                1 => XywhAlign::BottomLeft,
                2 => XywhAlign::TopRight,
                _ => XywhAlign::TopLeft,
            };
            self.config.album_photo_xywh.align = align;
        }
    }
}