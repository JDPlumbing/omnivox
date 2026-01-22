use crate::core::physics::camera::eclipse::EclipseType;
use crate::core::{SimTime, world::WorldResolver, WorldId,  UvoxId, world::world_env_descriptor::WorldSpace, CameraPose, SimDuration};
use crate::core::physics::camera::eclipse::compute_eclipse_at_time;
use crate::core::FrameError;

#[derive(Debug, Clone)]
pub struct EclipseEvent {
    pub time: SimTime,
    pub state: EclipseType,
}
#[derive(Debug, Clone)]
pub struct EclipseTimeline {
    pub events: Vec<EclipseEvent>,
}
pub fn compute_eclipse_timeline(
    resolver: &WorldResolver,
    observer_world: WorldId,
    observer_uvox: &UvoxId,
    space: &WorldSpace,
    camera_pose: CameraPose,
    start: SimTime,
    end: SimTime,
    step: SimDuration,
) -> Result<EclipseTimeline, EclipseError> {
    if step.is_zero() {
        return Err(EclipseError::InvalidStep);
    }

    let mut events = Vec::new();

    // Sample initial state
    let first = compute_eclipse_at_time(
        resolver,
        observer_world,
        observer_uvox,
        space,
        camera_pose,
        start,
    )?;

    let mut last_state = first.eclipse;

    events.push(EclipseEvent {
        time: start,
        state: last_state,
    });

    let mut t = SimTime(start.0 + step.0);

    while t.0 <= end.0 {
        let eclipse = compute_eclipse_at_time(
            resolver,
            observer_world,
            observer_uvox,
            space,
            camera_pose,
            t,
        )?;

        if eclipse.eclipse != last_state {
            events.push(EclipseEvent {
                time: t,
                state: eclipse.eclipse,
            });
            last_state = eclipse.eclipse;
        }

        t = SimTime(t.0 + step.0);
    }

    Ok(EclipseTimeline { events })
}
#[derive(Debug)]
pub enum EclipseError {
    Frame(FrameError),
   
    InvalidStep,
    NoPhysicalRadius,
    GeometryError,
}
impl From<FrameError> for EclipseError {
    fn from(err: FrameError) -> Self {
        EclipseError::Frame(err)
    }
}
