use ratatui::{
    layout::{Constraint, Layout},
    Frame,
};

use crate::{
    app::App,
    widgets::heart_rate::{charts::render_combined_chart, charts::ChartType, tables::render_table},
};

// TODO Ascii Heart Beat Animation

pub const CHART_BPM_MAX_ELEMENTS: usize = 120;
pub const CHART_RR_MAX_ELEMENTS: usize = 120;
pub const CHART_BPM_VERT_MARGIN: f64 = 3.0;
pub const CHART_RR_VERT_MARGIN: f64 = 0.1;

pub fn heart_rate_display(app: &App, frame: &mut Frame) {
    let area = frame.area();

    let vertical = Layout::vertical([Constraint::Min(4), Constraint::Percentage(100)]);
    let horizontal_shared = Layout::horizontal([Constraint::Percentage(100)]);
    let horizontal_split =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]);
    let [status_area, bottom] = vertical.areas(area);
    let [bpm_history, rr_history] = horizontal_split.areas(bottom);
    let [shared_chart] = horizontal_shared.areas(bottom);

    render_table(frame, status_area, app);
    let bpm_chart = app.settings.tui.chart_bpm_enabled;
    let rr_chart = app.settings.tui.chart_rr_enabled;
    let combined = app.settings.tui.charts_combine;

    if combined && bpm_chart && rr_chart {
        render_combined_chart(frame, shared_chart, app, ChartType::Combined);
    } else if bpm_chart && rr_chart {
        render_combined_chart(frame, bpm_history, app, ChartType::Bpm);
        render_combined_chart(frame, rr_history, app, ChartType::Rr);
    } else if bpm_chart {
        render_combined_chart(frame, shared_chart, app, ChartType::Bpm);
    } else if rr_chart {
        render_combined_chart(frame, shared_chart, app, ChartType::Rr);
    }
}
