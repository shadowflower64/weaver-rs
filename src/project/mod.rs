use crate::{chart::chart::ChartMidi, project::metadata::ProjectMetadata};
pub(crate) mod metadata;

pub struct Project {
    project_metadata: ProjectMetadata,
    chart: ChartMidi,
}
