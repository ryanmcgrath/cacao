use crate::uikit::scene::{Scene, SceneConnectionOptions, SceneSession};

pub trait WindowSceneDelegate {
    fn will_connect(&self, scene: Scene, session: SceneSession, options: SceneConnectionOptions);
}
